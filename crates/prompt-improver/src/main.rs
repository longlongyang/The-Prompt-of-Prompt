//! Prompt Improver CLI
//!
//! A tool that uses Google Gemini to rewrite and improve user prompts
//! based on official Prompt Engineering guidelines.
//!
//! Uses Batch API to generate 3 different versions for comparison (50% cheaper!).

mod i18n;

use gemini_client::{
    models, BatchJobRequest, GeminiClient, GenerateContentRequest, GenerationConfig, InlinedRequest,
};
use i18n::I18n;
use std::fs;
use std::io::{self, BufRead, Write};
use std::time::Duration;

/// Default number of prompt versions to generate
const DEFAULT_VERSION_COUNT: usize = 3;

/// Get the number of versions from environment variable or use default
fn get_version_count() -> usize {
    std::env::var("PROMPT_VERSIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_VERSION_COUNT)
        .max(1) // Minimum 1 version
        .min(10) // Maximum 10 versions
}

const SYSTEM_INSTRUCTION: &str = r#"You are an expert Prompt Engineer for Large Language Models, specifically Google Gemini.
Your goal is to rewrite user prompts to maximize performance, accuracy, and reliability.

Follow these principles when rewriting:
1.  **Clarity**: Separate instructions, context, and tasks using clear delimiters (like XML tags <context>, <task> or Markdown headers).
2.  **Few-Shot**: Whenever possible, generate 1-3 realistic examples (Input -> Output) to guide the model.
3.  **Constraints**: Explicitly state what the model should NOT do.
4.  **Persona**: Assign a specific role (e.g., "You are a senior Python architect").
5.  **Output Format**: Define the exact structure of the response (JSON, Markdown, etc.).
6.  **Thinking**: If the task is complex, instruct the model to "Think step-by-step" or "Plan before executing".

**Output Structure:**
Return your response in this format:

# Improved Prompt

```
[The rewritten prompt goes here]
```

# Explanation

[Bulleted list of improvements made]
"#;

const MERGE_SYSTEM_INSTRUCTION: &str = r#"You are a Senior Prompt Engineer and LLM Optimization Specialist. You possess deep expertise in Large Language Model architecture, instruction tuning, and prompt engineering best practices. Your goal is to analyze multiple prompt variations and synthesize them into a single, high-performance "Super Prompt."

<context>
You have been given three different improved prompts for the same original objective. Each version has unique strengths—one might have a superior persona, another better constraints, and a third clearer formatting instructions.
</context>

<task>
Your task is to synthesize these three inputs into one optimized Master Prompt. You must evaluate the strengths of each version, extract the highest-leverage components, and merge them into a cohesive, logically flowing prompt that is superior to any of the individual inputs.
</task>

<process_instructions>
1.  **Analyze**: Read all three prompts to determine the core intent and shared objective.
2.  **Deconstruct & Evaluate**: Identify the strongest elements across the versions:
    *   **Persona/Context**: Which version best defines the role and background?
    *   **Instructions**: Which version has the most precise, actionable steps?
    *   **Constraints**: Which version best defines what *not* to do (negative constraints)?
    *   **Formatting**: Which version provides the clearest output requirements?
    *   **Examples**: Are there high-quality few-shot examples to preserve?
3.  **Synthesize**: Merge these best elements into a single "Master Prompt." Ensure logical flow and remove redundancy.
4.  **Enhance**: Apply prompt engineering best practices to the final result:
    *   **Structure**: Use clear delimiters (XML tags or Markdown headers) to separate sections.
    *   **Clarity**: Ensure instructions are imperative, concise, and unambiguous.
    *   **Reasoning**: Add a "Chain of Thought" instruction (e.g., "Think step-by-step") if the task requires complex reasoning.
</process_instructions>

<output_requirements>
Please provide your response in the following format:

## Analysis of Inputs
*   **Strengths of Version 1**: [Brief notes]
*   **Strengths of Version 2**: [Brief notes]
*   **Strengths of Version 3**: [Brief notes]
*   **Rationale for Selection**: Briefly explain which specific elements you kept from each and why.

## The Optimized Master Prompt

```
[The final merged and optimized prompt goes here]
```
</output_requirements>
"#;

fn print_header(i18n: &I18n) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!(
        "║              {}                       ║",
        i18n.header_title()
    );
    println!("║         {}        ║", i18n.header_subtitle());
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

fn read_multiline_input(i18n: &I18n) -> io::Result<String> {
    println!("{}", i18n.enter_prompt());
    println!("───────────────────────────────────────────────────────────────");
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut empty_line_count = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            empty_line_count += 1;
            if empty_line_count >= 1 {
                // Only need ONE empty line to submit
                break;
            }
        } else {
            empty_line_count = 0;
            lines.push(line);
        }
    }

    Ok(lines.join("\n"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize i18n
    let i18n = I18n::new();

    print_header(&i18n);

    // Initialize Gemini client from environment
    let client = GeminiClient::from_env().map_err(|e| {
        eprintln!("{}", i18n.error_init_client());
        eprintln!("{}", i18n.error_set_api_key());
        e
    })?;

    // Read user's draft prompt
    let user_draft = read_multiline_input(&i18n)?;

    if user_draft.trim().is_empty() {
        eprintln!("{}", i18n.error_no_prompt());
        std::process::exit(1);
    }

    println!();
    
    // Get configurable version count
    let version_count = get_version_count();
    println!("{}", i18n.improving_prompt_with_count(version_count));
    println!("{}", i18n.batch_api_note());

    // Check if proxy is configured
    if let Ok(proxy) = std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("HTTP_PROXY")) {
        println!("{}", i18n.using_proxy(&proxy));
    }
    println!();
    io::stdout().flush().ok();

    // Build the base request
    let base_request = GenerateContentRequest::with_text(&user_draft)
        .model(models::gemini_3::PRO_PREVIEW)
        .system_instruction(SYSTEM_INSTRUCTION)
        .generation_config(GenerationConfig {
            temperature: Some(0.8), // Slightly higher for variety
            ..Default::default()
        });

    // Create batch requests with unique keys (configurable count)
    let batch_requests: Vec<_> = (1..=version_count)
        .map(|i| InlinedRequest::with_key(base_request.clone(), &format!("version_{}", i)))
        .collect();

    println!("{}", i18n.creating_batch_with_count(version_count));
    io::stdout().flush().ok();

    // Create batch job
    let batch_job = client
        .create_batch_job(
            BatchJobRequest::inline(models::gemini_3::PRO_PREVIEW, batch_requests)
                .display_name("prompt-improver-batch"),
        )
        .await
        .map_err(|e| {
            eprintln!("{}: {:?}", i18n.error_batch_create(), e);
            e
        })?;

    println!("{}", i18n.batch_created(&batch_job.name));
    println!("{}", i18n.waiting_batch());
    io::stdout().flush().ok();

    // Wait for completion
    let completed_job = client
        .wait_for_batch_job(&batch_job.name, Duration::from_secs(5))
        .await
        .map_err(|e| {
            eprintln!("{}: {:?}", i18n.error_batch_failed(), e);
            e
        })?;

    println!("{}", i18n.batch_completed());
    println!();

    // Process responses
    let responses = completed_job.responses().ok_or_else(|| {
        eprintln!("{}", i18n.error_no_responses());
        "No responses in batch job"
    })?;

    let mut total_input_tokens: u32 = 0;
    let mut total_output_tokens: u32 = 0;
    let mut total_thinking_tokens: u32 = 0;

    // Store the generated prompts for merging later
    let mut generated_prompts: Vec<String> = Vec::new();

    for (i, response) in responses.iter().enumerate() {
        let version_num = i + 1;
        let default_key = format!("version_{}", version_num);
        let key = response.key.as_deref().unwrap_or(&default_key);

        if let Some(error) = &response.error {
            eprintln!("❌ Error in {}: {:?}", key, error.message);
            continue;
        }

        if let Some(gen_response) = &response.response {
            if let Some(text) = gen_response.text() {
                // Save each version to a separate file
                let output_file = format!("improved_prompt_{}.md", version_num);
                fs::write(&output_file, text)?;
                println!("{}", i18n.version_saved(version_num, &output_file));

                // Store for merging later
                generated_prompts.push(text.to_string());

                // Accumulate token usage
                if let Some(usage) = &gen_response.usage_metadata {
                    total_input_tokens += usage.prompt_token_count;
                    total_output_tokens += usage.candidates_token_count;
                    if let Some(thinking) = usage.thoughts_token_count {
                        total_thinking_tokens += thinking;
                    }
                }
            }
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("{}", i18n.generated_versions_with_count(version_count));
    for i in 1..=version_count {
        println!("   • improved_prompt_{}.md", i);
    }
    println!("═══════════════════════════════════════════════════════════════");

    // Merge the prompts into a master prompt if we have at least 2
    if generated_prompts.len() >= 2 {
        println!();
        println!("{}", i18n.merging_prompts_with_count(generated_prompts.len()));
        io::stdout().flush().ok();

        // Build the merge request with all prompts
        let mut merge_input = String::from("Please synthesize these improved prompts into one optimized Master Prompt:\n\n");
        for (i, prompt) in generated_prompts.iter().enumerate() {
            merge_input.push_str(&format!("## Version {}\n{}\n\n", i + 1, prompt));
        }

        let merge_request = GenerateContentRequest::with_text(&merge_input)
            .model(models::gemini_3::PRO_PREVIEW)
            .system_instruction(MERGE_SYSTEM_INSTRUCTION)
            .generation_config(GenerationConfig {
                temperature: Some(0.7),
                ..Default::default()
            });

        match client.generate_content(merge_request).await {
            Ok(merge_response) => {
                if let Some(text) = merge_response.text() {
                    // Save the master prompt
                    fs::write("master_prompt.md", text)?;
                    println!("{}", i18n.master_saved());

                    // Add merge token usage
                    if let Some(usage) = &merge_response.usage_metadata {
                        total_input_tokens += usage.prompt_token_count;
                        total_output_tokens += usage.candidates_token_count;
                        if let Some(thinking) = usage.thoughts_token_count {
                            total_thinking_tokens += thinking;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {:?}", i18n.merge_failed(), e);
                eprintln!("{}", i18n.manual_merge_hint());
            }
        }

        println!();
        println!("═══════════════════════════════════════════════════════════════");
        println!("{}", i18n.final_output());
        for i in 1..=version_count {
            println!("   • improved_prompt_{}.md  ({})", i, i18n.version_label(i));
        }
        println!("   • master_prompt.md      ({})", i18n.merged_optimized());
        println!("═══════════════════════════════════════════════════════════════");
    }

    // Show aggregated token usage
    if total_input_tokens > 0 {
        println!();
        println!("{}", i18n.token_usage_header());
        println!("{:12} {} tokens", i18n.input_tokens(), total_input_tokens);
        println!("{:12} {} tokens", i18n.output_tokens(), total_output_tokens);
        if total_thinking_tokens > 0 {
            println!(
                "{:12} {} tokens",
                i18n.thinking_tokens(),
                total_thinking_tokens
            );
        }
        println!("   ─────────────────");
        println!(
            "{:12} {} tokens",
            i18n.total_tokens(),
            total_input_tokens + total_output_tokens + total_thinking_tokens
        );

        // Estimate cost (Batch API: $1/1M input, $6/1M output; Real-time: $2/1M input, $12/1M output)
        // Note: This is an approximate estimate using mixed pricing
        let input_cost = (total_input_tokens as f64 / 1_000_000.0) * 1.5; // Blended rate
        let output_cost = (total_output_tokens as f64 / 1_000_000.0) * 9.0; // Blended rate
        let thinking_cost = (total_thinking_tokens as f64 / 1_000_000.0) * 9.0;
        let total_cost = input_cost + output_cost + thinking_cost;

        println!();
        println!("{}", i18n.cost_header());
        println!("{:12} ${:.6}", i18n.input_tokens(), input_cost);
        println!(
            "{:12} ${:.6}",
            i18n.output_tokens(),
            output_cost + thinking_cost
        );
        println!("   ─────────────────");
        println!("{:12} ${:.6}", i18n.total_tokens(), total_cost);
    }

    Ok(())
}

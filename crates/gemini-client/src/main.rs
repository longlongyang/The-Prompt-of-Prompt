//! Gemini CLI - Example usage of the Gemini API client.

use gemini_client::{
    models, Content, CountTokensRequest, EmbedContentRequest, GeminiClient, GenerateContentRequest,
    GenerationConfig, Result, ThinkingLevel,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    // Create client from environment
    let client = GeminiClient::from_env()?;

    match command {
        "generate" | "gen" => {
            let prompt = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Explain how AI works in a few words".to_string());
            generate_content(&client, &prompt).await?;
        }
        "reason" => {
            // Use the advanced reasoning model
            let prompt = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Solve this step by step: What is 15% of 240?".to_string());
            generate_with_model(&client, &prompt, models::gemini_2_5_pro::STABLE).await?;
        }
        "fast" => {
            // Use the fastest model
            let prompt = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Quick answer: What is the capital of France?".to_string());
            generate_with_model(&client, &prompt, models::gemini_2_5_flash_lite::STABLE).await?;
        }
        "best" => {
            // Use Gemini 3 Pro with high thinking
            let prompt = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Analyze this complex topic in depth.".to_string());
            generate_gemini3(&client, &prompt, ThinkingLevel::High).await?;
        }
        "think-low" => {
            // Use Gemini 3 Pro with low thinking (faster)
            let prompt = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Quick question for Gemini 3.".to_string());
            generate_gemini3(&client, &prompt, ThinkingLevel::Low).await?;
        }
        "search" => {
            // Use Gemini 3 with Google Search grounding
            let prompt = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "What are the latest news about AI?".to_string());
            generate_with_search(&client, &prompt).await?;
        }
        "chat" => {
            interactive_chat(&client).await?;
        }
        "models" => {
            list_models(&client).await?;
        }
        "available" => {
            print_available_models();
        }
        "model" => {
            let model = args
                .get(2)
                .map(|s| s.as_str())
                .unwrap_or(models::defaults::GENERAL);
            get_model(&client, model).await?;
        }
        "count" => {
            let text = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Hello, world!".to_string());
            count_tokens(&client, &text).await?;
        }
        "embed" => {
            let text = args
                .get(2..)
                .map(|s| s.join(" "))
                .unwrap_or_else(|| "Hello, world!".to_string());
            embed_content(&client, &text).await?;
        }
        "files" => {
            list_files(&client).await?;
        }
        "help" | _ => {
            print_help();
        }
    }

    Ok(())
}

async fn generate_content(client: &GeminiClient, prompt: &str) -> Result<()> {
    println!("Generating content for: {}\n", prompt);

    let request = GenerateContentRequest::with_text(prompt)
        .generation_config(GenerationConfig::new().temperature(0.7));

    let response = client.generate_content(request).await?;

    print_response(&response);
    Ok(())
}

async fn generate_with_model(client: &GeminiClient, prompt: &str, model: &str) -> Result<()> {
    println!("Using model: {}", model);
    println!("Generating content for: {}\n", prompt);

    let request = GenerateContentRequest::with_text(prompt)
        .model(model)
        .generation_config(GenerationConfig::new().temperature(0.7));

    let response = client.generate_content(request).await?;

    print_response(&response);
    Ok(())
}

async fn generate_gemini3(
    client: &GeminiClient,
    prompt: &str,
    thinking_level: ThinkingLevel,
) -> Result<()> {
    println!(
        "Using Gemini 3 Pro with thinking level: {:?}",
        thinking_level
    );
    println!("Generating content for: {}\n", prompt);

    let request = GenerateContentRequest::with_text(prompt)
        .model(models::gemini_3::PRO_PREVIEW)
        .thinking_level(thinking_level);

    let response = client.generate_content(request).await?;

    print_response(&response);
    Ok(())
}

async fn generate_with_search(client: &GeminiClient, prompt: &str) -> Result<()> {
    println!("Using Gemini 3 Pro with Google Search grounding");
    println!("Generating content for: {}\n", prompt);

    let request = GenerateContentRequest::with_text(prompt)
        .model(models::gemini_3::PRO_PREVIEW)
        .with_google_search();

    let response = client.generate_content(request).await?;

    print_response(&response);

    // Show grounding metadata if available
    for candidate in &response.candidates {
        if let Some(grounding) = &candidate.grounding_metadata {
            println!("\n--- Grounding Information ---");
            if let Some(sources) = &grounding.grounding_chunks {
                println!("Sources:");
                for chunk in sources {
                    if let Some(web) = &chunk.web {
                        println!(
                            "  • {} ({})",
                            web.title.as_deref().unwrap_or("Untitled"),
                            web.uri
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_response(response: &gemini_client::GenerateContentResponse) {
    if let Some(text) = response.text() {
        println!("Response:\n{}", text);
    }

    print_usage_stats(
        response.usage_metadata.as_ref(),
        response.model_version.as_deref(),
    );
}

fn print_usage_stats(usage: Option<&gemini_client::UsageMetadata>, model: Option<&str>) {
    println!("\n╭─────────────────────────────────────────────────────╮");
    println!("│                    💰 USAGE STATS                   │");
    println!("├─────────────────────────────────────────────────────┤");

    if let Some(usage) = usage {
        let input_tokens = usage.prompt_token_count;
        let output_tokens = usage.candidates_token_count;
        let total_tokens = usage.total_token_count;
        let thinking_tokens = usage.thoughts_token_count.unwrap_or(0);

        println!(
            "│ 📥 Input tokens:    {:>10}                    │",
            input_tokens
        );
        println!(
            "│ 📤 Output tokens:   {:>10}                    │",
            output_tokens
        );
        if thinking_tokens > 0 {
            println!(
                "│ 🧠 Thinking tokens: {:>10}                    │",
                thinking_tokens
            );
        }
        println!(
            "│ 📊 Total tokens:    {:>10}                    │",
            total_tokens
        );
        println!("├─────────────────────────────────────────────────────┤");

        // Estimate cost based on model
        let model_name = model.unwrap_or("unknown");
        let (input_cost, output_cost) =
            estimate_cost(model_name, input_tokens, output_tokens + thinking_tokens);
        let total_cost = input_cost + output_cost;

        println!("│ 💵 Estimated Cost (Paid Tier):                      │");
        println!(
            "│    Input:  ${:<10.6}                            │",
            input_cost
        );
        println!(
            "│    Output: ${:<10.6}                            │",
            output_cost
        );
        println!(
            "│    Total:  ${:<10.6}                            │",
            total_cost
        );
    } else {
        println!("│ ⚠️  No usage data available                         │");
    }

    if let Some(model) = model {
        println!("├─────────────────────────────────────────────────────┤");
        println!("│ 🤖 Model: {:<40} │", truncate_str(model, 40));
    }

    println!("╰─────────────────────────────────────────────────────╯");
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:<width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Estimate cost in USD based on model and token counts.
/// Prices are per 1M tokens from the pricing documentation.
fn estimate_cost(model: &str, input_tokens: u32, output_tokens: u32) -> (f64, f64) {
    let input = input_tokens as f64;
    let output = output_tokens as f64;

    // Pricing per 1M tokens (standard tier, prompts <= 200k)
    let (input_price, output_price) = if model.contains("gemini-3-pro") {
        (2.00, 12.00) // Gemini 3 Pro
    } else if model.contains("gemini-2.5-pro") {
        (1.25, 10.00) // Gemini 2.5 Pro
    } else if model.contains("gemini-2.5-flash-lite") {
        (0.10, 0.40) // Gemini 2.5 Flash-Lite
    } else if model.contains("gemini-2.5-flash") {
        (0.30, 2.50) // Gemini 2.5 Flash
    } else if model.contains("gemini-2.0-flash-lite") {
        (0.075, 0.30) // Gemini 2.0 Flash-Lite
    } else if model.contains("gemini-2.0-flash") {
        (0.10, 0.40) // Gemini 2.0 Flash
    } else if model.contains("embedding") {
        (0.15, 0.0) // Embeddings (input only)
    } else {
        (0.30, 2.50) // Default to Flash pricing
    };

    let input_cost = (input / 1_000_000.0) * input_price;
    let output_cost = (output / 1_000_000.0) * output_price;

    (input_cost, output_cost)
}

fn print_available_models() {
    println!("Available Gemini Models:\n");

    println!("=== Gemini 3 (Most Intelligent) ===");
    println!(
        "  {} - Best for multimodal understanding & agentic tasks",
        models::gemini_3::PRO_PREVIEW
    );
    println!(
        "  {} - Native image generation",
        models::gemini_3::PRO_IMAGE_PREVIEW
    );

    println!("\n=== Gemini 2.5 Flash (Fast & Intelligent) ===");
    println!(
        "  {} - Stable, best price-performance",
        models::gemini_2_5_flash::STABLE
    );
    println!("  {} - Preview version", models::gemini_2_5_flash::PREVIEW);
    println!(
        "  {} - Native image generation",
        models::gemini_2_5_flash::IMAGE
    );
    println!(
        "  {} - Native audio (Live API)",
        models::gemini_2_5_flash::LIVE
    );
    println!("  {} - Text-to-speech", models::gemini_2_5_flash::TTS);

    println!("\n=== Gemini 2.5 Flash-Lite (Ultra Fast) ===");
    println!(
        "  {} - Stable, cost-efficient",
        models::gemini_2_5_flash_lite::STABLE
    );
    println!(
        "  {} - Preview version",
        models::gemini_2_5_flash_lite::PREVIEW
    );

    println!("\n=== Gemini 2.5 Pro (Advanced Thinking) ===");
    println!(
        "  {} - State-of-the-art reasoning",
        models::gemini_2_5_pro::STABLE
    );
    println!("  {} - Text-to-speech", models::gemini_2_5_pro::TTS);

    println!("\n=== Specialized Models ===");
    println!(
        "  {} - Browser automation",
        models::gemini_computer_use::PREVIEW
    );
    println!(
        "  {} - Embodied reasoning for robotics",
        models::gemini_robotics::ER_1_5_PREVIEW
    );

    println!("\n=== Gemini 2.0 Flash (Previous Gen) ===");
    println!("  {} - Latest", models::gemini_2_0_flash::LATEST);
    println!("  {} - Stable", models::gemini_2_0_flash::STABLE);
    println!("  {} - Image generation", models::gemini_2_0_flash::IMAGE);

    println!("\n=== Imagen (Dedicated Image Generation) ===");
    println!("  {} - Best quality", models::imagen_4::STANDARD);
    println!("  {} - Highest quality", models::imagen_4::ULTRA);
    println!("  {} - Speed optimized", models::imagen_4::FAST);
    println!("  {} - Previous generation", models::imagen_3::STABLE);

    println!("\n=== Veo (Video Generation) ===");
    println!(
        "  {} - Best quality, 720p/1080p with audio",
        models::veo_3_1::PREVIEW
    );
    println!("  {} - Fast with audio", models::veo_3_1::FAST);
    println!("  {} - Stable with audio", models::veo_3::STABLE);
    println!("  {} - Fast stable", models::veo_3::FAST);
    println!("  {} - Silent video", models::veo_2::STABLE);

    println!("\n=== Embedding Models ===");
    println!(
        "  {} - Latest (3072 dims, MRL)",
        models::embeddings::GEMINI_EMBEDDING_001
    );

    println!("\n=== Gemma (Open Models) ===");
    println!("  {} - Lightweight open model", models::gemma::GEMMA_3);
    println!(
        "  {} - Efficient for everyday devices",
        models::gemma::GEMMA_3N
    );

    println!("\n=== Defaults ===");
    println!("  General: {}", models::defaults::GENERAL);
    println!("  Reasoning: {}", models::defaults::REASONING);
    println!("  Fast: {}", models::defaults::FAST);
    println!("  Best: {}", models::defaults::BEST);
    println!("  Embedding: {}", models::defaults::EMBEDDING);
    println!("  Imagen: {}", models::defaults::IMAGEN);
    println!("  Video Gen: {}", models::defaults::VIDEO_GEN);
}

async fn interactive_chat(client: &GeminiClient) -> Result<()> {
    use std::io::{self, BufRead, Write};

    println!("Starting interactive chat (type 'quit' to exit)\n");

    let mut history: Vec<Content> = Vec::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("You: ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Add user message to history
        history.push(Content::user(input));

        // Build request with full history
        let request = GenerateContentRequest::new(history.clone());

        match client.generate_content(request).await {
            Ok(response) => {
                if let Some(text) = response.text() {
                    println!("\nGemini: {}\n", text);
                    // Add assistant response to history
                    history.push(Content::model(text));
                } else {
                    println!("\n(No response)\n");
                }
                // Show usage stats for this turn
                print_usage_stats(
                    response.usage_metadata.as_ref(),
                    response.model_version.as_deref(),
                );
            }
            Err(e) => {
                eprintln!("\nError: {}\n", e);
                // Remove the failed user message
                history.pop();
            }
        }
    }

    Ok(())
}

async fn list_models(client: &GeminiClient) -> Result<()> {
    println!("Available models:\n");

    let models = client.list_models().await?;

    for model in models {
        println!("• {} ({})", model.display_name, model.id());
        println!("  {}", model.description);
        println!(
            "  Input: {} tokens, Output: {} tokens",
            model.input_token_limit, model.output_token_limit
        );
        println!(
            "  Methods: {}",
            model.supported_generation_methods.join(", ")
        );
        println!();
    }

    Ok(())
}

async fn get_model(client: &GeminiClient, model: &str) -> Result<()> {
    println!("Getting model info for: {}\n", model);

    let model = client.get_model(model).await?;

    println!("Name: {}", model.name);
    println!("Display Name: {}", model.display_name);
    println!("Description: {}", model.description);
    println!("Version: {}", model.version);
    println!("Input Token Limit: {}", model.input_token_limit);
    println!("Output Token Limit: {}", model.output_token_limit);
    println!(
        "Supported Methods: {}",
        model.supported_generation_methods.join(", ")
    );
    if let Some(temp) = model.temperature {
        println!("Default Temperature: {}", temp);
    }
    if let Some(max_temp) = model.max_temperature {
        println!("Max Temperature: {}", max_temp);
    }

    Ok(())
}

async fn count_tokens(client: &GeminiClient, text: &str) -> Result<()> {
    println!("Counting tokens for: {}\n", text);

    let request = CountTokensRequest::with_text(text);
    let response = client.count_tokens(request).await?;

    println!("Total tokens: {}", response.total_tokens);

    // Show usage stats
    println!("\n╭─────────────────────────────────────────────────────╮");
    println!("│                    💰 USAGE STATS                   │");
    println!("├─────────────────────────────────────────────────────┤");
    println!(
        "│ 📊 Tokens counted:  {:>10}                    │",
        response.total_tokens
    );
    println!("│ 💵 Cost: FREE (count_tokens is free)               │");
    println!("╰─────────────────────────────────────────────────────╯");

    Ok(())
}

async fn embed_content(client: &GeminiClient, text: &str) -> Result<()> {
    println!("Generating embedding for: {}\n", text);
    println!("Using model: {}", models::defaults::EMBEDDING);

    let request = EmbedContentRequest::new(text)
        .model(models::defaults::EMBEDDING)
        .for_similarity(); // Optimized for semantic similarity

    let response = client.embed_content(request).await?;

    let values = &response.embedding.values;
    println!("Embedding dimension: {}", values.len());
    println!("First 10 values: {:?}", &values[..10.min(values.len())]);

    // Estimate token count (rough approximation: ~4 chars per token)
    let estimated_tokens = (text.len() / 4).max(1) as u32;
    let cost = (estimated_tokens as f64 / 1_000_000.0) * 0.15;

    println!("\n╭─────────────────────────────────────────────────────╮");
    println!("│                    💰 USAGE STATS                   │");
    println!("├─────────────────────────────────────────────────────┤");
    println!(
        "│ 📥 Est. input tokens: {:>8}                      │",
        estimated_tokens
    );
    println!(
        "│ 📏 Output dimensions: {:>8}                      │",
        values.len()
    );
    println!("│ 💵 Est. cost: ${:<10.6}                          │", cost);
    println!("├─────────────────────────────────────────────────────┤");
    println!("│ 🤖 Model: gemini-embedding-001                      │");
    println!("│ 💡 Price: $0.15 / 1M tokens                         │");
    println!("╰─────────────────────────────────────────────────────╯");

    Ok(())
}

async fn list_files(client: &GeminiClient) -> Result<()> {
    println!("Uploaded files:\n");

    let response = client.list_files(None, None).await?;

    if response.files.is_empty() {
        println!("No files found.");
    } else {
        for file in response.files {
            println!("• {}", file.name);
            if let Some(display_name) = file.display_name {
                println!("  Display Name: {}", display_name);
            }
            println!("  MIME Type: {}", file.mime_type);
            if let Some(size) = file.size_bytes {
                println!("  Size: {} bytes", size);
            }
            if let Some(state) = file.state {
                println!("  State: {}", state);
            }
            println!();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Gemini CLI - A command-line interface for the Gemini API\n");
    println!("USAGE:");
    println!("  gemini-cli <COMMAND> [ARGS]\n");
    println!("GENERATION COMMANDS:");
    println!(
        "  generate <prompt>  Generate content using default model ({})",
        models::defaults::GENERAL
    );
    println!("  gen <prompt>       Alias for generate");
    println!(
        "  reason <prompt>    Use reasoning model ({}) for complex tasks",
        models::defaults::REASONING
    );
    println!(
        "  fast <prompt>      Use fastest model ({}) for quick responses",
        models::defaults::FAST
    );
    println!("  best <prompt>      Use Gemini 3 Pro with high thinking for best results");
    println!("  chat               Start an interactive chat session\n");
    println!("GEMINI 3 COMMANDS:");
    println!("  think-low <prompt>   Use Gemini 3 Pro with low thinking (faster)");
    println!("  search <prompt>      Use Gemini 3 Pro with Google Search grounding\n");
    println!("MODEL COMMANDS:");
    println!("  available          List all available model constants");
    println!("  models             List models from API");
    println!("  model <name>       Get information about a specific model\n");
    println!("EMBEDDING & TOOLS:");
    println!(
        "  embed <text>       Generate embedding using {} (3072 dims)",
        models::defaults::EMBEDDING
    );
    println!("  count <text>       Count tokens in text");
    println!("  files              List uploaded files\n");
    println!("ENVIRONMENT:");
    println!("  GEMINI_API_KEY     Required. Your Gemini API key");
    println!("  GEMINI_MODEL       Optional. Override default model\n");
    println!("EXAMPLES:");
    println!("  gemini-cli generate \"What is the meaning of life?\"");
    println!("  gemini-cli reason \"Solve: If a train travels at 60mph for 2.5 hours...\"");
    println!("  gemini-cli fast \"What is 2+2?\"");
    println!("  gemini-cli best \"Analyze the implications of quantum computing\"");
    println!("  gemini-cli search \"What are the latest AI breakthroughs?\"");
    println!("  gemini-cli embed \"semantic search query\"");
    println!("  gemini-cli chat");
    println!("  gemini-cli available\n");
    println!("EMBEDDING TASK TYPES (SDK):");
    println!("  SEMANTIC_SIMILARITY  - Compare text similarity");
    println!("  RETRIEVAL_DOCUMENT   - Index documents for search");
    println!("  RETRIEVAL_QUERY      - Search queries");
    println!("  CLASSIFICATION       - Text classification");
    println!("  CLUSTERING           - Group similar texts");
    println!("  CODE_RETRIEVAL_QUERY - Search code blocks");
    println!("  QUESTION_ANSWERING   - Q&A systems");
    println!("  FACT_VERIFICATION    - Verify statements");
}

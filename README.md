<p align="center">
  <img src="https://img.shields.io/badge/Rust-🦀-orange?style=for-the-badge" alt="Rust"/>
  <img src="https://img.shields.io/badge/Gemini-AI-blue?style=for-the-badge" alt="Gemini AI"/>
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="MIT License"/>
</p>

<h1 align="center">🚀 The Prompt of Prompt</h1>

<p align="center">
  <em>A meta-prompt tool that uses AI to generate better prompts</em>
</p>

<p align="center">
  <strong>Transform your amateur prompts into professional, high-performance ones using Google Gemini AI</strong>
</p>

<p align="center">
  <a href="./README_CN.md">🇨🇳 中文文档</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#how-it-works">How It Works</a>
</p>

---

## ⭐ Why Star This Project?

> **Writing good prompts is hard.** Even experienced developers struggle to craft prompts that consistently get the best results from LLMs.

This tool solves that problem by using **Google's most advanced AI (Gemini 3 Pro)** to automatically rewrite your prompts following professional prompt engineering best practices.

**If you find this useful, please give us a ⭐ Star!** It helps more developers discover this tool.

---

## 🎯 The Problem

You write a prompt like:
```
help me write a python script to process csv files
```

And wonder why the AI gives you generic, incomplete, or inconsistent results.

## ✨ The Solution

**The Prompt of Prompt** transforms that into a professional prompt:

```markdown
<persona>
You are a Senior Python Developer with 10+ years of experience in data processing,
specializing in pandas, csv handling, and automation scripts.
</persona>

<context>
I need a Python script to process CSV files for data analysis purposes.
</context>

<task>
Create a Python script that:
1. Reads CSV files from a specified directory
2. Handles common issues (encoding, missing values, malformed rows)
3. Provides summary statistics
4. Exports cleaned data
</task>

<constraints>
- Use only standard library + pandas
- Include proper error handling
- Add logging for debugging
- Follow PEP 8 style guidelines
</constraints>

<output_format>
Provide the complete Python script with:
- Clear comments explaining each section
- Example usage in the docstring
- Type hints for all functions
</output_format>
```

**Result:** More accurate, complete, and consistent AI responses.

---

## 🔥 Features

| Feature | Description |
|---------|-------------|
| **🎯 Multiple Variations** | Generates N improved versions using Batch API (default: 3, configurable) |
| **🏆 Master Prompt** | Automatically merges the best elements into one optimized prompt |
| **💰 50% Cheaper** | Uses Gemini Batch API for cost savings |
| **🧠 Gemini 3 Pro** | Powered by Google's most intelligent AI model |
| **⚡ Fast** | Batch processing completes in seconds |
| **🌍 Bilingual** | Supports English and Chinese interface |

---

## 📦 Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- Google Gemini API Key ([Get one free](https://aistudio.google.com/apikey))

### Build from Source

```bash
# Clone the repository
git clone https://github.com/anthropics/the-prompt-of-prompt.git
cd the-prompt-of-prompt

# Build the project
cargo build --release

# The binary will be at ./target/release/prompt-improver
```

### Set API Key

```bash
export GEMINI_API_KEY="your-api-key-here"
```

---

## 🚀 Quick Start

```bash
# Run the prompt improver
cargo run --package prompt-improver

# Or use the built binary
./target/release/prompt-improver
```

### Example Session

```
╔══════════════════════════════════════════════════════════════╗
║              🚀 Gemini Prompt Improver                       ║
║         Transform your prompts into professional ones        ║
╚══════════════════════════════════════════════════════════════╝

📝 Enter your draft prompt (press Enter twice to submit):
───────────────────────────────────────────────────────────────
write a rust function to parse json

⏳ Improving your prompt with Gemini 3 Pro (Batch API - 3 versions)...
   💡 Using Batch API: 50% cheaper than real-time requests!

📤 Creating batch job with 3 requests...
✅ Batch job created: batchJobs/abc123
⏳ Waiting for batch job to complete...
📥 Batch job completed!

💾 Version 1 saved to: improved_prompt_1.md
💾 Version 2 saved to: improved_prompt_2.md
💾 Version 3 saved to: improved_prompt_3.md

🔀 Merging 3 versions into a Master Prompt...
✅ Master Prompt saved to: master_prompt.md

═══════════════════════════════════════════════════════════════
🏆 Final Output:
   • improved_prompt_1.md  (Version 1)
   • improved_prompt_2.md  (Version 2)
   • improved_prompt_3.md  (Version 3)
   • master_prompt.md      (✨ Merged & Optimized)
═══════════════════════════════════════════════════════════════

📊 Total Token Usage (3 batch + 1 merge request):
   Input:    1,234 tokens
   Output:   5,678 tokens
   Total:    6,912 tokens

💰 Estimated Cost (Batch + Real-time API):
   Total:    $0.012345
```

---

## 🛠️ How It Works

```
┌─────────────────┐
│  Your Draft     │
│  Prompt         │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────────────────┐
│              Gemini 3 Pro (Batch API)               │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐             │
│  │Version 1│  │Version 2│  │Version 3│             │
│  └─────────┘  └─────────┘  └─────────┘             │
└─────────────────────────────────────────────────────┘
         │           │           │
         └─────────┬─────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│              Gemini 3 Pro (Merge)                   │
│         Synthesize best elements from all 3         │
└─────────────────────────────────────────────────────┘
                   │
                   ▼
         ┌─────────────────┐
         │  Master Prompt  │
         │  (Optimized!)   │
         └─────────────────┘
```

### Prompt Engineering Principles Applied

1. **📋 Clear Structure** - Uses XML tags or Markdown to separate sections
2. **🎭 Persona Definition** - Assigns a specific expert role
3. **📝 Few-Shot Examples** - Adds input/output examples when helpful
4. **🚫 Negative Constraints** - Explicitly states what NOT to do
5. **📄 Output Format** - Defines exact response structure
6. **🧠 Chain of Thought** - Adds step-by-step reasoning instructions

---

## 🌍 Language Support

The CLI supports both English and Chinese interfaces:

```bash
# English (default)
export LANG=en_US.UTF-8
cargo run --package prompt-improver

# Chinese
export LANG=zh_CN.UTF-8
cargo run --package prompt-improver
```

---

## 📁 Project Structure

```
the-prompt-of-prompt/
├── crates/
│   ├── gemini-client/      # 🔧 Gemini API SDK (full-featured)
│   │   ├── src/
│   │   │   ├── client.rs   # API client implementation
│   │   │   ├── types.rs    # Request/Response types
│   │   │   └── models.rs   # Model constants
│   │   └── Cargo.toml
│   │
│   └── prompt-improver/    # 🚀 CLI tool (this tool!)
│       ├── src/
│       │   └── main.rs     # Main application
│       └── Cargo.toml
│
├── docs/                   # 📚 Documentation
│   ├── core capabilities/  # Gemini API features
│   └── prompt engineering/ # Prompt engineering guides
│
├── README.md               # 📖 This file
├── README_CN.md            # 🇨🇳 Chinese documentation
└── Cargo.toml              # Workspace configuration
```

---

## 🔧 Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `GEMINI_API_KEY` | Your Google Gemini API key | ✅ Yes |
| `PROMPT_VERSION_COUNT` | Number of prompt variations to generate (default: 3) | No |
| `HTTPS_PROXY` | Proxy server (if needed) | No |
| `LANG` | Language (`en_US.UTF-8` or `zh_CN.UTF-8`) | No |

---

## 📊 Gemini Client SDK

This project includes a full-featured Rust SDK for the Gemini API:

```rust
use gemini_client::{GeminiClient, GenerateContentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GeminiClient::from_env()?;
    
    let response = client.generate_content(
        GenerateContentRequest::with_text("Hello, Gemini!")
    ).await?;
    
    println!("{}", response.text().unwrap_or("No response"));
    Ok(())
}
```

### SDK Features

- ✅ Text generation with streaming
- ✅ Image generation (Gemini 3, Imagen 4)
- ✅ Video generation (Veo 2, 3, 3.1)
- ✅ Audio understanding and TTS
- ✅ Document/PDF processing
- ✅ Vision and object detection
- ✅ Embeddings with multiple task types
- ✅ Context caching (implicit & explicit)
- ✅ Batch API (50% cost savings)
- ✅ Function calling
- ✅ Structured output (JSON schema)

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## ❓ FAQ

**Q: Where should I start if I'm new to prompt engineering?**

A: Begin by asking yourself: *What makes a good prompt?* A good prompt is clear, specific, and provides enough context for the AI to understand your intent. Try to define the persona, task, constraints, and desired output format. You can start by entering a simple draft prompt and let this tool guide you toward a more professional version!

**Q: Should I use the master prompt directly?**

A: Yes, in most cases! The master prompt combines the best elements from all generated versions. However, you may want to fine-tune some settings in the prompt to better match your specific use case, such as adjusting the persona, adding domain-specific constraints, or modifying the output format.

---

## ⭐ Star History

If this project helped you, please consider giving it a star! ⭐

Your support helps us continue improving this tool and keeps us motivated!

---

<p align="center">
  Made with ❤️ by the Prompt of Prompt Team
</p>

<p align="center">
  <a href="https://github.com/anthropics/the-prompt-of-prompt/issues">Report Bug</a> •
  <a href="https://github.com/anthropics/the-prompt-of-prompt/issues">Request Feature</a>
</p>

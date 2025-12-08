//! Internationalization (i18n) support for the Prompt Improver CLI
//!
//! Supports English (default) and Chinese based on the LANG environment variable.

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    /// Detect language from environment
    pub fn from_env() -> Self {
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("zh") {
                return Language::Chinese;
            }
        }
        // Also check LANGUAGE and LC_ALL
        if let Ok(lang) = std::env::var("LANGUAGE") {
            if lang.starts_with("zh") {
                return Language::Chinese;
            }
        }
        if let Ok(lang) = std::env::var("LC_ALL") {
            if lang.starts_with("zh") {
                return Language::Chinese;
            }
        }
        Language::English
    }
}

/// All translatable strings
pub struct I18n {
    pub lang: Language,
}

impl I18n {
    pub fn new() -> Self {
        Self {
            lang: Language::from_env(),
        }
    }

    // Header
    pub fn header_title(&self) -> &'static str {
        match self.lang {
            Language::English => "🚀 Gemini Prompt Improver",
            Language::Chinese => "🚀 Gemini 提示词优化器",
        }
    }

    pub fn header_subtitle(&self) -> &'static str {
        match self.lang {
            Language::English => "Transform your prompts into professional ones",
            Language::Chinese => "将你的提示词转化为专业提示词",
        }
    }

    // Input
    pub fn enter_prompt(&self) -> &'static str {
        match self.lang {
            Language::English => "📝 Enter your draft prompt (press Enter twice to submit):",
            Language::Chinese => "📝 请输入你的草稿提示词（按两次回车提交）：",
        }
    }

    // Errors
    pub fn error_init_client(&self) -> &'static str {
        match self.lang {
            Language::English => "❌ Error: Failed to initialize Gemini client.",
            Language::Chinese => "❌ 错误：初始化 Gemini 客户端失败。",
        }
    }

    pub fn error_set_api_key(&self) -> &'static str {
        match self.lang {
            Language::English => "   Make sure GEMINI_API_KEY environment variable is set.",
            Language::Chinese => "   请确保已设置 GEMINI_API_KEY 环境变量。",
        }
    }

    pub fn error_no_prompt(&self) -> &'static str {
        match self.lang {
            Language::English => "❌ Error: No prompt provided.",
            Language::Chinese => "❌ 错误：未提供提示词。",
        }
    }

    pub fn error_batch_create(&self) -> &'static str {
        match self.lang {
            Language::English => "❌ Error: Failed to create batch job",
            Language::Chinese => "❌ 错误：创建批处理任务失败",
        }
    }

    pub fn error_batch_failed(&self) -> &'static str {
        match self.lang {
            Language::English => "❌ Error: Batch job failed",
            Language::Chinese => "❌ 错误：批处理任务失败",
        }
    }

    pub fn error_no_responses(&self) -> &'static str {
        match self.lang {
            Language::English => "❌ Error: No responses in batch job",
            Language::Chinese => "❌ 错误：批处理任务无响应",
        }
    }

    // Progress
    pub fn improving_prompt_with_count(&self, count: usize) -> String {
        match self.lang {
            Language::English => {
                format!("⏳ Improving your prompt with Gemini 3 Pro (Batch API - {} versions)...", count)
            }
            Language::Chinese => {
                format!("⏳ 正在使用 Gemini 3 Pro 优化你的提示词（Batch API - {}个版本）...", count)
            }
        }
    }

    pub fn batch_api_note(&self) -> &'static str {
        match self.lang {
            Language::English => "   💡 Using Batch API: 50% cheaper than real-time requests!",
            Language::Chinese => "   💡 使用 Batch API：比实时请求便宜 50%！",
        }
    }

    pub fn using_proxy(&self, proxy: &str) -> String {
        match self.lang {
            Language::English => format!("🌐 Using proxy: {}", proxy),
            Language::Chinese => format!("🌐 使用代理：{}", proxy),
        }
    }

    pub fn creating_batch_with_count(&self, count: usize) -> String {
        match self.lang {
            Language::English => format!("📤 Creating batch job with {} requests...", count),
            Language::Chinese => format!("📤 正在创建包含 {} 个请求的批处理任务...", count),
        }
    }

    pub fn batch_created(&self, name: &str) -> String {
        match self.lang {
            Language::English => format!("✅ Batch job created: {}", name),
            Language::Chinese => format!("✅ 批处理任务已创建：{}", name),
        }
    }

    pub fn waiting_batch(&self) -> &'static str {
        match self.lang {
            Language::English => {
                "⏳ Waiting for batch job to complete (polling every 5 seconds)..."
            }
            Language::Chinese => "⏳ 等待批处理任务完成（每5秒轮询一次）...",
        }
    }

    pub fn batch_completed(&self) -> &'static str {
        match self.lang {
            Language::English => "📥 Batch job completed!",
            Language::Chinese => "📥 批处理任务已完成！",
        }
    }

    // Output
    pub fn version_saved(&self, version: usize, file: &str) -> String {
        match self.lang {
            Language::English => format!("💾 Version {} saved to: {}", version, file),
            Language::Chinese => format!("💾 版本 {} 已保存到：{}", version, file),
        }
    }

    pub fn generated_versions_with_count(&self, count: usize) -> String {
        match self.lang {
            Language::English => format!("📄 Generated {} versions of your improved prompt:", count),
            Language::Chinese => format!("📄 已生成 {} 个优化版本：", count),
        }
    }

    pub fn merging_prompts_with_count(&self, count: usize) -> String {
        match self.lang {
            Language::English => format!("🔀 Merging {} versions into a Master Prompt...", count),
            Language::Chinese => format!("🔀 正在将 {} 个版本合并为大师级提示词...", count),
        }
    }

    pub fn master_saved(&self) -> &'static str {
        match self.lang {
            Language::English => "✅ Master Prompt saved to: master_prompt.md",
            Language::Chinese => "✅ 大师级提示词已保存到：master_prompt.md",
        }
    }

    pub fn merge_failed(&self) -> &'static str {
        match self.lang {
            Language::English => "⚠️  Warning: Failed to merge prompts",
            Language::Chinese => "⚠️  警告：合并提示词失败",
        }
    }

    pub fn manual_merge_hint(&self) -> &'static str {
        match self.lang {
            Language::English => "   You can manually merge the versions.",
            Language::Chinese => "   你可以手动合并这些版本。",
        }
    }

    pub fn final_output(&self) -> &'static str {
        match self.lang {
            Language::English => "🏆 Final Output:",
            Language::Chinese => "🏆 最终输出：",
        }
    }

    pub fn version_label(&self, version: usize) -> String {
        match self.lang {
            Language::English => format!("Version {}", version),
            Language::Chinese => format!("版本 {}", version),
        }
    }

    pub fn merged_optimized(&self) -> &'static str {
        match self.lang {
            Language::English => "✨ Merged & Optimized",
            Language::Chinese => "✨ 合并优化版",
        }
    }

    // Token usage
    pub fn token_usage_header(&self) -> &'static str {
        match self.lang {
            Language::English => "📊 Total Token Usage (3 batch + 1 merge request):",
            Language::Chinese => "📊 总 Token 使用量（3 个批处理 + 1 个合并请求）：",
        }
    }

    pub fn input_tokens(&self) -> &'static str {
        match self.lang {
            Language::English => "   Input:",
            Language::Chinese => "   输入：",
        }
    }

    pub fn output_tokens(&self) -> &'static str {
        match self.lang {
            Language::English => "   Output:",
            Language::Chinese => "   输出：",
        }
    }

    pub fn thinking_tokens(&self) -> &'static str {
        match self.lang {
            Language::English => "   Thinking:",
            Language::Chinese => "   思考：",
        }
    }

    pub fn total_tokens(&self) -> &'static str {
        match self.lang {
            Language::English => "   Total:",
            Language::Chinese => "   总计：",
        }
    }

    pub fn cost_header(&self) -> &'static str {
        match self.lang {
            Language::English => "💰 Estimated Cost (Batch + Real-time API):",
            Language::Chinese => "💰 预估成本（Batch + 实时 API）：",
        }
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

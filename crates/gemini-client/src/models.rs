//! Gemini model definitions and constants.
//!
//! This module contains constants for all available Gemini models
//! and their capabilities.

/// Gemini 3 Pro - Most intelligent model for multimodal understanding
pub mod gemini_3 {
    /// Gemini 3 Pro Preview - Best for multimodal understanding and agentic tasks
    pub const PRO_PREVIEW: &str = "gemini-3-pro-preview";
    /// Gemini 3 Pro Image Preview - Image generation support
    pub const PRO_IMAGE_PREVIEW: &str = "gemini-3-pro-image-preview";
}

/// Gemini 2.5 Flash - Fast and intelligent
pub mod gemini_2_5_flash {
    /// Gemini 2.5 Flash - Stable, best price-performance
    pub const STABLE: &str = "gemini-2.5-flash";
    /// Gemini 2.5 Flash Preview
    pub const PREVIEW: &str = "gemini-2.5-flash-preview-09-2025";
    /// Gemini 2.5 Flash Image - Image generation
    pub const IMAGE: &str = "gemini-2.5-flash-image";
    /// Gemini 2.5 Flash Live - Audio/video streaming
    pub const LIVE: &str = "gemini-2.5-flash-live";
    /// Gemini 2.5 Flash Preview Native Audio
    pub const NATIVE_AUDIO: &str = "gemini-2.5-flash-preview-native-audio";
    /// Gemini 2.5 Flash TTS - Text-to-speech
    pub const TTS: &str = "gemini-2.5-flash-preview-tts";
}

/// Gemini 2.5 Flash-Lite - Ultra fast, cost-efficient
pub mod gemini_2_5_flash_lite {
    /// Gemini 2.5 Flash-Lite - Stable
    pub const STABLE: &str = "gemini-2.5-flash-lite";
    /// Gemini 2.5 Flash-Lite Preview
    pub const PREVIEW: &str = "gemini-2.5-flash-lite-preview-09-2025";
}

/// Gemini 2.5 Pro - Advanced thinking model
pub mod gemini_2_5_pro {
    /// Gemini 2.5 Pro - Stable, state-of-the-art reasoning
    pub const STABLE: &str = "gemini-2.5-pro";
    /// Gemini 2.5 Pro TTS - Text-to-speech
    pub const TTS: &str = "gemini-2.5-pro-preview-tts";
}

/// Gemini 2.0 Flash - Second generation workhorse
pub mod gemini_2_0_flash {
    /// Gemini 2.0 Flash - Latest
    pub const LATEST: &str = "gemini-2.0-flash";
    /// Gemini 2.0 Flash - Stable
    pub const STABLE: &str = "gemini-2.0-flash-001";
    /// Gemini 2.0 Flash - Experimental
    pub const EXPERIMENTAL: &str = "gemini-2.0-flash-exp";
    /// Gemini 2.0 Flash Image Generation
    pub const IMAGE: &str = "gemini-2.0-flash-preview-image-generation";
    /// Gemini 2.0 Flash Live (deprecated Dec 9, 2025)
    pub const LIVE: &str = "gemini-2.0-flash-live-001";
}

/// Gemini 2.0 Flash-Lite - Second generation fast model
pub mod gemini_2_0_flash_lite {
    /// Gemini 2.0 Flash-Lite - Latest
    pub const LATEST: &str = "gemini-2.0-flash-lite";
    /// Gemini 2.0 Flash-Lite - Stable
    pub const STABLE: &str = "gemini-2.0-flash-lite-001";
}

/// Embedding models
pub mod embeddings {
    /// Gemini Embedding 001 - Latest recommended model (3072 dimensions, supports MRL)
    pub const GEMINI_EMBEDDING_001: &str = "gemini-embedding-001";
    /// Experimental embedding model (deprecating Oct 2025)
    pub const GEMINI_EMBEDDING_EXP: &str = "gemini-embedding-exp-03-07";
    /// Legacy text embedding model (deprecating Oct 2025)
    pub const TEXT_EMBEDDING_004: &str = "text-embedding-004";
}

/// Veo 3.1 video generation models (8s 720p/1080p with audio)
pub mod veo_3_1 {
    /// Veo 3.1 Preview - Best quality, 720p/1080p with audio
    pub const PREVIEW: &str = "veo-3.1-generate-preview";
    /// Veo 3.1 Fast Preview - Optimized for speed
    pub const FAST: &str = "veo-3.1-fast-generate-preview";
}

/// Veo 3 video generation models (8s with audio)
pub mod veo_3 {
    /// Veo 3 Stable
    pub const STABLE: &str = "veo-3.0-generate-001";
    /// Veo 3 Fast - Optimized for speed
    pub const FAST: &str = "veo-3.0-fast-generate-001";
}

/// Veo 2 video generation models (5-8s, silent)
pub mod veo_2 {
    /// Veo 2 Stable
    pub const STABLE: &str = "veo-2.0-generate-001";
}

/// Imagen 4 image generation models
pub mod imagen_4 {
    /// Imagen 4 Standard - Best quality
    pub const STANDARD: &str = "imagen-4.0-generate-001";
    /// Imagen 4 Ultra - Highest quality
    pub const ULTRA: &str = "imagen-4.0-ultra-generate-001";
    /// Imagen 4 Fast - Speed optimized
    pub const FAST: &str = "imagen-4.0-fast-generate-001";
}

/// Imagen 3 image generation models
pub mod imagen_3 {
    /// Imagen 3 - State-of-the-art image generation
    pub const STABLE: &str = "imagen-3.0-generate-002";
}

/// Gemini Robotics-ER - Embodied reasoning for robotics
pub mod gemini_robotics {
    /// Robotics-ER 1.5 Preview - Physical world understanding
    pub const ER_1_5_PREVIEW: &str = "gemini-robotics-er-1.5-preview";
}

/// Gemini 2.5 Computer Use - Browser automation
pub mod gemini_computer_use {
    /// Computer Use Preview - Browser control agents
    pub const PREVIEW: &str = "gemini-2.5-computer-use-preview-10-2025";
}

/// Gemma open models
pub mod gemma {
    /// Gemma 3 - Lightweight open model
    pub const GEMMA_3: &str = "gemma-3";
    /// Gemma 3n - Efficient for everyday devices
    pub const GEMMA_3N: &str = "gemma-3n";
}

/// Default models for different use cases
pub mod defaults {
    use super::*;

    /// Default model for general use (best price-performance)
    pub const GENERAL: &str = gemini_2_5_flash::STABLE;

    /// Default model for advanced reasoning
    pub const REASONING: &str = gemini_2_5_pro::STABLE;

    /// Default model for fastest responses
    pub const FAST: &str = gemini_2_5_flash_lite::STABLE;

    /// Default model for highest capability
    pub const BEST: &str = gemini_3::PRO_PREVIEW;

    /// Default model for embeddings (gemini-embedding-001)
    pub const EMBEDDING: &str = embeddings::GEMINI_EMBEDDING_001;

    /// Default model for image generation (Gemini native)
    pub const IMAGE_GEN: &str = gemini_2_5_flash::IMAGE;

    /// Default model for image generation (Imagen)
    pub const IMAGEN: &str = imagen_4::STANDARD;

    /// Default model for video generation (Veo 3.1)
    pub const VIDEO_GEN: &str = veo_3_1::PREVIEW;
}

/// Model capabilities information
#[derive(Debug, Clone)]
pub struct ModelCapabilities {
    pub audio_generation: bool,
    pub batch_api: bool,
    pub caching: bool,
    pub code_execution: bool,
    pub file_search: bool,
    pub function_calling: bool,
    pub google_maps_grounding: bool,
    pub image_generation: bool,
    pub live_api: bool,
    pub search_grounding: bool,
    pub structured_outputs: bool,
    pub thinking: bool,
    pub url_context: bool,
}

impl ModelCapabilities {
    /// Capabilities for Gemini 2.5 Flash
    pub fn gemini_2_5_flash() -> Self {
        Self {
            audio_generation: false,
            batch_api: true,
            caching: true,
            code_execution: true,
            file_search: true,
            function_calling: true,
            google_maps_grounding: true,
            image_generation: false,
            live_api: false,
            search_grounding: true,
            structured_outputs: true,
            thinking: true,
            url_context: true,
        }
    }

    /// Capabilities for Gemini 2.5 Pro
    pub fn gemini_2_5_pro() -> Self {
        Self {
            audio_generation: false,
            batch_api: true,
            caching: true,
            code_execution: true,
            file_search: true,
            function_calling: true,
            google_maps_grounding: true,
            image_generation: false,
            live_api: false,
            search_grounding: true,
            structured_outputs: true,
            thinking: true,
            url_context: true,
        }
    }

    /// Capabilities for Gemini 3 Pro Preview
    pub fn gemini_3_pro() -> Self {
        Self {
            audio_generation: false,
            batch_api: true,
            caching: true,
            code_execution: true,
            file_search: true,
            function_calling: true,
            google_maps_grounding: false,
            image_generation: false,
            live_api: false,
            search_grounding: true,
            structured_outputs: true,
            thinking: true,
            url_context: true,
        }
    }
}

/// Token limits for different models
#[derive(Debug, Clone, Copy)]
pub struct TokenLimits {
    pub input: u32,
    pub output: u32,
}

impl TokenLimits {
    /// Token limits for most Gemini 2.5/3 models
    pub const STANDARD: Self = Self {
        input: 1_048_576,
        output: 65_536,
    };

    /// Token limits for image generation models
    pub const IMAGE_GEN: Self = Self {
        input: 65_536,
        output: 32_768,
    };

    /// Token limits for TTS models
    pub const TTS: Self = Self {
        input: 8_192,
        output: 16_384,
    };

    /// Token limits for live/streaming models
    pub const LIVE: Self = Self {
        input: 131_072,
        output: 8_192,
    };

    /// Token limits for Gemini 2.0 models
    pub const GEMINI_2_0: Self = Self {
        input: 1_048_576,
        output: 8_192,
    };
}

/// Rate limits for Tier 1 (paid billing account linked)
/// Use these to understand your API limits
pub mod rate_limits {
    /// Rate limit info for a model
    #[derive(Debug, Clone, Copy)]
    pub struct RateLimit {
        /// Requests per minute
        pub rpm: u32,
        /// Tokens per minute (input)
        pub tpm: u32,
        /// Requests per day (0 = unlimited)
        pub rpd: u32,
        /// Batch enqueued tokens (0 = not supported)
        pub batch_tokens: u64,
    }

    impl RateLimit {
        pub const fn new(rpm: u32, tpm: u32, rpd: u32, batch_tokens: u64) -> Self {
            Self {
                rpm,
                tpm,
                rpd,
                batch_tokens,
            }
        }
    }

    /// Tier 1 rate limits for common models
    pub mod tier_1 {
        use super::RateLimit;

        /// Gemini 3 Pro Preview: 50 RPM, 1M TPM, 1K RPD
        pub const GEMINI_3_PRO: RateLimit = RateLimit::new(50, 1_000_000, 1_000, 50_000_000);

        /// Gemini 2.5 Pro: 150 RPM, 2M TPM, 10K RPD
        pub const GEMINI_2_5_PRO: RateLimit = RateLimit::new(150, 2_000_000, 10_000, 5_000_000);

        /// Gemini 2.5 Flash: 1000 RPM, 1M TPM, 10K RPD
        pub const GEMINI_2_5_FLASH: RateLimit = RateLimit::new(1_000, 1_000_000, 10_000, 3_000_000);

        /// Gemini 2.5 Flash-Lite: 4000 RPM, 4M TPM, unlimited RPD
        pub const GEMINI_2_5_FLASH_LITE: RateLimit =
            RateLimit::new(4_000, 4_000_000, 0, 10_000_000);

        /// Gemini 2.0 Flash: 2000 RPM, 4M TPM, unlimited RPD
        pub const GEMINI_2_0_FLASH: RateLimit = RateLimit::new(2_000, 4_000_000, 0, 10_000_000);

        /// Gemini 2.0 Flash-Lite: 4000 RPM, 4M TPM, unlimited RPD
        pub const GEMINI_2_0_FLASH_LITE: RateLimit =
            RateLimit::new(4_000, 4_000_000, 0, 10_000_000);

        /// Gemini 2.5 Flash Image: 500 RPM, 500K TPM, 2K RPD
        pub const GEMINI_2_5_FLASH_IMAGE: RateLimit = RateLimit::new(500, 500_000, 2_000, 0);

        /// Gemini 2.0 Flash Image: 1000 RPM, 1M TPM, 10K RPD
        pub const GEMINI_2_0_FLASH_IMAGE: RateLimit = RateLimit::new(1_000, 1_000_000, 10_000, 0);

        /// Imagen 4 Standard/Fast: 10 RPM, 70 RPD
        pub const IMAGEN_4: RateLimit = RateLimit::new(10, 0, 70, 0);

        /// Imagen 4 Ultra: 5 RPM, 30 RPD
        pub const IMAGEN_4_ULTRA: RateLimit = RateLimit::new(5, 0, 30, 0);

        /// Imagen 3: 20 RPM
        pub const IMAGEN_3: RateLimit = RateLimit::new(20, 0, 0, 0);

        /// Veo 3.1/3: 2 RPM, 10 RPD
        pub const VEO_3: RateLimit = RateLimit::new(2, 0, 10, 0);

        /// Veo 2: 2 RPM, 50 RPD
        pub const VEO_2: RateLimit = RateLimit::new(2, 0, 50, 0);

        /// Gemini Embedding: 3000 RPM, 1M TPM
        pub const EMBEDDING: RateLimit = RateLimit::new(3_000, 1_000_000, 0, 0);

        /// Gemma 3/3n: 30 RPM, 15K TPM, 14.4K RPD
        pub const GEMMA: RateLimit = RateLimit::new(30, 15_000, 14_400, 0);

        /// Gemini Robotics-ER: 300 RPM, 1M TPM, 10K RPD
        pub const ROBOTICS_ER: RateLimit = RateLimit::new(300, 1_000_000, 10_000, 0);

        /// Gemini Computer Use: 150 RPM, 2M TPM, 10K RPD
        pub const COMPUTER_USE: RateLimit = RateLimit::new(150, 2_000_000, 10_000, 0);

        /// Gemini 2.5 Flash TTS: 10 RPM, 10K TPM, 100 RPD
        pub const FLASH_TTS: RateLimit = RateLimit::new(10, 10_000, 100, 0);

        /// Gemini 2.5 Pro TTS: 10 RPM, 10K TPM, 50 RPD
        pub const PRO_TTS: RateLimit = RateLimit::new(10, 10_000, 50, 0);

        /// Gemini 3 Pro Image: 20 RPM, 100K TPM, 250 RPD
        pub const GEMINI_3_IMAGE: RateLimit = RateLimit::new(20, 100_000, 250, 2_000_000);
    }
}

/// Thinking budget ranges for Gemini models.
/// Use with `ThinkingConfig::budget(tokens)`.
pub mod thinking_budget {
    /// Dynamic thinking: model decides based on task complexity.
    pub const DYNAMIC: i32 = -1;

    /// Disable thinking (Gemini 2.5 Flash/Lite only, not supported on 2.5 Pro or 3 Pro).
    pub const OFF: i32 = 0;

    /// Gemini 2.5 Pro thinking budget range.
    pub mod pro_2_5 {
        /// Minimum thinking budget for 2.5 Pro.
        pub const MIN: i32 = 128;
        /// Maximum thinking budget for 2.5 Pro.
        pub const MAX: i32 = 32768;
    }

    /// Gemini 2.5 Flash thinking budget range.
    pub mod flash_2_5 {
        /// Minimum thinking budget for 2.5 Flash (0 to disable).
        pub const MIN: i32 = 0;
        /// Maximum thinking budget for 2.5 Flash.
        pub const MAX: i32 = 24576;
    }

    /// Gemini 2.5 Flash Lite thinking budget range.
    pub mod flash_lite_2_5 {
        /// Minimum thinking budget for 2.5 Flash Lite.
        pub const MIN: i32 = 512;
        /// Maximum thinking budget for 2.5 Flash Lite.
        pub const MAX: i32 = 24576;
    }
}

/// Explicit caching requires versioned model names (with -001 suffix).
/// Use these constants when creating cached content.
///
/// Note: Implicit caching (automatic on Gemini 2.5 models) doesn't require these.
pub mod cacheable {
    /// Gemini 2.0 Flash for caching (requires -001 suffix).
    pub const GEMINI_2_0_FLASH: &str = "gemini-2.0-flash-001";
    /// Gemini 2.0 Flash Lite for caching.
    pub const GEMINI_2_0_FLASH_LITE: &str = "gemini-2.0-flash-lite-001";
    /// Gemini 2.0 Flash Live for caching.
    pub const GEMINI_2_0_FLASH_LIVE: &str = "gemini-2.0-flash-live-001";

    /// Minimum token counts for caching by model family.
    pub mod min_tokens {
        /// Gemini 3 Pro Preview minimum tokens for caching.
        pub const GEMINI_3_PRO: u32 = 2048;
        /// Gemini 2.5 Pro minimum tokens for caching.
        pub const GEMINI_2_5_PRO: u32 = 4096;
        /// Gemini 2.5 Flash minimum tokens for caching.
        pub const GEMINI_2_5_FLASH: u32 = 1024;
    }
}

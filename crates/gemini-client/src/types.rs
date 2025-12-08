//! Core types used across the Gemini API.

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

// ============================================================================
// Content Types
// ============================================================================

/// Represents content for the model (prompt or response).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    /// The parts that make up the content.
    #[serde(default)]
    pub parts: Vec<Part>,
    /// The role of the content creator (user or model).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl Content {
    /// Create a new text content with the given role.
    pub fn new(role: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::text(text)],
            role: Some(role.into()),
        }
    }

    /// Create user content.
    pub fn user(text: impl Into<String>) -> Self {
        Self::new("user", text)
    }

    /// Create model content.
    pub fn model(text: impl Into<String>) -> Self {
        Self::new("model", text)
    }

    /// Create system instruction content.
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::text(text)],
            role: None, // System instructions don't have a role
        }
    }

    /// Create user content with a file reference.
    pub fn user_with_file(file_uri: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::file_data(file_uri, mime_type)],
            role: Some("user".to_string()),
        }
    }

    /// Create user content with text and a file reference.
    pub fn user_with_text_and_file(
        text: impl Into<String>,
        file_uri: impl Into<String>,
        mime_type: impl Into<String>,
    ) -> Self {
        Self {
            parts: vec![Part::file_data(file_uri, mime_type), Part::text(text)],
            role: Some("user".to_string()),
        }
    }

    /// Add a part to the content.
    pub fn add_part(mut self, part: Part) -> Self {
        self.parts.push(part);
        self
    }

    /// Add a file part to the content.
    pub fn add_file(mut self, file_uri: impl Into<String>, mime_type: impl Into<String>) -> Self {
        self.parts.push(Part::file_data(file_uri, mime_type));
        self
    }
}

/// A part of content - can be text, inline data, or file reference.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    /// Text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Inline binary data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<Blob>,
    /// Reference to a file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,
    /// Function call made by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    /// Response to a function call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<FunctionResponse>,
    /// Thought signature for maintaining reasoning context (Gemini 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
    /// Media resolution for images/video (Gemini 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resolution: Option<MediaResolutionConfig>,
    /// Whether this part is a "thought" (intermediate reasoning, not charged).
    /// Used by Gemini 3 Pro Image for interim thought images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought: Option<bool>,
    /// Video metadata for clipping and frame rate control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

impl Part {
    /// Create a text part.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            inline_data: None,
            file_data: None,
            function_call: None,
            function_response: None,
            thought_signature: None,
            media_resolution: None,
            thought: None,
            video_metadata: None,
        }
    }

    /// Create an inline data part.
    pub fn inline_data(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            text: None,
            inline_data: Some(Blob {
                mime_type: mime_type.into(),
                data: data.into(),
            }),
            file_data: None,
            function_call: None,
            function_response: None,
            thought_signature: None,
            media_resolution: None,
            thought: None,
            video_metadata: None,
        }
    }

    /// Create a file data part.
    pub fn file_data(mime_type: impl Into<String>, file_uri: impl Into<String>) -> Self {
        Self {
            text: None,
            inline_data: None,
            file_data: Some(FileData {
                mime_type: mime_type.into(),
                file_uri: file_uri.into(),
            }),
            function_call: None,
            function_response: None,
            thought_signature: None,
            media_resolution: None,
            thought: None,
            video_metadata: None,
        }
    }

    /// Create a YouTube video part.
    /// Example: `Part::youtube("https://www.youtube.com/watch?v=VIDEO_ID")`
    pub fn youtube(url: impl Into<String>) -> Self {
        Self {
            text: None,
            inline_data: None,
            file_data: Some(FileData {
                mime_type: "video/*".to_string(),
                file_uri: url.into(),
            }),
            function_call: None,
            function_response: None,
            thought_signature: None,
            media_resolution: None,
            thought: None,
            video_metadata: None,
        }
    }

    /// Create a YouTube video part with clipping.
    /// `start` and `end` are in seconds (e.g., "1250s", "1570s").
    pub fn youtube_clip(
        url: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            text: None,
            inline_data: None,
            file_data: Some(FileData {
                mime_type: "video/*".to_string(),
                file_uri: url.into(),
            }),
            function_call: None,
            function_response: None,
            thought_signature: None,
            media_resolution: None,
            thought: None,
            video_metadata: Some(VideoMetadata {
                start_offset: Some(start.into()),
                end_offset: Some(end.into()),
                fps: None,
            }),
        }
    }

    /// Create a PDF document part from base64-encoded data.
    /// Gemini can process PDFs up to 50MB or 1000 pages.
    /// Each page ≈ 258 tokens.
    pub fn pdf(base64_data: impl Into<String>) -> Self {
        Self::inline_data("application/pdf", base64_data)
    }

    /// Create a PDF document part from raw bytes.
    /// Automatically base64-encodes the data.
    pub fn pdf_bytes(data: &[u8]) -> Self {
        Self::inline_data("application/pdf", general_purpose::STANDARD.encode(data))
    }

    // ========================================================================
    // Audio Input Helpers
    // ========================================================================
    // Supported formats: WAV, MP3, AIFF, AAC, OGG Vorbis, FLAC
    // - Each second of audio = 32 tokens (1 minute = 1,920 tokens)
    // - Max audio length: 9.5 hours per prompt
    // - Audio is downsampled to 16 Kbps
    // - Multi-channel audio is combined to mono

    /// Create an audio part from base64-encoded data.
    /// Supported MIME types: audio/wav, audio/mp3, audio/aiff, audio/aac, audio/ogg, audio/flac
    pub fn audio(mime_type: impl Into<String>, base64_data: impl Into<String>) -> Self {
        Self::inline_data(mime_type, base64_data)
    }

    /// Create an audio part from raw bytes.
    /// Automatically base64-encodes the data.
    pub fn audio_bytes(mime_type: impl Into<String>, data: &[u8]) -> Self {
        Self::inline_data(mime_type, general_purpose::STANDARD.encode(data))
    }

    /// Create an MP3 audio part from raw bytes.
    pub fn mp3(data: &[u8]) -> Self {
        Self::audio_bytes("audio/mp3", data)
    }

    /// Create a WAV audio part from raw bytes.
    pub fn wav(data: &[u8]) -> Self {
        Self::audio_bytes("audio/wav", data)
    }

    /// Create an AAC audio part from raw bytes.
    pub fn aac(data: &[u8]) -> Self {
        Self::audio_bytes("audio/aac", data)
    }

    /// Create an OGG Vorbis audio part from raw bytes.
    pub fn ogg(data: &[u8]) -> Self {
        Self::audio_bytes("audio/ogg", data)
    }

    /// Create a FLAC audio part from raw bytes.
    pub fn flac(data: &[u8]) -> Self {
        Self::audio_bytes("audio/flac", data)
    }

    /// Create an AIFF audio part from raw bytes.
    pub fn aiff(data: &[u8]) -> Self {
        Self::audio_bytes("audio/aiff", data)
    }

    /// Create an inline data part with media resolution (Gemini 3).
    pub fn inline_data_with_resolution(
        mime_type: impl Into<String>,
        data: impl Into<String>,
        resolution: MediaResolution,
    ) -> Self {
        Self {
            text: None,
            inline_data: Some(Blob {
                mime_type: mime_type.into(),
                data: data.into(),
            }),
            file_data: None,
            function_call: None,
            function_response: None,
            thought_signature: None,
            media_resolution: Some(MediaResolutionConfig { level: resolution }),
            thought: None,
            video_metadata: None,
        }
    }

    /// Set video metadata (clipping, FPS) on this part.
    pub fn with_video_metadata(mut self, metadata: VideoMetadata) -> Self {
        self.video_metadata = Some(metadata);
        self
    }

    /// Check if this part is a thought (intermediate reasoning).
    pub fn is_thought(&self) -> bool {
        self.thought.unwrap_or(false)
    }

    /// Set the thought signature on this part.
    /// Required for Gemini 3 Pro function calling - must pass back signatures exactly as received.
    pub fn with_thought_signature(mut self, signature: impl Into<String>) -> Self {
        self.thought_signature = Some(signature.into());
        self
    }

    /// Check if this part has a thought signature.
    pub fn has_thought_signature(&self) -> bool {
        self.thought_signature.is_some()
    }

    /// Get the thought signature if present.
    pub fn get_thought_signature(&self) -> Option<&str> {
        self.thought_signature.as_deref()
    }

    /// Create a function response part.
    pub fn function_response(name: impl Into<String>, response: serde_json::Value) -> Self {
        Self {
            text: None,
            inline_data: None,
            file_data: None,
            function_call: None,
            function_response: Some(FunctionResponse {
                name: name.into(),
                response,
            }),
            thought_signature: None,
            media_resolution: None,
            thought: None,
            video_metadata: None,
        }
    }
}

/// Dummy thought signatures for skipping validation.
/// Use these when injecting custom function call blocks or transferring traces from other models.
///
/// # Example
/// ```
/// use gemini_client::{Part, thought_signatures};
///
/// let part = Part::text("Custom function call context")
///     .with_thought_signature(thought_signatures::SKIP_VALIDATION);
/// ```
pub mod thought_signatures {
    /// Dummy signature to skip thought signature validation.
    /// Use when transferring history from a different model to Gemini 3 Pro.
    pub const SKIP_VALIDATION: &str = "skip_thought_signature_validator";

    /// Alternative dummy signature for context engineering scenarios.
    pub const CONTEXT_ENGINEERING: &str = "context_engineering_is_the_way_to_go";
}

/// Binary data with MIME type.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    /// MIME type of the data.
    pub mime_type: String,
    /// Base64-encoded data.
    pub data: String,
}

/// Reference to a file in the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileData {
    /// MIME type of the file.
    pub mime_type: String,
    /// URI of the file (can be a File API URI or YouTube URL).
    pub file_uri: String,
}

/// Video metadata for clipping intervals and frame rate control.
/// Used with video inputs to customize processing.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadata {
    /// Start offset for video clipping (e.g., "1250s" for 1250 seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<String>,
    /// End offset for video clipping (e.g., "1570s" for 1570 seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<String>,
    /// Custom frame rate for video sampling.
    /// Default is 1 FPS. Use lower values (<1) for long/static videos,
    /// higher values for fast action sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f32>,
}

impl VideoMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set video clipping interval.
    /// Offsets are in seconds (e.g., "40s", "1250s").
    pub fn clip(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            start_offset: Some(start.into()),
            end_offset: Some(end.into()),
            fps: None,
        }
    }

    /// Set custom frame rate.
    /// Default is 1 FPS. Use lower for long videos, higher for fast action.
    pub fn with_fps(mut self, fps: f32) -> Self {
        self.fps = Some(fps);
        self
    }

    /// Set start offset.
    pub fn start(mut self, offset: impl Into<String>) -> Self {
        self.start_offset = Some(offset.into());
        self
    }

    /// Set end offset.
    pub fn end(mut self, offset: impl Into<String>) -> Self {
        self.end_offset = Some(offset.into());
        self
    }
}

// ============================================================================
// Gemini 3 Features
// ============================================================================

/// Media resolution levels for controlling token usage (Gemini 3).
/// Higher resolutions improve fine text/detail recognition but increase tokens.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MediaResolution {
    /// Low resolution - 280 tokens for images, 70 tokens/frame for video.
    MediaResolutionLow,
    /// Medium resolution - 560 tokens for images, 70 tokens/frame for video.
    MediaResolutionMedium,
    /// High resolution - 1120 tokens for images, 280 tokens/frame for video.
    MediaResolutionHigh,
}

/// Configuration for media resolution.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaResolutionConfig {
    /// The resolution level.
    pub level: MediaResolution,
}

/// Thinking level for controlling reasoning depth (Gemini 3).
/// Controls the maximum depth of internal reasoning before producing a response.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingLevel {
    /// Minimizes latency and cost. Best for simple tasks.
    Low,
    /// Medium reasoning depth (coming soon).
    Medium,
    /// Maximizes reasoning depth. Default for Gemini 3 Pro.
    High,
}

/// Thinking configuration for Gemini 2.5 and 3.x models.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    /// The thinking level: "low" or "high" (Gemini 3 only).
    /// Default is "high" for Gemini 3 Pro. Cannot disable thinking on Gemini 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_level: Option<ThinkingLevel>,
    /// Thinking budget in tokens (Gemini 2.5 series).
    /// - 2.5 Pro: 128-32768 (cannot disable)
    /// - 2.5 Flash: 0-24576 (0 to disable)
    /// - 2.5 Flash Lite: 512-24576 (0 to disable)
    /// Set to -1 for dynamic thinking (model decides).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<i32>,
    /// Include thought summaries in the response.
    /// When true, response parts may have `thought: true` for thought summaries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_thoughts: Option<bool>,
}

impl ThinkingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set thinking level (recommended for Gemini 3).
    /// - `Low`: Minimizes latency and cost
    /// - `High`: Maximizes reasoning depth (default for Gemini 3 Pro)
    pub fn level(mut self, level: ThinkingLevel) -> Self {
        self.thinking_level = Some(level);
        self
    }

    /// Set thinking budget in tokens (for Gemini 2.5 series).
    /// Use -1 for dynamic thinking (model decides based on complexity).
    pub fn budget(mut self, tokens: i32) -> Self {
        self.thinking_budget = Some(tokens);
        self
    }

    /// Enable dynamic thinking (model decides how much to think).
    /// Equivalent to setting budget to -1.
    pub fn dynamic(mut self) -> Self {
        self.thinking_budget = Some(-1);
        self
    }

    /// Enable thought summaries in the response.
    /// Response parts will have `thought: true` for thought summaries.
    pub fn include_thoughts(mut self) -> Self {
        self.include_thoughts = Some(true);
        self
    }

    /// Disable thought summaries in the response.
    pub fn exclude_thoughts(mut self) -> Self {
        self.include_thoughts = Some(false);
        self
    }
}

/// Image generation configuration (Gemini 3 Pro Image).
/// Supported aspect ratios: "1:1", "2:3", "3:2", "3:4", "4:3", "4:5", "5:4", "9:16", "16:9", "21:9"
/// Supported image sizes: "1K", "2K", "4K" (must be uppercase)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {
    /// Aspect ratio (e.g., "16:9", "1:1", "4:3").
    /// Supported: "1:1", "2:3", "3:2", "3:4", "4:3", "4:5", "5:4", "9:16", "16:9", "21:9"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Image size - must be uppercase: "1K", "2K", or "4K".
    /// Default is "1K". Higher resolutions available with Gemini 3 Pro Image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<String>,
}

impl ImageConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect_ratio(mut self, ratio: impl Into<String>) -> Self {
        self.aspect_ratio = Some(ratio.into());
        self
    }

    pub fn image_size(mut self, size: impl Into<String>) -> Self {
        self.image_size = Some(size.into());
        self
    }

    /// 16:9 aspect ratio for widescreen.
    pub fn widescreen() -> Self {
        Self::new().aspect_ratio("16:9")
    }

    /// 1:1 square aspect ratio.
    pub fn square() -> Self {
        Self::new().aspect_ratio("1:1")
    }

    /// 9:16 portrait aspect ratio (vertical video/stories).
    pub fn portrait() -> Self {
        Self::new().aspect_ratio("9:16")
    }

    /// 21:9 ultrawide cinematic aspect ratio.
    pub fn ultrawide() -> Self {
        Self::new().aspect_ratio("21:9")
    }

    /// 1K resolution (default).
    pub fn resolution_1k() -> Self {
        Self::new().image_size("1K")
    }

    /// 2K resolution.
    pub fn resolution_2k() -> Self {
        Self::new().image_size("2K")
    }

    /// 4K resolution (highest quality, Gemini 3 Pro Image only).
    pub fn resolution_4k() -> Self {
        Self::new().image_size("4K")
    }

    /// Widescreen at 2K resolution.
    pub fn widescreen_2k() -> Self {
        Self::new().aspect_ratio("16:9").image_size("2K")
    }

    /// Square at 2K resolution.
    pub fn square_2k() -> Self {
        Self::new().aspect_ratio("1:1").image_size("2K")
    }
}

// ============================================================================
// Text-to-Speech (TTS) Configuration
// ============================================================================

/// Speech configuration for TTS models (gemini-2.5-flash-preview-tts, gemini-2.5-pro-preview-tts).
/// Use with response_modalities: ["AUDIO"].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeechConfig {
    /// Voice configuration for single-speaker TTS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_config: Option<VoiceConfig>,
    /// Multi-speaker voice configuration (up to 2 speakers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_speaker_voice_config: Option<MultiSpeakerVoiceConfig>,
}

impl SpeechConfig {
    /// Create single-speaker TTS configuration with a prebuilt voice.
    pub fn single_speaker(voice_name: impl Into<String>) -> Self {
        Self {
            voice_config: Some(VoiceConfig::prebuilt(voice_name)),
            multi_speaker_voice_config: None,
        }
    }

    /// Create multi-speaker TTS configuration.
    pub fn multi_speaker(speakers: Vec<SpeakerVoiceConfig>) -> Self {
        Self {
            voice_config: None,
            multi_speaker_voice_config: Some(MultiSpeakerVoiceConfig {
                speaker_voice_configs: speakers,
            }),
        }
    }
}

/// Voice configuration for TTS.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoiceConfig {
    /// Prebuilt voice configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuilt_voice_config: Option<PrebuiltVoiceConfig>,
}

impl VoiceConfig {
    /// Create a prebuilt voice configuration.
    pub fn prebuilt(voice_name: impl Into<String>) -> Self {
        Self {
            prebuilt_voice_config: Some(PrebuiltVoiceConfig {
                voice_name: voice_name.into(),
            }),
        }
    }
}

/// Prebuilt voice configuration specifying a voice name.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrebuiltVoiceConfig {
    /// Voice name. Available voices:
    /// Zephyr (Bright), Puck (Upbeat), Charon (Informative), Kore (Firm),
    /// Fenrir (Excitable), Leda (Youthful), Orus (Firm), Aoede (Breezy),
    /// Callirrhoe (Easy-going), Autonoe (Bright), Enceladus (Breathy),
    /// Iapetus (Clear), Umbriel (Easy-going), Algieba (Smooth), Despina (Smooth),
    /// Erinome (Clear), Algenib (Gravelly), Rasalgethi (Informative),
    /// Laomedeia (Upbeat), Achernar (Soft), Alnilam (Firm), Schedar (Even),
    /// Gacrux (Mature), Pulcherrima (Forward), Achird (Friendly),
    /// Zubenelgenubi (Casual), Vindemiatrix (Gentle), Sadachbia (Lively),
    /// Sadaltager (Knowledgeable), Sulafat (Warm)
    pub voice_name: String,
}

/// Multi-speaker voice configuration for TTS (up to 2 speakers).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiSpeakerVoiceConfig {
    /// Configuration for each speaker.
    pub speaker_voice_configs: Vec<SpeakerVoiceConfig>,
}

/// Configuration for a single speaker in multi-speaker TTS.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerVoiceConfig {
    /// Speaker name (must match the name used in the prompt).
    pub speaker: String,
    /// Voice configuration for this speaker.
    pub voice_config: VoiceConfig,
}

impl SpeakerVoiceConfig {
    /// Create a speaker voice configuration.
    pub fn new(speaker: impl Into<String>, voice_name: impl Into<String>) -> Self {
        Self {
            speaker: speaker.into(),
            voice_config: VoiceConfig::prebuilt(voice_name),
        }
    }
}

/// Prebuilt TTS voice names.
pub mod tts_voices {
    pub const ZEPHYR: &str = "Zephyr"; // Bright
    pub const PUCK: &str = "Puck"; // Upbeat
    pub const CHARON: &str = "Charon"; // Informative
    pub const KORE: &str = "Kore"; // Firm
    pub const FENRIR: &str = "Fenrir"; // Excitable
    pub const LEDA: &str = "Leda"; // Youthful
    pub const ORUS: &str = "Orus"; // Firm
    pub const AOEDE: &str = "Aoede"; // Breezy
    pub const CALLIRRHOE: &str = "Callirrhoe"; // Easy-going
    pub const AUTONOE: &str = "Autonoe"; // Bright
    pub const ENCELADUS: &str = "Enceladus"; // Breathy
    pub const IAPETUS: &str = "Iapetus"; // Clear
    pub const UMBRIEL: &str = "Umbriel"; // Easy-going
    pub const ALGIEBA: &str = "Algieba"; // Smooth
    pub const DESPINA: &str = "Despina"; // Smooth
    pub const ERINOME: &str = "Erinome"; // Clear
    pub const ALGENIB: &str = "Algenib"; // Gravelly
    pub const RASALGETHI: &str = "Rasalgethi"; // Informative
    pub const LAOMEDEIA: &str = "Laomedeia"; // Upbeat
    pub const ACHERNAR: &str = "Achernar"; // Soft
    pub const ALNILAM: &str = "Alnilam"; // Firm
    pub const SCHEDAR: &str = "Schedar"; // Even
    pub const GACRUX: &str = "Gacrux"; // Mature
    pub const PULCHERRIMA: &str = "Pulcherrima"; // Forward
    pub const ACHIRD: &str = "Achird"; // Friendly
    pub const ZUBENELGENUBI: &str = "Zubenelgenubi"; // Casual
    pub const VINDEMIATRIX: &str = "Vindemiatrix"; // Gentle
    pub const SADACHBIA: &str = "Sadachbia"; // Lively
    pub const SADALTAGER: &str = "Sadaltager"; // Knowledgeable
    pub const SULAFAT: &str = "Sulafat"; // Warm
}

// ============================================================================
// Built-in Tools (Gemini 3)
// ============================================================================

/// Google Search tool for grounding responses with web data.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearchTool {}

/// URL Context tool for fetching and understanding web pages.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UrlContextTool {}

/// Code Execution tool for running code.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeExecutionTool {}

/// A tool that can be used by the model.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolConfig {
    /// Function declarations for custom tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
    /// Google Search grounding tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search: Option<GoogleSearchTool>,
    /// URL Context tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_context: Option<UrlContextTool>,
    /// Code Execution tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_execution: Option<CodeExecutionTool>,
}

impl ToolConfig {
    /// Create a tool with function declarations.
    pub fn functions(declarations: Vec<FunctionDeclaration>) -> Self {
        Self {
            function_declarations: Some(declarations),
            google_search: None,
            url_context: None,
            code_execution: None,
        }
    }

    /// Create a Google Search tool.
    pub fn google_search() -> Self {
        Self {
            function_declarations: None,
            google_search: Some(GoogleSearchTool {}),
            url_context: None,
            code_execution: None,
        }
    }

    /// Create a URL Context tool.
    pub fn url_context() -> Self {
        Self {
            function_declarations: None,
            google_search: None,
            url_context: Some(UrlContextTool {}),
            code_execution: None,
        }
    }

    /// Create a Code Execution tool.
    pub fn code_execution() -> Self {
        Self {
            function_declarations: None,
            google_search: None,
            url_context: None,
            code_execution: Some(CodeExecutionTool {}),
        }
    }
}

// ============================================================================
// Function Calling Types
// ============================================================================

/// A function call made by the model.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    /// Name of the function to call.
    pub name: String,
    /// Arguments to pass to the function.
    pub args: serde_json::Value,
}

/// Response from a function call.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponse {
    /// Name of the function that was called.
    pub name: String,
    /// Response from the function.
    pub response: serde_json::Value,
}

/// Declaration of a function that can be called.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {
    /// Name of the function.
    pub name: String,
    /// Description of what the function does.
    pub description: String,
    /// Parameters schema (JSON Schema format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// Tool containing function declarations.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    /// Function declarations available to the model.
    pub function_declarations: Vec<FunctionDeclaration>,
}

impl Tool {
    /// Create a new tool with function declarations.
    pub fn new(declarations: Vec<FunctionDeclaration>) -> Self {
        Self {
            function_declarations: declarations,
        }
    }
}

// ============================================================================
// Function Calling Configuration
// ============================================================================

/// Mode for function calling behavior.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FunctionCallingMode {
    /// Model decides whether to call a function or respond with text (default).
    Auto,
    /// Model is constrained to always call a function.
    Any,
    /// Model is prohibited from calling functions.
    None,
    /// Model predicts function calls or text with schema validation (Preview).
    Validated,
}

/// Configuration for function calling behavior.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig {
    /// The mode for function calling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingMode>,
    /// List of function names the model is allowed to call.
    /// Only used when mode is ANY or VALIDATED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}

impl FunctionCallingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set mode to AUTO (model decides).
    pub fn auto() -> Self {
        Self {
            mode: Some(FunctionCallingMode::Auto),
            allowed_function_names: None,
        }
    }

    /// Set mode to ANY (always call a function).
    pub fn any() -> Self {
        Self {
            mode: Some(FunctionCallingMode::Any),
            allowed_function_names: None,
        }
    }

    /// Set mode to ANY with specific allowed functions.
    pub fn any_of(function_names: Vec<&str>) -> Self {
        Self {
            mode: Some(FunctionCallingMode::Any),
            allowed_function_names: Some(
                function_names.into_iter().map(|s| s.to_string()).collect(),
            ),
        }
    }

    /// Set mode to NONE (disable function calling).
    pub fn none() -> Self {
        Self {
            mode: Some(FunctionCallingMode::None),
            allowed_function_names: None,
        }
    }

    /// Set mode to VALIDATED (Preview - schema validation).
    pub fn validated() -> Self {
        Self {
            mode: Some(FunctionCallingMode::Validated),
            allowed_function_names: None,
        }
    }

    /// Set mode to VALIDATED with specific allowed functions.
    pub fn validated_of(function_names: Vec<&str>) -> Self {
        Self {
            mode: Some(FunctionCallingMode::Validated),
            allowed_function_names: Some(
                function_names.into_iter().map(|s| s.to_string()).collect(),
            ),
        }
    }
}

/// Tool configuration settings (controls function calling behavior).
/// This is the `toolConfig` field in the API request.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToolSettings {
    /// Configuration for function calling behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

impl ToolSettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with function calling configuration.
    pub fn with_function_calling(config: FunctionCallingConfig) -> Self {
        Self {
            function_calling_config: Some(config),
        }
    }

    /// Force the model to call any function.
    pub fn force_function_call() -> Self {
        Self::with_function_calling(FunctionCallingConfig::any())
    }

    /// Force the model to call specific functions only.
    pub fn force_function_call_of(function_names: Vec<&str>) -> Self {
        Self::with_function_calling(FunctionCallingConfig::any_of(function_names))
    }

    /// Disable function calling.
    pub fn disable_function_call() -> Self {
        Self::with_function_calling(FunctionCallingConfig::none())
    }
}

// ============================================================================
// Generation Configuration
// ============================================================================

/// Response modality type for controlling output format.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseModality {
    /// Text output.
    Text,
    /// Image output (for image generation models).
    Image,
    /// Audio output (for TTS models).
    Audio,
}

/// Configuration for content generation.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    /// Response modalities to enable (e.g., ["TEXT", "IMAGE"] for image generation).
    /// Required for image generation with gemini-2.5-flash-image or gemini-3-pro-image-preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modalities: Option<Vec<String>>,
    /// Temperature for sampling (0.0-2.0).
    /// For Gemini 3, keep at default 1.0 for best results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Top-k sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
    /// Stop sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// Number of candidates to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<i32>,
    /// Response MIME type (e.g., "application/json").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,
    /// Response JSON schema for structured output.
    /// Must be a valid JSON Schema. Use with response_mime_type = "application/json".
    #[serde(skip_serializing_if = "Option::is_none", rename = "responseJsonSchema")]
    pub response_schema: Option<serde_json::Value>,
    /// Thinking configuration for 2.5 and 3.x models.
    /// Use thinkingBudget=0 to disable thinking in 2.5 Flash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
    /// Default media resolution for all media in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resolution: Option<MediaResolution>,
    /// Image generation configuration (for image models).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    /// Speech configuration for TTS (Text-to-Speech) output.
    /// When set, the model outputs audio in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_config: Option<SpeechConfig>,
}

impl GenerationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn max_output_tokens(mut self, max: i32) -> Self {
        self.max_output_tokens = Some(max);
        self
    }

    pub fn top_p(mut self, p: f32) -> Self {
        self.top_p = Some(p);
        self
    }

    pub fn top_k(mut self, k: i32) -> Self {
        self.top_k = Some(k);
        self
    }

    pub fn stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(sequences);
        self
    }

    pub fn response_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.response_mime_type = Some(mime_type.into());
        self
    }

    pub fn response_schema(mut self, schema: serde_json::Value) -> Self {
        self.response_schema = Some(schema);
        self
    }

    /// Set thinking configuration.
    pub fn thinking_config(mut self, config: ThinkingConfig) -> Self {
        self.thinking_config = Some(config);
        self
    }

    /// Set thinking level (Gemini 3).
    /// - `Low`: Minimizes latency and cost
    /// - `Medium`: Balanced (coming soon)  
    /// - `High`: Maximizes reasoning depth (default for Gemini 3 Pro)
    pub fn thinking_level(mut self, level: ThinkingLevel) -> Self {
        let config = self
            .thinking_config
            .get_or_insert_with(ThinkingConfig::default);
        config.thinking_level = Some(level);
        self
    }

    /// Set thinking budget in tokens.
    /// Set to 0 to disable thinking (useful for Gemini 2.5 Flash).
    pub fn thinking_budget(mut self, tokens: i32) -> Self {
        let config = self
            .thinking_config
            .get_or_insert_with(ThinkingConfig::default);
        config.thinking_budget = Some(tokens);
        self
    }

    /// Disable thinking entirely (sets budget to 0).
    /// Useful for faster responses with Gemini 2.5 Flash.
    pub fn disable_thinking(self) -> Self {
        self.thinking_budget(0)
    }

    /// Enable dynamic thinking (model decides based on task complexity).
    /// Equivalent to setting budget to -1.
    pub fn dynamic_thinking(mut self) -> Self {
        let config = self
            .thinking_config
            .get_or_insert_with(ThinkingConfig::default);
        config.thinking_budget = Some(-1);
        self
    }

    /// Include thought summaries in the response.
    /// Response parts will have `thought: true` for thought summaries.
    /// Use `part.is_thought()` to check if a part is a thought summary.
    pub fn include_thoughts(mut self) -> Self {
        let config = self
            .thinking_config
            .get_or_insert_with(ThinkingConfig::default);
        config.include_thoughts = Some(true);
        self
    }

    /// Set default media resolution for images/video.
    pub fn media_resolution(mut self, resolution: MediaResolution) -> Self {
        self.media_resolution = Some(resolution);
        self
    }

    /// Set image generation configuration.
    pub fn image_config(mut self, config: ImageConfig) -> Self {
        self.image_config = Some(config);
        self
    }

    /// Configure for JSON structured output.
    /// Sets response_mime_type to "application/json".
    pub fn json_output(mut self) -> Self {
        self.response_mime_type = Some("application/json".to_string());
        self
    }

    /// Configure for structured JSON output with a schema.
    /// Sets response_mime_type to "application/json" and provides the schema.
    ///
    /// # Example
    /// ```
    /// use gemini_client::GenerationConfig;
    /// use serde_json::json;
    ///
    /// let config = GenerationConfig::new().structured_output(json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "name": { "type": "string" },
    ///         "age": { "type": "integer" }
    ///     },
    ///     "required": ["name", "age"]
    /// }));
    /// ```
    pub fn structured_output(mut self, schema: serde_json::Value) -> Self {
        self.response_mime_type = Some("application/json".to_string());
        self.response_schema = Some(schema);
        self
    }

    /// Set response modalities (e.g., ["TEXT", "IMAGE"] for image generation).
    pub fn response_modalities(mut self, modalities: Vec<&str>) -> Self {
        self.response_modalities = Some(modalities.into_iter().map(|s| s.to_string()).collect());
        self
    }

    /// Enable text and image output (for image generation models).
    /// Required for gemini-2.5-flash-image and gemini-3-pro-image-preview.
    pub fn with_image_output(mut self) -> Self {
        self.response_modalities = Some(vec!["TEXT".to_string(), "IMAGE".to_string()]);
        self
    }

    /// Enable text only output (default).
    pub fn text_only(mut self) -> Self {
        self.response_modalities = Some(vec!["TEXT".to_string()]);
        self
    }

    /// Set speech configuration for TTS output.
    pub fn speech_config(mut self, config: SpeechConfig) -> Self {
        self.speech_config = Some(config);
        self
    }

    /// Enable audio output with the specified voice.
    /// Sets response_modalities to ["AUDIO"] and configures the voice.
    pub fn with_audio_output(mut self, voice_name: &str) -> Self {
        self.response_modalities = Some(vec!["AUDIO".to_string()]);
        self.speech_config = Some(SpeechConfig::single_speaker(voice_name));
        self
    }

    /// Enable text and audio output with the specified voice.
    /// Sets response_modalities to ["TEXT", "AUDIO"] for getting both.
    pub fn with_text_and_audio(mut self, voice_name: &str) -> Self {
        self.response_modalities = Some(vec!["TEXT".to_string(), "AUDIO".to_string()]);
        self.speech_config = Some(SpeechConfig::single_speaker(voice_name));
        self
    }
}

// ============================================================================
// Safety Settings
// ============================================================================

/// Safety category for content filtering.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    HarmCategoryUnspecified,
    HarmCategoryHateSpeech,
    HarmCategorySexuallyExplicit,
    HarmCategoryDangerousContent,
    HarmCategoryHarassment,
    HarmCategoryCivicIntegrity,
}

/// Threshold for blocking harmful content.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    HarmBlockThresholdUnspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
    Off,
}

/// Safety setting for a specific category.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafetySetting {
    /// The harm category.
    pub category: HarmCategory,
    /// The threshold for blocking.
    pub threshold: HarmBlockThreshold,
}

/// Safety rating returned in responses.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    /// The harm category.
    pub category: HarmCategory,
    /// The probability of harm.
    pub probability: String,
    /// Whether the content was blocked.
    #[serde(default)]
    pub blocked: bool,
}

// ============================================================================
// Response Types
// ============================================================================

/// Usage metadata for a generation request.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    /// Number of tokens in the prompt.
    pub prompt_token_count: u32,
    /// Number of tokens in the response.
    #[serde(default)]
    pub candidates_token_count: u32,
    /// Total tokens used.
    pub total_token_count: u32,
    /// Tokens used for thinking/reasoning.
    #[serde(default)]
    pub thoughts_token_count: Option<u32>,
    /// Number of tokens that were cache hits (implicit or explicit caching).
    /// Available when using Gemini 2.5 models (implicit) or explicit cached content.
    #[serde(default)]
    pub cached_content_token_count: Option<u32>,
}

impl UsageMetadata {
    /// Get the cache hit ratio (0.0 to 1.0).
    /// Returns None if no caching was used.
    pub fn cache_hit_ratio(&self) -> Option<f64> {
        let cached = self.cached_content_token_count?;
        if self.prompt_token_count == 0 {
            return None;
        }
        Some(cached as f64 / self.prompt_token_count as f64)
    }

    /// Check if any tokens were cached.
    pub fn has_cache_hit(&self) -> bool {
        self.cached_content_token_count.unwrap_or(0) > 0
    }
}

/// Reason why generation stopped.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    FinishReasonUnspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Language,
    Other,
    Blocklist,
    ProhibitedContent,
    Spii,
    MalformedFunctionCall,
}

/// A generation candidate.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    /// The generated content.
    #[serde(default)]
    pub content: Option<Content>,
    /// Why generation stopped.
    #[serde(default)]
    pub finish_reason: Option<FinishReason>,
    /// Safety ratings for the content.
    #[serde(default)]
    pub safety_ratings: Vec<SafetyRating>,
    /// Index of this candidate.
    #[serde(default)]
    pub index: u32,
    /// Grounding metadata from search or URL context.
    #[serde(default)]
    pub grounding_metadata: Option<GroundingMetadata>,
}

impl Candidate {
    /// Get the text from this candidate.
    pub fn text(&self) -> Option<&str> {
        self.content
            .as_ref()?
            .parts
            .first()
            .and_then(|p| p.text.as_deref())
    }
}

// ============================================================================
// Grounding Types
// ============================================================================

/// Metadata about grounding from external sources.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroundingMetadata {
    /// Search entry point (if using Google Search).
    #[serde(default)]
    pub search_entry_point: Option<SearchEntryPoint>,
    /// Grounding chunks (source documents).
    #[serde(default)]
    pub grounding_chunks: Option<Vec<GroundingChunk>>,
    /// Grounding supports (citations).
    #[serde(default)]
    pub grounding_supports: Option<Vec<GroundingSupport>>,
    /// Web search queries used.
    #[serde(default)]
    pub web_search_queries: Option<Vec<String>>,
}

/// Entry point for search results.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchEntryPoint {
    /// Rendered HTML content.
    #[serde(default)]
    pub rendered_content: Option<String>,
    /// SDK blob for search entry.
    #[serde(default)]
    pub sdk_blob: Option<String>,
}

/// A chunk of grounding content.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroundingChunk {
    /// Web source.
    #[serde(default)]
    pub web: Option<GroundingChunkWeb>,
}

/// Web source for grounding.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroundingChunkWeb {
    /// URI of the web source.
    pub uri: String,
    /// Title of the web source.
    #[serde(default)]
    pub title: Option<String>,
}

/// Support (citation) for grounded content.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroundingSupport {
    /// Segment of text being supported.
    #[serde(default)]
    pub segment: Option<GroundingSegment>,
    /// Indices into grounding_chunks.
    #[serde(default)]
    pub grounding_chunk_indices: Vec<u32>,
    /// Confidence scores.
    #[serde(default)]
    pub confidence_scores: Vec<f32>,
}

/// A segment of text that is grounded.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroundingSegment {
    /// Start index in the text.
    #[serde(default)]
    pub start_index: Option<u32>,
    /// End index in the text.
    #[serde(default)]
    pub end_index: Option<u32>,
    /// The text content.
    #[serde(default)]
    pub text: Option<String>,
}

// ============================================================================
// Model Types
// ============================================================================

/// Information about a Gemini model.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// Resource name of the model.
    pub name: String,
    /// Base model ID.
    #[serde(default)]
    pub base_model_id: Option<String>,
    /// Model version.
    pub version: String,
    /// Display name.
    pub display_name: String,
    /// Description of the model.
    pub description: String,
    /// Maximum input tokens.
    #[serde(default)]
    pub input_token_limit: u32,
    /// Maximum output tokens.
    #[serde(default)]
    pub output_token_limit: u32,
    /// Supported generation methods.
    #[serde(default)]
    pub supported_generation_methods: Vec<String>,
    /// Default temperature.
    #[serde(default)]
    pub temperature: Option<f32>,
    /// Maximum temperature.
    #[serde(default)]
    pub max_temperature: Option<f32>,
    /// Default top-p.
    #[serde(default)]
    pub top_p: Option<f32>,
    /// Default top-k.
    #[serde(default)]
    pub top_k: Option<i32>,
}

impl Model {
    /// Get the short model ID (without "models/" prefix).
    pub fn id(&self) -> &str {
        self.name.strip_prefix("models/").unwrap_or(&self.name)
    }
}

// ============================================================================
// Vision / Object Detection / Segmentation Types
// ============================================================================

/// A 2D bounding box with coordinates normalized to [0, 1000].
/// Format: [ymin, xmin, ymax, xmax]
///
/// To convert to pixel coordinates:
/// ```
/// let abs_x1 = (box_2d[1] as f32 / 1000.0 * image_width as f32) as i32;
/// let abs_y1 = (box_2d[0] as f32 / 1000.0 * image_height as f32) as i32;
/// let abs_x2 = (box_2d[3] as f32 / 1000.0 * image_width as f32) as i32;
/// let abs_y2 = (box_2d[2] as f32 / 1000.0 * image_height as f32) as i32;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoundingBox2D {
    /// Bounding box coordinates: [ymin, xmin, ymax, xmax] normalized to 0-1000.
    pub box_2d: [i32; 4],
    /// Label identifying the detected object.
    pub label: String,
}

impl BoundingBox2D {
    /// Convert normalized coordinates to absolute pixel coordinates.
    /// Returns (x1, y1, x2, y2) in pixels.
    pub fn to_pixels(&self, image_width: u32, image_height: u32) -> (i32, i32, i32, i32) {
        let y1 = (self.box_2d[0] as f32 / 1000.0 * image_height as f32) as i32;
        let x1 = (self.box_2d[1] as f32 / 1000.0 * image_width as f32) as i32;
        let y2 = (self.box_2d[2] as f32 / 1000.0 * image_height as f32) as i32;
        let x2 = (self.box_2d[3] as f32 / 1000.0 * image_width as f32) as i32;
        (x1, y1, x2, y2)
    }

    /// Get width in pixels.
    pub fn width(&self, image_width: u32) -> i32 {
        let (x1, _, x2, _) = self.to_pixels(image_width, 1);
        x2 - x1
    }

    /// Get height in pixels.
    pub fn height(&self, image_height: u32) -> i32 {
        let (_, y1, _, y2) = self.to_pixels(1, image_height);
        y2 - y1
    }
}

/// A segmentation mask result from Gemini 2.5+.
/// Contains a bounding box, label, and base64-encoded PNG mask.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SegmentationMask {
    /// Bounding box coordinates: [ymin, xmin, ymax, xmax] normalized to 0-1000.
    pub box_2d: [i32; 4],
    /// Label identifying the segmented object.
    pub label: String,
    /// Base64-encoded PNG mask (probability map 0-255).
    /// Prefixed with "data:image/png;base64," when returned from API.
    /// The mask needs to be resized to match the bounding box dimensions,
    /// then binarized at threshold (127 for midpoint).
    pub mask: String,
}

impl SegmentationMask {
    /// Convert normalized coordinates to absolute pixel coordinates.
    /// Returns (x1, y1, x2, y2) in pixels.
    pub fn to_pixels(&self, image_width: u32, image_height: u32) -> (i32, i32, i32, i32) {
        let y1 = (self.box_2d[0] as f32 / 1000.0 * image_height as f32) as i32;
        let x1 = (self.box_2d[1] as f32 / 1000.0 * image_width as f32) as i32;
        let y2 = (self.box_2d[2] as f32 / 1000.0 * image_height as f32) as i32;
        let x2 = (self.box_2d[3] as f32 / 1000.0 * image_width as f32) as i32;
        (x1, y1, x2, y2)
    }

    /// Get the raw base64 mask data (without the data URI prefix).
    pub fn mask_base64(&self) -> &str {
        self.mask
            .strip_prefix("data:image/png;base64,")
            .unwrap_or(&self.mask)
    }
}

/// Helper prompts for common vision tasks.
pub mod vision_prompts {
    /// Prompt for object detection with bounding boxes.
    pub const OBJECT_DETECTION: &str =
        "Detect all prominent items in the image. Return a JSON array where each object has \
         'box_2d' as [ymin, xmin, ymax, xmax] normalized to 0-1000, and 'label' with a descriptive name.";

    /// Prompt for segmentation with masks.
    pub const SEGMENTATION: &str =
        "Give the segmentation masks for all objects. Return a JSON array where each entry has \
         'box_2d' as [ymin, xmin, ymax, xmax] normalized to 0-1000, 'mask' as a base64 PNG, and 'label'.";

    /// Prompt for specific object detection (template).
    pub fn detect_objects(objects: &str) -> String {
        format!(
            "Detect {} in the image. Return a JSON array where each object has \
             'box_2d' as [ymin, xmin, ymax, xmax] normalized to 0-1000, and 'label'.",
            objects
        )
    }

    /// Prompt for segmenting specific objects (template).
    pub fn segment_objects(objects: &str) -> String {
        format!(
            "Give the segmentation masks for {}. Return a JSON array where each entry has \
             'box_2d' as [ymin, xmin, ymax, xmax] normalized to 0-1000, 'mask' as base64 PNG, and 'label'.",
            objects
        )
    }
}

/// Helper prompts for common audio understanding tasks.
/// Audio details:
/// - 1 second of audio = 32 tokens (1 minute = 1,920 tokens)
/// - Max 9.5 hours of audio per prompt
/// - Supports: WAV, MP3, AIFF, AAC, OGG, FLAC
pub mod audio_prompts {
    /// Prompt for generating a transcript of audio speech.
    pub const TRANSCRIBE: &str = "Generate a transcript of the speech.";

    /// Prompt for describing audio content.
    pub const DESCRIBE: &str = "Describe this audio clip.";

    /// Prompt for summarizing audio content.
    pub const SUMMARIZE: &str = "Summarize the main points from this audio.";

    /// Prompt for identifying speakers in audio.
    pub const IDENTIFY_SPEAKERS: &str =
        "Identify the different speakers in this audio and describe their voices.";

    /// Prompt for detecting non-speech sounds.
    pub const DETECT_SOUNDS: &str =
        "Identify all non-speech sounds in this audio (music, ambient noise, sound effects, etc.).";

    /// Prompt for sentiment analysis of speech.
    pub const ANALYZE_SENTIMENT: &str =
        "Analyze the sentiment and emotional tone of the speakers in this audio.";

    /// Generate a transcript for a specific time range.
    /// Timestamps use MM:SS format.
    pub fn transcribe_range(start: &str, end: &str) -> String {
        format!(
            "Provide a transcript of the speech from {} to {}.",
            start, end
        )
    }

    /// Answer a question about the audio content.
    pub fn question(q: &str) -> String {
        format!("Listen to the audio and answer: {}", q)
    }

    /// Transcribe with speaker labels.
    pub const TRANSCRIBE_WITH_SPEAKERS: &str =
        "Generate a transcript of the speech, labeling each speaker (e.g., Speaker 1, Speaker 2).";

    /// Extract key moments or topics with timestamps.
    pub const KEY_MOMENTS: &str =
        "List the key moments or topics discussed in this audio with their timestamps (MM:SS format).";
}

/// Helper functions for building JSON schemas for structured output.
///
/// # Example
/// ```
/// use gemini_client::{GenerationConfig, json_schema};
///
/// // Simple object schema
/// let schema = json_schema::object(
///     &[("name", json_schema::string()), ("age", json_schema::integer())],
///     &["name", "age"]
/// );
///
/// let config = GenerationConfig::new().structured_output(schema);
/// ```
pub mod json_schema {
    use serde_json::{json, Value};

    /// Create a string type schema.
    pub fn string() -> Value {
        json!({"type": "string"})
    }

    /// Create a string type with description.
    pub fn string_with_desc(description: &str) -> Value {
        json!({"type": "string", "description": description})
    }

    /// Create an integer type schema.
    pub fn integer() -> Value {
        json!({"type": "integer"})
    }

    /// Create an integer with min/max bounds.
    pub fn integer_range(min: i64, max: i64) -> Value {
        json!({"type": "integer", "minimum": min, "maximum": max})
    }

    /// Create a number (float) type schema.
    pub fn number() -> Value {
        json!({"type": "number"})
    }

    /// Create a boolean type schema.
    pub fn boolean() -> Value {
        json!({"type": "boolean"})
    }

    /// Create a nullable type (e.g., string or null).
    pub fn nullable(inner_type: &str) -> Value {
        json!({"type": [inner_type, "null"]})
    }

    /// Create an enum (string with fixed values).
    pub fn enum_values(values: &[&str]) -> Value {
        json!({"type": "string", "enum": values})
    }

    /// Create an array of items with a given schema.
    pub fn array(items: Value) -> Value {
        json!({"type": "array", "items": items})
    }

    /// Create an array with min/max items.
    pub fn array_bounded(items: Value, min: usize, max: usize) -> Value {
        json!({"type": "array", "items": items, "minItems": min, "maxItems": max})
    }

    /// Create an object schema from properties.
    ///
    /// # Arguments
    /// * `properties` - Slice of (name, schema) tuples
    /// * `required` - Slice of required property names
    pub fn object(properties: &[(&str, Value)], required: &[&str]) -> Value {
        let props: serde_json::Map<String, Value> = properties
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        json!({
            "type": "object",
            "properties": props,
            "required": required
        })
    }

    /// Create an object that allows additional properties.
    pub fn object_open(properties: &[(&str, Value)], required: &[&str]) -> Value {
        let props: serde_json::Map<String, Value> = properties
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        json!({
            "type": "object",
            "properties": props,
            "required": required,
            "additionalProperties": true
        })
    }

    /// Create an object with explicit property ordering (required for Gemini 2.0).
    /// Gemini 2.0 Flash/Lite requires `propertyOrdering` to define preferred structure.
    ///
    /// # Example
    /// ```
    /// use gemini_client::json_schema;
    ///
    /// let schema = json_schema::object_ordered(
    ///     &[("name", json_schema::string()), ("age", json_schema::integer())],
    ///     &["name", "age"],
    ///     &["name", "age"]  // property ordering
    /// );
    /// ```
    pub fn object_ordered(
        properties: &[(&str, Value)],
        required: &[&str],
        property_ordering: &[&str],
    ) -> Value {
        let props: serde_json::Map<String, Value> = properties
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        json!({
            "type": "object",
            "properties": props,
            "required": required,
            "propertyOrdering": property_ordering
        })
    }

    /// Add property ordering to an existing object schema (for Gemini 2.0 compatibility).
    pub fn with_property_ordering(mut schema: Value, ordering: &[&str]) -> Value {
        if let Some(obj) = schema.as_object_mut() {
            obj.insert("propertyOrdering".to_string(), json!(ordering));
        }
        schema
    }

    /// Create a date-time formatted string.
    pub fn datetime() -> Value {
        json!({"type": "string", "format": "date-time"})
    }

    /// Create a date formatted string.
    pub fn date() -> Value {
        json!({"type": "string", "format": "date"})
    }

    /// Create a time formatted string.
    pub fn time() -> Value {
        json!({"type": "string", "format": "time"})
    }
}

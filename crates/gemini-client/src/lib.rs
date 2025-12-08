//! Gemini API Client for Rust
//!
//! A comprehensive Rust SDK for the Google Gemini Generative Language API.
//!
//! # Example
//!
//! ```no_run
//! use gemini_client::{GeminiClient, GenerateContentRequest, models};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GeminiClient::from_env()?;
//!     
//!     // Use default model
//!     let request = GenerateContentRequest::with_text("Explain AI in a few words");
//!     let response = client.generate_content(request).await?;
//!     
//!     // Or specify a model
//!     let request = GenerateContentRequest::with_text("Complex reasoning task")
//!         .model(models::gemini_2_5_pro::STABLE);
//!     let response = client.generate_content(request).await?;
//!     
//!     if let Some(text) = response.text() {
//!         println!("{}", text);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod types;

// Re-export main types for convenience
pub use client::{
    BatchEmbedContentsRequest,
    BatchEmbedContentsResponse,
    BatchError,
    BatchInputConfig,
    // Batch API (50% cheaper async processing)
    BatchJob,
    BatchJobRequest,
    BatchJobState,
    BatchOutputConfig,
    BatchRequestMetadata,
    CachedContent,
    ClientConfig,
    ContentEmbedding,
    CountTokensRequest,
    CountTokensResponse,
    CreateCachedContentRequest,
    EmbedContentRequest,
    EmbedContentResponse,
    // File upload logging
    FileLogger,
    FileMapping,
    FileMetadata,
    GeminiClient,
    GenerateContentRequest,
    GenerateContentResponse,
    GenerateVideosConfig,
    // Video generation (Veo)
    GenerateVideosRequest,
    GenerateVideosResponse,
    GeneratedVideo,
    InlinedRequest,
    InlinedRequests,
    InlinedResponse,
    InlinedResponses,
    ListBatchJobsResponse,
    ListCachedContentsResponse,
    ListFilesResponse,
    OperationError,
    TaskType,
    UploadFileResponse,
    VideoFile,
    VideoImage,
    VideoInput,
    VideoOperation,
    VideoReferenceImage,
};

pub use error::{GeminiError, Result};

pub use types::{
    // Audio understanding
    audio_prompts,
    // JSON schema helpers for structured output
    json_schema,
    // Thought signatures for multi-turn function calling
    thought_signatures,
    tts_voices,
    vision_prompts,
    Blob,
    // Vision / Object Detection / Segmentation
    BoundingBox2D,
    Candidate,
    // Gemini 3 features
    CodeExecutionTool,
    Content,
    FileData,
    FinishReason,
    FunctionCall,
    // Function calling configuration
    FunctionCallingConfig,
    FunctionCallingMode,
    FunctionDeclaration,
    FunctionResponse,
    GenerationConfig,
    GoogleSearchTool,
    // Grounding types
    GroundingChunk,
    GroundingChunkWeb,
    GroundingMetadata,
    GroundingSegment,
    GroundingSupport,
    HarmBlockThreshold,
    HarmCategory,
    ImageConfig,
    MediaResolution,
    MediaResolutionConfig,
    Model,
    MultiSpeakerVoiceConfig,
    Part,
    PrebuiltVoiceConfig,
    ResponseModality,
    SafetyRating,
    SafetySetting,
    SearchEntryPoint,
    SegmentationMask,
    SpeakerVoiceConfig,
    // Text-to-Speech (TTS)
    SpeechConfig,
    ThinkingConfig,
    ThinkingLevel,
    Tool,
    ToolConfig,
    ToolSettings,
    UrlContextTool,
    UsageMetadata,
    VideoMetadata,
    VoiceConfig,
};

// Re-export models module for easy access
pub use models::defaults as default_models;
pub use models::thinking_budget;

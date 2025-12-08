//! Error types for the Gemini API client.

use thiserror::Error;

/// Errors that can occur when using the Gemini API client.
#[derive(Error, Debug)]
pub enum GeminiError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    ApiError {
        status: u16,
        message: String,
        details: Option<serde_json::Value>,
    },

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Missing API key
    #[error("Missing API key: set GEMINI_API_KEY environment variable")]
    MissingApiKey,

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Stream error
    #[error("Stream error: {0}")]
    StreamError(String),

    /// File operation error
    #[error("File operation error: {0}")]
    FileError(String),
}

/// Result type alias for Gemini operations.
pub type Result<T> = std::result::Result<T, GeminiError>;

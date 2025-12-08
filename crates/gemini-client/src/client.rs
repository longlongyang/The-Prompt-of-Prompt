//! Gemini API client.

use crate::error::{GeminiError, Result};
use crate::models;
use crate::types::*;
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tokio_stream::Stream;

/// Base URL for the Gemini API.
const API_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Default model to use.
const DEFAULT_MODEL: &str = models::defaults::GENERAL;

// ============================================================================
// Embedding Task Types
// ============================================================================

/// Task types for embedding generation.
/// Different task types optimize embeddings for specific use cases.
pub struct TaskType;

impl TaskType {
    /// Embeddings optimized to assess text similarity.
    /// Use cases: Recommendation systems, duplicate detection.
    pub const SEMANTIC_SIMILARITY: &'static str = "SEMANTIC_SIMILARITY";

    /// Embeddings optimized to classify texts according to preset labels.
    /// Use cases: Sentiment analysis, spam detection.
    pub const CLASSIFICATION: &'static str = "CLASSIFICATION";

    /// Embeddings optimized to cluster texts based on their similarities.
    /// Use cases: Document organization, market research, anomaly detection.
    pub const CLUSTERING: &'static str = "CLUSTERING";

    /// Embeddings optimized for document search (indexing).
    /// Use for documents to be retrieved.
    pub const RETRIEVAL_DOCUMENT: &'static str = "RETRIEVAL_DOCUMENT";

    /// Embeddings optimized for general search queries.
    /// Use for queries; use RETRIEVAL_DOCUMENT for documents to be retrieved.
    pub const RETRIEVAL_QUERY: &'static str = "RETRIEVAL_QUERY";

    /// Embeddings optimized for retrieval of code blocks based on natural language queries.
    /// Use for queries; use RETRIEVAL_DOCUMENT for code blocks to be retrieved.
    pub const CODE_RETRIEVAL_QUERY: &'static str = "CODE_RETRIEVAL_QUERY";

    /// Embeddings for questions in a question-answering system.
    /// Optimized for finding documents that answer the question.
    /// Use for questions; use RETRIEVAL_DOCUMENT for documents to be retrieved.
    pub const QUESTION_ANSWERING: &'static str = "QUESTION_ANSWERING";

    /// Embeddings for statements that need to be verified.
    /// Optimized for retrieving documents that contain evidence supporting or refuting the statement.
    /// Use for the target text; use RETRIEVAL_DOCUMENT for documents to be retrieved.
    pub const FACT_VERIFICATION: &'static str = "FACT_VERIFICATION";
}

/// Configuration for the Gemini client.
#[derive(Debug, Clone, Default)]
pub struct ClientConfig {
    /// API key for authentication.
    pub api_key: String,
    /// Base URL for the API.
    pub base_url: String,
    /// Default model to use.
    pub default_model: String,
    /// HTTP/HTTPS proxy URL (e.g., "http://127.0.0.1:7890").
    pub proxy: Option<String>,
}

impl ClientConfig {
    /// Create a new configuration with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: API_BASE_URL.to_string(),
            default_model: DEFAULT_MODEL.to_string(),
            proxy: None,
        }
    }

    /// Create configuration from environment variables.
    /// Reads GEMINI_API_KEY, GEMINI_MODEL, and HTTPS_PROXY/HTTP_PROXY.
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY").map_err(|_| GeminiError::MissingApiKey)?;
        let model = env::var("GEMINI_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());
        let proxy = env::var("HTTPS_PROXY")
            .or_else(|_| env::var("https_proxy"))
            .or_else(|_| env::var("HTTP_PROXY"))
            .or_else(|_| env::var("http_proxy"))
            .ok();
        Ok(Self {
            api_key,
            base_url: API_BASE_URL.to_string(),
            default_model: model,
            proxy,
        })
    }

    /// Set the default model.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }

    /// Set a custom base URL.
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set a proxy URL (e.g., "http://127.0.0.1:7890").
    pub fn with_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
}

/// The main Gemini API client.
#[derive(Debug, Clone)]
pub struct GeminiClient {
    config: ClientConfig,
    http: reqwest::Client,
}

impl GeminiClient {
    /// Create a new client with the given configuration.
    /// If a proxy is configured, it will be used for all requests.
    pub fn new(config: ClientConfig) -> Self {
        let mut builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy {
            if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
                builder = builder.proxy(proxy);
            }
        }

        let http = builder.build().unwrap_or_else(|_| reqwest::Client::new());
        Self { config, http }
    }

    /// Create a client from environment variables.
    pub fn from_env() -> Result<Self> {
        Ok(Self::new(ClientConfig::from_env()?))
    }

    /// Get the default headers for API requests.
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-goog-api-key",
            HeaderValue::from_str(&self.config.api_key).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Build a URL for the given endpoint.
    fn url(&self, endpoint: &str) -> String {
        format!("{}{}", self.config.base_url, endpoint)
    }

    /// Build a model URL for the given model and method.
    fn model_url(&self, model: Option<&str>, method: &str) -> String {
        let model = model.unwrap_or(&self.config.default_model);
        self.url(&format!("/models/{}:{}", model, method))
    }

    /// Handle API error responses.
    async fn handle_error(&self, response: reqwest::Response) -> GeminiError {
        let status = response.status().as_u16();
        match response.json::<serde_json::Value>().await {
            Ok(body) => {
                let message = body["error"]["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string();
                GeminiError::ApiError {
                    status,
                    message,
                    details: Some(body),
                }
            }
            Err(_) => GeminiError::ApiError {
                status,
                message: "Failed to parse error response".to_string(),
                details: None,
            },
        }
    }

    // ========================================================================
    // Models API
    // ========================================================================

    /// List available models.
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let url = self.url("/models");
        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        #[derive(Deserialize)]
        struct ListModelsResponse {
            models: Vec<Model>,
        }

        let resp: ListModelsResponse = response.json().await?;
        Ok(resp.models)
    }

    /// Get information about a specific model.
    pub async fn get_model(&self, model: &str) -> Result<Model> {
        let url = self.url(&format!("/models/{}", model));
        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Generate content using the specified model.
    pub async fn generate_content(
        &self,
        request: GenerateContentRequest,
    ) -> Result<GenerateContentResponse> {
        let model = request.model.as_deref();
        let url = self.model_url(model, "generateContent");

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Generate content with streaming response.
    pub async fn stream_generate_content(
        &self,
        request: GenerateContentRequest,
    ) -> Result<impl Stream<Item = Result<GenerateContentResponse>>> {
        let model = request.model.as_deref();
        let url = self.model_url(model, "streamGenerateContent");
        let url = format!("{}?alt=sse", url);

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        let stream = response.bytes_stream().map(|result| {
            result.map_err(GeminiError::from).and_then(|bytes| {
                let text = String::from_utf8_lossy(&bytes);
                // Parse SSE data lines
                let mut responses = Vec::new();
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data.trim() != "[DONE]" {
                            if let Ok(resp) = serde_json::from_str(data) {
                                responses.push(resp);
                            }
                        }
                    }
                }
                // Return first response from this chunk
                responses
                    .into_iter()
                    .next()
                    .ok_or_else(|| GeminiError::StreamError("No data in chunk".to_string()))
            })
        });

        Ok(stream)
    }

    /// Count tokens for the given content.
    pub async fn count_tokens(&self, request: CountTokensRequest) -> Result<CountTokensResponse> {
        let model = request.model.as_deref();
        let url = self.model_url(model, "countTokens");

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    // ========================================================================
    // Embeddings API
    // ========================================================================

    /// Generate embeddings for content.
    pub async fn embed_content(
        &self,
        request: EmbedContentRequest,
    ) -> Result<EmbedContentResponse> {
        let model = request.model.as_deref().unwrap_or("text-embedding-004");
        let url = self.model_url(Some(model), "embedContent");

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Batch embed multiple contents.
    pub async fn batch_embed_contents(
        &self,
        request: BatchEmbedContentsRequest,
    ) -> Result<BatchEmbedContentsResponse> {
        let model = request.model.as_deref().unwrap_or("text-embedding-004");
        let url = self.model_url(Some(model), "batchEmbedContents");

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    // ========================================================================
    // Files API
    // ========================================================================

    /// Upload a file to the Gemini API.
    ///
    /// Files are stored for 48 hours and can be used with generateContent.
    /// Use the Files API when total request size exceeds 20 MB.
    ///
    /// # Example
    /// ```no_run
    /// use gemini_client::GeminiClient;
    ///
    /// let client = GeminiClient::from_env().unwrap();
    ///
    /// // Upload a file
    /// let file = client.upload_file("path/to/audio.mp3", None).await?;
    /// println!("Uploaded: {} -> {}", "path/to/audio.mp3", file.uri.as_ref().unwrap());
    ///
    /// // Use in generation
    /// let response = client.generate_content(
    ///     gemini_client::GenerateContentRequest::with_file_data(
    ///         file.uri.as_ref().unwrap(),
    ///         &file.mime_type,
    ///     ).add_text("Describe this audio clip")
    /// ).await?;
    /// ```
    pub async fn upload_file(
        &self,
        file_path: impl AsRef<std::path::Path>,
        display_name: Option<&str>,
    ) -> Result<FileMetadata> {
        let path = file_path.as_ref();

        // Read the file
        let file_bytes = tokio::fs::read(path).await.map_err(|e| {
            crate::GeminiError::InvalidConfig(format!("Failed to read file: {}", e))
        })?;

        // Detect MIME type from extension
        let mime_type = mime_from_path(path);
        let display = display_name
            .map(|s| s.to_string())
            .or_else(|| path.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "uploaded_file".to_string());

        self.upload_file_bytes(&file_bytes, &mime_type, &display)
            .await
    }

    /// Upload a file and log the mapping to a FileLogger.
    ///
    /// This is a convenience method that combines upload and logging.
    ///
    /// # Example
    /// ```no_run
    /// use gemini_client::{GeminiClient, FileLogger};
    ///
    /// let client = GeminiClient::from_env().unwrap();
    /// let logger = FileLogger::new("uploads.log").unwrap();
    ///
    /// // Upload and log in one call
    /// let file = client.upload_file_logged("audio.mp3", None, &logger).await?;
    ///
    /// // Later, find the mapping
    /// if let Some(mapping) = logger.find_by_local_path("audio.mp3")? {
    ///     println!("File {} is at {}", mapping.local_path, mapping.remote_uri);
    /// }
    /// ```
    pub async fn upload_file_logged(
        &self,
        file_path: impl AsRef<std::path::Path>,
        display_name: Option<&str>,
        logger: &FileLogger,
    ) -> Result<FileMetadata> {
        let path = file_path.as_ref();
        let path_str = path.to_string_lossy().to_string();

        let file = self.upload_file(path, display_name).await?;
        logger.log(&path_str, &file)?;

        Ok(file)
    }

    /// Upload file bytes directly to the Gemini API.
    ///
    /// # Example
    /// ```no_run
    /// use gemini_client::GeminiClient;
    ///
    /// let client = GeminiClient::from_env().unwrap();
    /// let bytes = std::fs::read("audio.mp3").unwrap();
    /// let file = client.upload_file_bytes(&bytes, "audio/mpeg", "my-audio").await?;
    /// ```
    pub async fn upload_file_bytes(
        &self,
        bytes: &[u8],
        mime_type: &str,
        display_name: &str,
    ) -> Result<FileMetadata> {
        let num_bytes = bytes.len();

        // Step 1: Initiate resumable upload
        let upload_url = format!(
            "{}/upload/v1beta/files?key={}",
            self.config.base_url.trim_end_matches("/v1beta"),
            self.config.api_key
        );

        let init_response = self
            .http
            .post(&upload_url)
            .header("X-Goog-Upload-Protocol", "resumable")
            .header("X-Goog-Upload-Command", "start")
            .header("X-Goog-Upload-Header-Content-Length", num_bytes.to_string())
            .header("X-Goog-Upload-Header-Content-Type", mime_type)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"{{"file": {{"display_name": "{}"}}}}"#,
                display_name
            ))
            .send()
            .await?;

        if !init_response.status().is_success() {
            let status = init_response.status().as_u16();
            let text = init_response.text().await.unwrap_or_default();
            return Err(crate::GeminiError::ApiError {
                status,
                message: format!("Failed to initiate upload: {}", text),
                details: None,
            });
        }

        // Get the upload URL from response headers
        let resume_url = init_response
            .headers()
            .get("x-goog-upload-url")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| crate::GeminiError::ApiError {
                status: 500,
                message: "Missing upload URL in response".to_string(),
                details: None,
            })?
            .to_string();

        // Step 2: Upload the actual bytes
        let upload_response = self
            .http
            .post(&resume_url)
            .header("Content-Length", num_bytes.to_string())
            .header("X-Goog-Upload-Offset", "0")
            .header("X-Goog-Upload-Command", "upload, finalize")
            .body(bytes.to_vec())
            .send()
            .await?;

        if !upload_response.status().is_success() {
            let status = upload_response.status().as_u16();
            let text = upload_response.text().await.unwrap_or_default();
            return Err(crate::GeminiError::ApiError {
                status,
                message: format!("Failed to upload file: {}", text),
                details: None,
            });
        }

        // Parse the response to get file metadata
        let upload_result: UploadFileResponse = upload_response.json().await?;
        Ok(upload_result.file)
    }

    /// List uploaded files.
    pub async fn list_files(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListFilesResponse> {
        let mut url = self.url("/files");
        let mut params = Vec::new();
        if let Some(size) = page_size {
            params.push(format!("pageSize={}", size));
        }
        if let Some(token) = page_token {
            params.push(format!("pageToken={}", token));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Get metadata for a file.
    pub async fn get_file(&self, name: &str) -> Result<FileMetadata> {
        let url = self.url(&format!("/files/{}", name));
        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Delete a file.
    pub async fn delete_file(&self, name: &str) -> Result<()> {
        let url = self.url(&format!("/files/{}", name));
        let response = self
            .http
            .delete(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(())
    }

    // ========================================================================
    // Cached Contents API
    // ========================================================================

    /// Create cached content.
    pub async fn create_cached_content(
        &self,
        request: CreateCachedContentRequest,
    ) -> Result<CachedContent> {
        let url = self.url("/cachedContents");

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// List cached contents.
    pub async fn list_cached_contents(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListCachedContentsResponse> {
        let mut url = self.url("/cachedContents");
        let mut params = Vec::new();
        if let Some(size) = page_size {
            params.push(format!("pageSize={}", size));
        }
        if let Some(token) = page_token {
            params.push(format!("pageToken={}", token));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Get cached content.
    pub async fn get_cached_content(&self, name: &str) -> Result<CachedContent> {
        let url = self.url(&format!("/{}", name));
        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Update cached content (only expiration can be updated).
    pub async fn update_cached_content(
        &self,
        name: &str,
        ttl: Option<&str>,
        expire_time: Option<&str>,
    ) -> Result<CachedContent> {
        let url = self.url(&format!("/{}", name));

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct UpdateRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            ttl: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expire_time: Option<String>,
        }

        let request = UpdateRequest {
            ttl: ttl.map(|s| s.to_string()),
            expire_time: expire_time.map(|s| s.to_string()),
        };

        let response = self
            .http
            .patch(&url)
            .headers(self.headers())
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Delete cached content.
    pub async fn delete_cached_content(&self, name: &str) -> Result<()> {
        let url = self.url(&format!("/{}", name));
        let response = self
            .http
            .delete(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(())
    }

    // ========================================================================
    // Batch API (50% cost reduction for async processing)
    // ========================================================================

    /// Create a batch job with inline requests.
    ///
    /// Batch API processes requests asynchronously at 50% of the standard cost.
    /// Target turnaround is 24 hours but usually much faster.
    ///
    /// # Example
    /// ```no_run
    /// use gemini_client::{GeminiClient, BatchJobRequest, GenerateContentRequest, InlinedRequest, models};
    ///
    /// let client = GeminiClient::from_env().unwrap();
    /// let requests = vec![
    ///     InlinedRequest::with_key(GenerateContentRequest::with_text("Tell me a joke"), "joke"),
    ///     InlinedRequest::new(GenerateContentRequest::with_text("Why is the sky blue?")),
    /// ];
    ///
    /// let job = client.create_batch_job(
    ///     BatchJobRequest::inline(models::gemini_2_5_flash::STABLE, requests)
    ///         .display_name("my-batch-job")
    /// ).await?;
    ///
    /// println!("Created batch job: {}", job.name);
    /// ```
    pub async fn create_batch_job(&self, request: BatchJobRequest) -> Result<BatchJob> {
        let url = self.model_url(Some(&request.model), "batchGenerateContent");

        // Wrap the request in a "batch" object as required by the API
        let wrapped_request = serde_json::json!({
            "batch": &request
        });

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&wrapped_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        // Parse the JSON and extract the batch job from metadata
        let json: serde_json::Value = response.json().await?;

        // The response wraps the batch job in a "metadata" field
        let batch_json = json.get("metadata").cloned().unwrap_or(json);

        serde_json::from_value(batch_json).map_err(|e| crate::GeminiError::ApiError {
            status: 500,
            message: format!("Failed to parse batch response: {}", e),
            details: None,
        })
    }

    /// Get a batch job status.
    pub async fn get_batch_job(&self, name: &str) -> Result<BatchJob> {
        let url = self.url(&format!("/{}", name));
        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        let json: serde_json::Value = response.json().await?;

        // The response wraps the batch job in a "metadata" field
        let batch_json = json.get("metadata").cloned().unwrap_or(json);

        serde_json::from_value(batch_json).map_err(|e| crate::GeminiError::ApiError {
            status: 500,
            message: format!("Failed to parse batch job: {}", e),
            details: None,
        })
    }

    /// List all batch jobs.
    pub async fn list_batch_jobs(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListBatchJobsResponse> {
        let mut url = self.url("/batches");
        let mut params = Vec::new();
        if let Some(size) = page_size {
            params.push(format!("pageSize={}", size));
        }
        if let Some(token) = page_token {
            params.push(format!("pageToken={}", token));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Cancel a batch job.
    pub async fn cancel_batch_job(&self, name: &str) -> Result<BatchJob> {
        let url = self.url(&format!("/{}:cancel", name));
        let response = self.http.post(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Delete a batch job.
    pub async fn delete_batch_job(&self, name: &str) -> Result<()> {
        let url = self.url(&format!("/{}", name));
        let response = self
            .http
            .delete(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(())
    }

    /// Wait for a batch job to complete, polling at the specified interval.
    ///
    /// Returns the completed job, or an error if the job failed or was cancelled.
    ///
    /// # Example
    /// ```no_run
    /// use gemini_client::{GeminiClient, BatchJobRequest, InlinedRequest, GenerateContentRequest, models};
    /// use std::time::Duration;
    ///
    /// let client = GeminiClient::from_env().unwrap();
    /// let requests = vec![
    ///     InlinedRequest::with_key(GenerateContentRequest::with_text("Hello"), "greeting"),
    /// ];
    ///
    /// let job = client.create_batch_job(
    ///     BatchJobRequest::inline(models::gemini_2_5_flash::STABLE, requests)
    /// ).await?;
    ///
    /// // Wait for completion, polling every 30 seconds
    /// let completed_job = client.wait_for_batch_job(&job.name, Duration::from_secs(30)).await?;
    ///
    /// if let Some(responses) = completed_job.responses() {
    ///     for response in responses {
    ///         println!("Key: {:?}, Response: {:?}", response.key, response.response.as_ref().and_then(|r| r.text()));
    ///     }
    /// }
    /// ```
    pub async fn wait_for_batch_job(
        &self,
        name: &str,
        poll_interval: std::time::Duration,
    ) -> Result<BatchJob> {
        loop {
            let job = self.get_batch_job(name).await?;

            if job.is_complete() {
                if job.failed() {
                    let error_msg = job
                        .error
                        .as_ref()
                        .and_then(|e| e.message.as_deref())
                        .unwrap_or("Unknown error");
                    return Err(crate::GeminiError::ApiError {
                        status: 500,
                        message: format!("Batch job failed: {}", error_msg),
                        details: None,
                    });
                }
                if job.cancelled() {
                    return Err(crate::GeminiError::ApiError {
                        status: 499,
                        message: "Batch job was cancelled".to_string(),
                        details: None,
                    });
                }
                return Ok(job);
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    // ========================================================================
    // Video Generation API (Veo)
    // ========================================================================

    /// Start video generation (returns a long-running operation).
    ///
    /// Videos take time to generate. Use `get_video_operation` to poll for completion.
    ///
    /// # Example
    /// ```no_run
    /// use gemini_client::{GeminiClient, GenerateVideosRequest, models};
    ///
    /// let client = GeminiClient::from_env().unwrap();
    /// let request = GenerateVideosRequest::new("A cinematic shot of a lion")
    ///     .model(models::veo_3_1::PREVIEW)
    ///     .aspect_ratio("16:9")
    ///     .duration(8);
    ///
    /// let operation = client.generate_videos(request).await?;
    ///
    /// // Poll until done
    /// loop {
    ///     let op = client.get_video_operation(&operation.name).await?;
    ///     if op.done {
    ///         let video_uri = op.response.unwrap().generated_videos[0].video.uri;
    ///         break;
    ///     }
    ///     tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    /// }
    /// ```
    pub async fn generate_videos(&self, request: GenerateVideosRequest) -> Result<VideoOperation> {
        let url = format!(
            "{}/models/{}:predictLongRunning?key={}",
            self.config.base_url, request.model, self.config.api_key
        );

        // Build the request body in the format expected by the API
        #[derive(Serialize)]
        struct PredictRequest {
            instances: Vec<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parameters: Option<serde_json::Value>,
        }

        let mut instance = serde_json::json!({
            "prompt": request.prompt
        });

        if let Some(np) = &request.negative_prompt {
            instance["negativePrompt"] = serde_json::json!(np);
        }
        if let Some(img) = &request.image {
            instance["image"] = serde_json::to_value(img).unwrap_or_default();
        }
        if let Some(lf) = &request.last_frame {
            instance["lastFrame"] = serde_json::to_value(lf).unwrap_or_default();
        }
        if let Some(refs) = &request.reference_images {
            instance["referenceImages"] = serde_json::to_value(refs).unwrap_or_default();
        }
        if let Some(vid) = &request.video {
            instance["video"] = serde_json::to_value(vid).unwrap_or_default();
        }

        let parameters = request
            .config
            .as_ref()
            .map(|c| serde_json::to_value(c).unwrap_or_default());

        let body = PredictRequest {
            instances: vec![instance],
            parameters,
        };

        let response = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Get the status of a video generation operation.
    pub async fn get_video_operation(&self, operation_name: &str) -> Result<VideoOperation> {
        let url = format!(
            "{}/{}?key={}",
            self.config.base_url, operation_name, self.config.api_key
        );

        let response = self.http.get(&url).headers(self.headers()).send().await?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        Ok(response.json().await?)
    }

    /// Download a generated video from its URI.
    pub async fn download_video(&self, uri: &str) -> Result<Vec<u8>> {
        let url = if uri.contains('?') {
            format!("{}&key={}", uri, self.config.api_key)
        } else {
            format!("{}?key={}", uri, self.config.api_key)
        };

        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(GeminiError::ApiError {
                status: response.status().as_u16(),
                message: "Failed to download video".to_string(),
                details: None,
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request for generating content.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    /// The model to use (optional, uses default if not specified).
    #[serde(skip)]
    pub model: Option<String>,
    /// The content to generate from.
    pub contents: Vec<Content>,
    /// System instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
    /// Generation configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Safety settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    /// Tools available to the model (function declarations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Cached content to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
    /// Built-in tools (Google Search, URL Context, Code Execution) - uses same "tools" key.
    /// These get merged with `tools` when serializing.
    #[serde(skip_serializing_if = "Option::is_none", rename = "tools")]
    pub builtin_tools: Option<Vec<ToolConfig>>,
    /// Tool configuration (function calling mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolSettings>,
}

impl GenerateContentRequest {
    pub fn new(contents: Vec<Content>) -> Self {
        Self {
            contents,
            ..Default::default()
        }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        Self::new(vec![Content::user(text)])
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn system_instruction(mut self, instruction: impl Into<String>) -> Self {
        self.system_instruction = Some(Content {
            parts: vec![Part::text(instruction)],
            role: None,
        });
        self
    }

    pub fn generation_config(mut self, config: GenerationConfig) -> Self {
        self.generation_config = Some(config);
        self
    }

    pub fn safety_settings(mut self, settings: Vec<SafetySetting>) -> Self {
        self.safety_settings = Some(settings);
        self
    }

    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn add_content(mut self, content: Content) -> Self {
        self.contents.push(content);
        self
    }

    /// Add built-in tools like Google Search, URL Context, Code Execution.
    pub fn with_builtin_tools(mut self, tools: Vec<ToolConfig>) -> Self {
        self.builtin_tools = Some(tools);
        self
    }

    /// Enable Google Search grounding.
    pub fn with_google_search(mut self) -> Self {
        let tools = self.builtin_tools.get_or_insert_with(Vec::new);
        tools.push(ToolConfig::google_search());
        self
    }

    /// Enable URL Context tool.
    pub fn with_url_context(mut self) -> Self {
        let tools = self.builtin_tools.get_or_insert_with(Vec::new);
        tools.push(ToolConfig::url_context());
        self
    }

    /// Enable Code Execution tool.
    pub fn with_code_execution(mut self) -> Self {
        let tools = self.builtin_tools.get_or_insert_with(Vec::new);
        tools.push(ToolConfig::code_execution());
        self
    }

    /// Set function calling configuration (mode and allowed functions).
    pub fn function_calling_config(mut self, config: FunctionCallingConfig) -> Self {
        self.tool_config = Some(ToolSettings::with_function_calling(config));
        self
    }

    /// Force the model to always call a function.
    pub fn force_function_call(mut self) -> Self {
        self.tool_config = Some(ToolSettings::force_function_call());
        self
    }

    /// Force the model to call only specific functions.
    pub fn force_function_call_of(mut self, function_names: Vec<&str>) -> Self {
        self.tool_config = Some(ToolSettings::force_function_call_of(function_names));
        self
    }

    /// Disable function calling temporarily.
    pub fn disable_function_calling(mut self) -> Self {
        self.tool_config = Some(ToolSettings::disable_function_call());
        self
    }

    /// Set thinking level for Gemini 3 models.
    pub fn thinking_level(mut self, level: ThinkingLevel) -> Self {
        let config = self
            .generation_config
            .get_or_insert_with(GenerationConfig::default);
        let thinking = config
            .thinking_config
            .get_or_insert_with(ThinkingConfig::default);
        thinking.thinking_level = Some(level);
        self
    }

    /// Disable thinking for faster responses (Gemini 2.5 Flash).
    pub fn disable_thinking(mut self) -> Self {
        let config = self
            .generation_config
            .get_or_insert_with(GenerationConfig::default);
        let thinking = config
            .thinking_config
            .get_or_insert_with(ThinkingConfig::default);
        thinking.thinking_budget = Some(0);
        self
    }
}

/// Response from content generation.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    /// Generated candidates.
    #[serde(default)]
    pub candidates: Vec<Candidate>,
    /// Usage metadata.
    pub usage_metadata: Option<UsageMetadata>,
    /// Model version used.
    pub model_version: Option<String>,
}

impl GenerateContentResponse {
    /// Get the text from the first candidate.
    pub fn text(&self) -> Option<&str> {
        self.candidates.first().and_then(|c| c.text())
    }
}

/// Request for counting tokens.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensRequest {
    #[serde(skip)]
    pub model: Option<String>,
    pub contents: Vec<Content>,
}

impl CountTokensRequest {
    pub fn new(contents: Vec<Content>) -> Self {
        Self {
            model: None,
            contents,
        }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        Self::new(vec![Content::user(text)])
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
}

/// Response from token counting.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: u32,
}

/// Request for embedding content.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmbedContentRequest {
    #[serde(skip)]
    pub model: Option<String>,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimensionality: Option<i32>,
}

impl EmbedContentRequest {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            model: None,
            content: Content::user(text),
            task_type: None,
            title: None,
            output_dimensionality: None,
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the task type for the embedding.
    /// Use TaskType constants for common task types.
    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = Some(task_type.into());
        self
    }

    /// Set the title (only used with RETRIEVAL_DOCUMENT task type).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the output dimensionality.
    /// Recommended values: 768, 1536, or 3072 (default).
    /// Smaller dimensions save storage while maintaining quality.
    pub fn output_dimensionality(mut self, dim: i32) -> Self {
        self.output_dimensionality = Some(dim);
        self
    }

    /// Configure for semantic similarity comparison.
    pub fn for_similarity(mut self) -> Self {
        self.task_type = Some(TaskType::SEMANTIC_SIMILARITY.to_string());
        self
    }

    /// Configure for document retrieval (indexing documents).
    pub fn for_document(mut self) -> Self {
        self.task_type = Some(TaskType::RETRIEVAL_DOCUMENT.to_string());
        self
    }

    /// Configure for query retrieval (searching).
    pub fn for_query(mut self) -> Self {
        self.task_type = Some(TaskType::RETRIEVAL_QUERY.to_string());
        self
    }

    /// Configure for classification.
    pub fn for_classification(mut self) -> Self {
        self.task_type = Some(TaskType::CLASSIFICATION.to_string());
        self
    }

    /// Configure for clustering.
    pub fn for_clustering(mut self) -> Self {
        self.task_type = Some(TaskType::CLUSTERING.to_string());
        self
    }
}

/// Response from embedding content.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmbedContentResponse {
    pub embedding: ContentEmbedding,
}

/// Embedding values.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContentEmbedding {
    pub values: Vec<f32>,
}

/// Request for batch embedding.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchEmbedContentsRequest {
    #[serde(skip)]
    pub model: Option<String>,
    pub requests: Vec<EmbedContentRequest>,
}

/// Response from batch embedding.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchEmbedContentsResponse {
    pub embeddings: Vec<ContentEmbedding>,
}

/// Response from file upload.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    pub file: FileMetadata,
}

/// File metadata.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub name: String,
    pub display_name: Option<String>,
    pub mime_type: String,
    pub size_bytes: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub expiration_time: Option<String>,
    pub sha256_hash: Option<String>,
    pub uri: Option<String>,
    pub state: Option<String>,
}

impl FileMetadata {
    /// Check if the file is ready to use (processing complete).
    pub fn is_active(&self) -> bool {
        self.state.as_deref() == Some("ACTIVE")
    }

    /// Check if the file is still processing.
    pub fn is_processing(&self) -> bool {
        self.state.as_deref() == Some("PROCESSING")
    }

    /// Get a short name without the "files/" prefix.
    pub fn short_name(&self) -> &str {
        self.name.strip_prefix("files/").unwrap_or(&self.name)
    }
}

/// Response from listing files.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListFilesResponse {
    #[serde(default)]
    pub files: Vec<FileMetadata>,
    pub next_page_token: Option<String>,
}

/// Request for creating cached content.
///
/// Note: For explicit caching, you must use an explicit model version suffix
/// (e.g., "gemini-2.0-flash-001" not just "gemini-2.0-flash").
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateCachedContentRequest {
    /// The model to use (must include version suffix like "-001").
    pub model: String,
    /// The content to cache.
    pub contents: Vec<Content>,
    /// Optional system instruction to cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
    /// Time to live as a duration string (e.g., "300s", "1h").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    /// Absolute expiration time (ISO 8601 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// Display name to identify the cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl CreateCachedContentRequest {
    /// Create a new cached content request.
    ///
    /// Note: Use explicit model version (e.g., "gemini-2.0-flash-001").
    pub fn new(model: impl Into<String>, contents: Vec<Content>) -> Self {
        Self {
            model: model.into(),
            contents,
            system_instruction: None,
            ttl: None,
            expire_time: None,
            display_name: None,
        }
    }

    /// Create a cached content request with a file.
    pub fn with_file(model: impl Into<String>, file_uri: &str, mime_type: &str) -> Self {
        Self {
            model: model.into(),
            contents: vec![Content::user_with_file(file_uri, mime_type)],
            system_instruction: None,
            ttl: None,
            expire_time: None,
            display_name: None,
        }
    }

    /// Set the system instruction.
    pub fn system_instruction(mut self, instruction: impl Into<String>) -> Self {
        self.system_instruction = Some(Content::system(instruction));
        self
    }

    /// Set the TTL (time to live) as a duration string.
    /// Examples: "300s" (5 minutes), "3600s" (1 hour), "86400s" (1 day).
    pub fn ttl(mut self, ttl: impl Into<String>) -> Self {
        self.ttl = Some(ttl.into());
        self
    }

    /// Set the TTL in seconds.
    pub fn ttl_seconds(mut self, seconds: u64) -> Self {
        self.ttl = Some(format!("{}s", seconds));
        self
    }

    /// Set the absolute expiration time (ISO 8601 format).
    pub fn expire_time(mut self, time: impl Into<String>) -> Self {
        self.expire_time = Some(time.into());
        self
    }

    /// Set the display name.
    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }
}

/// Cached content resource.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CachedContent {
    /// Resource name (e.g., "cachedContents/abc123").
    pub name: String,
    /// The model this cache is for.
    pub model: String,
    /// When the cache was created.
    pub create_time: Option<String>,
    /// When the cache was last updated.
    pub update_time: Option<String>,
    /// When the cache will expire.
    pub expire_time: Option<String>,
    /// Display name for identification.
    pub display_name: Option<String>,
    /// Token usage for this cache.
    pub usage_metadata: Option<CachedContentUsageMetadata>,
}

impl CachedContent {
    /// Get the short name without the "cachedContents/" prefix.
    pub fn short_name(&self) -> &str {
        self.name
            .strip_prefix("cachedContents/")
            .unwrap_or(&self.name)
    }

    /// Get the total token count in this cache.
    pub fn token_count(&self) -> Option<u32> {
        self.usage_metadata.as_ref().map(|u| u.total_token_count)
    }
}

/// Usage metadata for cached content.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CachedContentUsageMetadata {
    pub total_token_count: u32,
}

/// Response from listing cached contents.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListCachedContentsResponse {
    #[serde(default)]
    pub cached_contents: Vec<CachedContent>,
    pub next_page_token: Option<String>,
}

// ============================================================================
// Batch API Types (50% cheaper async processing)
// ============================================================================

/// State of a batch job.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BatchJobState {
    /// State unspecified.
    BatchStateUnspecified,
    /// Job is pending and waiting to be processed.
    BatchStatePending,
    /// Job is currently being processed.
    BatchStateRunning,
    /// Job completed successfully.
    BatchStateSucceeded,
    /// Job failed.
    BatchStateFailed,
    /// Job was cancelled.
    BatchStateCancelled,
}

/// A single inlined request in a batch.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct InlinedRequest {
    /// The generate content request.
    pub request: GenerateContentRequest,
    /// Optional metadata with a key to identify the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BatchRequestMetadata>,
}

/// Metadata for identifying a batch request.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchRequestMetadata {
    /// A unique key to identify this request in the batch.
    pub key: Option<String>,
}

/// A single inlined response from a batch.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InlinedResponse {
    /// The generate content response (if successful).
    pub response: Option<GenerateContentResponse>,
    /// Error status (if failed).
    pub error: Option<BatchError>,
    /// The key from the request metadata.
    pub key: Option<String>,
}

/// Error from batch processing.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchError {
    pub code: Option<i32>,
    pub message: Option<String>,
}

/// Input configuration for a batch job.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct BatchInputConfig {
    /// Inlined requests (for inline mode) - only used when sending.
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub requests: Option<InlinedRequests>,
    /// GCS file name for JSONL input (for file mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// Wrapper for inlined requests (send-only, not deserialized).
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct InlinedRequests {
    pub requests: Vec<InlinedRequest>,
}

/// Output configuration for a batch job.
/// Output configuration for a batch job (response only).
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchOutputConfig {
    /// Inlined responses (for inline mode).
    pub inlined_responses: Option<InlinedResponses>,
    /// GCS file name for JSONL output (for file mode).
    pub file_name: Option<String>,
}

/// Wrapper for inlined responses.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InlinedResponses {
    /// The inner responses field (API returns as "inlinedResponses")
    #[serde(alias = "responses")]
    pub inlined_responses: Vec<InlinedResponse>,
}

/// Request to create a batch job.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BatchJobRequest {
    /// The model to use (not serialized, used in URL).
    #[serde(skip)]
    pub model: String,
    /// Optional display name for the batch job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Input configuration specifying the requests.
    pub input_config: BatchInputConfig,
}

impl BatchJobRequest {
    /// Create a new batch job request with inlined requests.
    pub fn inline(model: impl Into<String>, requests: Vec<InlinedRequest>) -> Self {
        Self {
            model: model.into(),
            display_name: None,
            input_config: BatchInputConfig {
                requests: Some(InlinedRequests { requests }),
                file_name: None,
            },
        }
    }

    /// Create a new batch job request from a GCS file.
    pub fn from_file(model: impl Into<String>, file_name: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            display_name: None,
            input_config: BatchInputConfig {
                requests: None,
                file_name: Some(file_name.into()),
            },
        }
    }

    /// Set the display name.
    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }
}

impl InlinedRequest {
    /// Create a new inlined request.
    pub fn new(request: GenerateContentRequest) -> Self {
        Self {
            request,
            metadata: None,
        }
    }

    /// Create a new inlined request with a key.
    pub fn with_key(request: GenerateContentRequest, key: impl Into<String>) -> Self {
        Self {
            request,
            metadata: Some(BatchRequestMetadata {
                key: Some(key.into()),
            }),
        }
    }
}

/// A batch job resource.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchJob {
    /// The batch job resource name (e.g., "batches/abc123").
    pub name: String,
    /// Optional display name.
    pub display_name: Option<String>,
    /// Current state of the batch job.
    pub state: BatchJobState,
    /// Creation timestamp.
    pub create_time: Option<String>,
    /// Update timestamp.
    pub update_time: Option<String>,
    /// Error details (if state is FAILED).
    pub error: Option<BatchError>,
    /// Input configuration.
    pub src: Option<BatchInputConfig>,
    /// Output configuration (available when succeeded) - older API format.
    pub dest: Option<BatchOutputConfig>,
    /// Output configuration (available when succeeded) - newer API format.
    pub output: Option<BatchOutputConfig>,
}

impl BatchJob {
    /// Check if the job is still pending.
    pub fn is_pending(&self) -> bool {
        matches!(self.state, BatchJobState::BatchStatePending)
    }

    /// Check if the job is running.
    pub fn is_running(&self) -> bool {
        matches!(self.state, BatchJobState::BatchStateRunning)
    }

    /// Check if the job completed successfully.
    pub fn succeeded(&self) -> bool {
        matches!(self.state, BatchJobState::BatchStateSucceeded)
    }

    /// Check if the job failed.
    pub fn failed(&self) -> bool {
        matches!(self.state, BatchJobState::BatchStateFailed)
    }

    /// Check if the job was cancelled.
    pub fn cancelled(&self) -> bool {
        matches!(self.state, BatchJobState::BatchStateCancelled)
    }

    /// Check if the job is complete (succeeded, failed, or cancelled).
    pub fn is_complete(&self) -> bool {
        matches!(
            self.state,
            BatchJobState::BatchStateSucceeded
                | BatchJobState::BatchStateFailed
                | BatchJobState::BatchStateCancelled
        )
    }

    /// Get the inlined responses (if available).
    /// Checks both `output` (newer API) and `dest` (older API) fields.
    pub fn responses(&self) -> Option<&Vec<InlinedResponse>> {
        // Try the newer `output` field first, then fall back to `dest`
        self.output
            .as_ref()
            .and_then(|o| o.inlined_responses.as_ref())
            .map(|r| &r.inlined_responses)
            .or_else(|| {
                self.dest
                    .as_ref()
                    .and_then(|d| d.inlined_responses.as_ref())
                    .map(|r| &r.inlined_responses)
            })
    }

    /// Get the output file name (for file-based batches).
    pub fn output_file(&self) -> Option<&str> {
        self.output
            .as_ref()
            .and_then(|o| o.file_name.as_deref())
            .or_else(|| self.dest.as_ref().and_then(|d| d.file_name.as_deref()))
    }
}

/// Response from listing batch jobs.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListBatchJobsResponse {
    #[serde(default)]
    pub batch_jobs: Vec<BatchJob>,
    pub next_page_token: Option<String>,
}

// ============================================================================
// Video Generation Types (Veo)
// ============================================================================

/// Request for generating videos with Veo.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenerateVideosRequest {
    /// The model to use (e.g., "veo-3.1-generate-preview").
    #[serde(skip)]
    pub model: String,
    /// Text prompt describing the video to generate.
    pub prompt: String,
    /// Optional negative prompt (what NOT to include).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Optional starting image for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<VideoImage>,
    /// Optional last frame image for interpolation (Veo 3.1 only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_frame: Option<VideoImage>,
    /// Reference images for style/content guidance (Veo 3.1 only, up to 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_images: Option<Vec<VideoReferenceImage>>,
    /// Video to extend (Veo 3.1 only, must be from previous generation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoInput>,
    /// Video configuration options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GenerateVideosConfig>,
}

impl GenerateVideosRequest {
    /// Create a new video generation request with a text prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            model: models::defaults::VIDEO_GEN.to_string(),
            prompt: prompt.into(),
            negative_prompt: None,
            image: None,
            last_frame: None,
            reference_images: None,
            video: None,
            config: None,
        }
    }

    /// Set the model to use.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set a negative prompt (things to avoid in the video).
    pub fn negative_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(prompt.into());
        self
    }

    /// Set the starting image for image-to-video generation.
    pub fn image(mut self, image: VideoImage) -> Self {
        self.image = Some(image);
        self
    }

    /// Set the last frame for interpolation (Veo 3.1 only).
    pub fn last_frame(mut self, image: VideoImage) -> Self {
        self.last_frame = Some(image);
        self
    }

    /// Add reference images for style guidance (Veo 3.1 only, up to 3).
    pub fn reference_images(mut self, images: Vec<VideoReferenceImage>) -> Self {
        self.reference_images = Some(images);
        self
    }

    /// Set the video to extend (Veo 3.1 only).
    pub fn video(mut self, video: VideoInput) -> Self {
        self.video = Some(video);
        self
    }

    /// Set video generation configuration.
    pub fn config(mut self, config: GenerateVideosConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set aspect ratio (convenience method).
    pub fn aspect_ratio(mut self, ratio: &str) -> Self {
        let config = self
            .config
            .get_or_insert_with(GenerateVideosConfig::default);
        config.aspect_ratio = Some(ratio.to_string());
        self
    }

    /// Set resolution (convenience method).
    pub fn resolution(mut self, res: &str) -> Self {
        let config = self
            .config
            .get_or_insert_with(GenerateVideosConfig::default);
        config.resolution = Some(res.to_string());
        self
    }

    /// Set duration in seconds (convenience method).
    pub fn duration(mut self, seconds: u8) -> Self {
        let config = self
            .config
            .get_or_insert_with(GenerateVideosConfig::default);
        config.duration_seconds = Some(seconds);
        self
    }
}

/// Configuration for video generation.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerateVideosConfig {
    /// Aspect ratio: "16:9" (default) or "9:16".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Resolution: "720p" (default) or "1080p" (8s only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Duration in seconds: 4, 6, or 8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u8>,
    /// Number of videos to generate (1 for Veo 3.x, 1-2 for Veo 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_videos: Option<u8>,
    /// Person generation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_generation: Option<String>,
    /// Seed for reproducibility (Veo 3 only, doesn't guarantee determinism).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
}

/// Image input for video generation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoImage {
    /// Base64-encoded image bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_bytes: Option<String>,
    /// MIME type of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// URI of an uploaded image file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl VideoImage {
    /// Create from base64-encoded bytes.
    pub fn from_bytes(bytes: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            image_bytes: Some(bytes.into()),
            mime_type: Some(mime_type.into()),
            uri: None,
        }
    }

    /// Create from a file URI.
    pub fn from_uri(uri: impl Into<String>) -> Self {
        Self {
            image_bytes: None,
            mime_type: None,
            uri: Some(uri.into()),
        }
    }
}

/// Reference image for guiding video content (Veo 3.1 only).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoReferenceImage {
    /// The reference image.
    pub image: VideoImage,
    /// Reference type: "asset" for preserving subject appearance.
    pub reference_type: String,
}

impl VideoReferenceImage {
    /// Create an asset reference image (preserves subject appearance).
    pub fn asset(image: VideoImage) -> Self {
        Self {
            image,
            reference_type: "asset".to_string(),
        }
    }
}

/// Video input for extending previously generated videos.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoInput {
    /// URI of the video to extend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Video bytes (from previous generation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_bytes: Option<Vec<u8>>,
}

/// Long-running operation for video generation.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoOperation {
    /// Operation name/ID.
    pub name: String,
    /// Whether the operation is complete.
    #[serde(default)]
    pub done: bool,
    /// Error if the operation failed.
    #[serde(default)]
    pub error: Option<OperationError>,
    /// Response when operation completes successfully.
    #[serde(default)]
    pub response: Option<GenerateVideosResponse>,
}

/// Error from a long-running operation.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationError {
    pub code: i32,
    pub message: String,
}

/// Response from video generation.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenerateVideosResponse {
    /// Generated videos.
    #[serde(default)]
    pub generated_videos: Vec<GeneratedVideo>,
}

/// A generated video.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedVideo {
    /// The video file information.
    pub video: VideoFile,
}

/// Video file information.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoFile {
    /// URI to download the video.
    #[serde(default)]
    pub uri: Option<String>,
    /// Video bytes (after download).
    #[serde(default)]
    pub video_bytes: Option<Vec<u8>>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Detect MIME type from file extension.
fn mime_from_path(path: &std::path::Path) -> String {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "aac" => "audio/aac",
        "ogg" => "audio/ogg",
        "flac" => "audio/flac",
        "m4a" => "audio/mp4",
        "weba" => "audio/webm",
        // Video
        "mp4" => "video/mp4",
        "mpeg" | "mpg" => "video/mpeg",
        "mov" => "video/quicktime",
        "avi" => "video/x-msvideo",
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "flv" => "video/x-flv",
        "wmv" => "video/x-ms-wmv",
        "3gp" => "video/3gpp",
        // Images
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "heic" | "heif" => "image/heic",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        // Documents
        "pdf" => "application/pdf",
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "md" => "text/markdown",
        "csv" => "text/csv",
        // Code
        "py" => "text/x-python",
        "rs" => "text/x-rust",
        "go" => "text/x-go",
        "java" => "text/x-java",
        "c" => "text/x-c",
        "cpp" | "cc" | "cxx" => "text/x-c++",
        "h" | "hpp" => "text/x-c",
        "ts" => "text/typescript",
        "rb" => "text/x-ruby",
        "php" => "text/x-php",
        "swift" => "text/x-swift",
        "kt" => "text/x-kotlin",
        // Default
        _ => "application/octet-stream",
    }
    .to_string()
}

// ============================================================================
// File Upload Logger
// ============================================================================

/// A logger that tracks local file to remote URI mappings.
///
/// Use this to keep a record of which local files have been uploaded
/// and their corresponding remote URIs.
///
/// # Example
/// ```no_run
/// use gemini_client::{GeminiClient, FileLogger};
///
/// let client = GeminiClient::from_env().unwrap();
/// let logger = FileLogger::new("file_mappings.log").unwrap();
///
/// // Upload and log
/// let file = client.upload_file("audio.mp3", None).await?;
/// logger.log("audio.mp3", &file)?;
///
/// // Later, read all mappings
/// for mapping in logger.read_all()? {
///     println!("{} -> {}", mapping.local_path, mapping.remote_uri);
/// }
/// ```
#[derive(Debug)]
pub struct FileLogger {
    path: PathBuf,
    file: Mutex<std::fs::File>,
}

/// A single file mapping entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMapping {
    /// Local file path that was uploaded.
    pub local_path: String,
    /// Remote file name (e.g., "files/abc123").
    pub remote_name: String,
    /// Remote file URI for use in API calls.
    pub remote_uri: String,
    /// MIME type of the file.
    pub mime_type: String,
    /// Upload timestamp (ISO 8601).
    pub uploaded_at: String,
    /// File expiration time (ISO 8601).
    pub expires_at: Option<String>,
}

impl FileLogger {
    /// Create a new file logger that writes to the specified path.
    ///
    /// The log file will be created if it doesn't exist, or appended to if it does.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| GeminiError::InvalidConfig(format!("Failed to open log file: {}", e)))?;

        Ok(Self {
            path,
            file: Mutex::new(file),
        })
    }

    /// Log a file upload mapping.
    pub fn log(&self, local_path: impl AsRef<str>, file: &FileMetadata) -> Result<()> {
        let mapping = FileMapping {
            local_path: local_path.as_ref().to_string(),
            remote_name: file.name.clone(),
            remote_uri: file.uri.clone().unwrap_or_default(),
            mime_type: file.mime_type.clone(),
            uploaded_at: chrono_now(),
            expires_at: file.expiration_time.clone(),
        };

        let line = serde_json::to_string(&mapping).map_err(|e| {
            GeminiError::InvalidConfig(format!("Failed to serialize mapping: {}", e))
        })?;

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", line)
            .map_err(|e| GeminiError::InvalidConfig(format!("Failed to write to log: {}", e)))?;
        file.flush()
            .map_err(|e| GeminiError::InvalidConfig(format!("Failed to flush log: {}", e)))?;

        Ok(())
    }

    /// Read all file mappings from the log.
    pub fn read_all(&self) -> Result<Vec<FileMapping>> {
        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| GeminiError::InvalidConfig(format!("Failed to read log file: {}", e)))?;

        let mut mappings = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<FileMapping>(line) {
                Ok(mapping) => mappings.push(mapping),
                Err(e) => {
                    // Skip malformed lines but log a warning
                    eprintln!("Warning: Skipping malformed log line: {}", e);
                }
            }
        }
        Ok(mappings)
    }

    /// Find a mapping by local path.
    pub fn find_by_local_path(&self, local_path: &str) -> Result<Option<FileMapping>> {
        let mappings = self.read_all()?;
        Ok(mappings.into_iter().find(|m| m.local_path == local_path))
    }

    /// Find a mapping by remote name.
    pub fn find_by_remote_name(&self, remote_name: &str) -> Result<Option<FileMapping>> {
        let mappings = self.read_all()?;
        Ok(mappings.into_iter().find(|m| m.remote_name == remote_name))
    }

    /// Get the path to the log file.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

/// Get current time as ISO 8601 string.
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    // Simple ISO 8601 format without external dependency
    let secs = duration.as_secs();
    let days_since_1970 = secs / 86400;
    let secs_today = secs % 86400;

    // Calculate year/month/day (simplified - doesn't account for leap years perfectly)
    let mut year = 1970i64;
    let mut remaining_days = days_since_1970 as i64;

    loop {
        let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            366
        } else {
            365
        };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month = 1;
    for &dim in &days_in_month {
        if remaining_days < dim as i64 {
            break;
        }
        remaining_days -= dim as i64;
        month += 1;
    }
    let day = remaining_days + 1;

    let hours = secs_today / 3600;
    let minutes = (secs_today % 3600) / 60;
    let seconds = secs_today % 60;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

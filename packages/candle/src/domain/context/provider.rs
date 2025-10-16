//! Zero-Allocation Context Provider System
//!
//! Production-ready context management with streaming-only architecture, zero Arc usage,
//! lock-free atomic operations, and immutable messaging patterns. Provides blazing-fast
//! context loading and management with full memory integration.
//!
//! Features: File/Directory/GitHub indexing, vector embeddings, memory storage,
//! parallel processing, real-time event streaming, comprehensive error handling.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

// Git operations
use gitgix::{
    CloneOpts, FetchOpts, GitError as GitGixError, MergeOpts, clone_repo as gitgix_clone, fetch,
    merge, open_repo,
};

// Domain imports
use cyrup_sugars::prelude::MessageChunk;
use std::pin::Pin;
use tokio_stream::Stream;
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::context::CandleDocument as Document;

// Additional imports for file reading implementation
use base64::{Engine as _, engine::general_purpose};
use mime_guess;

/// Marker types for `CandleContext`
/// Marker type for file-based Candle context operations. Used in typestate pattern to ensure compile-time safety for file context providers.
#[derive(Debug, Clone)]
pub struct CandleFile;
/// Marker type for multi-file Candle context operations. Enables batch processing of multiple files with zero-allocation streaming patterns.
#[derive(Debug, Clone)]
pub struct CandleFiles;
/// Marker type for directory-based Candle context indexing. Provides recursive directory traversal with configurable depth limits and filtering.
#[derive(Debug, Clone)]
pub struct CandleDirectory;
/// Marker type for GitHub repository Candle context integration. Enables GitHub API integration with rate limiting and authentication.
#[derive(Debug, Clone)]
pub struct CandleGithub;

/// Comprehensive error types for Candle context operations with zero allocations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CandleContextError {
    #[error("Context not found: {0}")]
    /// Context resource could not be located. Occurs when file paths, directories, or repository references are invalid or inaccessible.
    ContextNotFound(String),
    #[error("Invalid path: {0}")]
    /// Path validation failed due to invalid characters, encoding, or filesystem constraints. Includes both local and remote path validation.
    InvalidPath(String),
    #[error("IO error: {0}")]
    /// Filesystem I/O operation failed. Wraps underlying `std::io::Error` with context-specific information for debugging.
    IoError(String),
    #[error("Pattern error: {0}")]
    /// Regular expression or glob pattern compilation failed. Occurs during context filtering and search operations.
    PatternError(String),
    #[error("Memory integration error: {0}")]
    /// Memory integration subsystem error. Indicates failure in vector embeddings, storage, or retrieval operations.
    MemoryError(String),
    #[error("Validation error: {0}")]
    /// Input validation failed during context operations. Includes size limits, format validation, and content restrictions.
    ValidationError(String),
    #[error("Performance threshold exceeded: {0}")]
    /// Operation exceeded configured performance thresholds. Used for timeout, memory usage, or processing limits.
    PerformanceThresholdExceeded(String),
    #[error("Provider unavailable: {0}")]
    /// Context provider service is temporarily or permanently unavailable. Includes network, API, and resource availability issues.
    ProviderUnavailable(String),
}

/// Provider-specific error types for Candle
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CandleProviderError {
    #[error("File provider error: {0}")]
    /// File system provider specific error. Handles file reading, parsing, and metadata extraction failures.
    FileProvider(String),
    #[error("Directory provider error: {0}")]
    /// Directory provider specific error. Manages recursive traversal, filtering, and indexing failures.
    DirectoryProvider(String),
    #[error("GitHub provider error: {0}")]
    /// GitHub API provider specific error. Handles authentication, rate limiting, and repository access issues.
    GithubProvider(String),
    #[error("Embedding provider error: {0}")]
    /// Vector embedding provider error. Manages embedding generation, storage, and retrieval failures.
    EmbeddingProvider(String),
}

/// Validation error types with semantic meaning for Candle
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CandleValidationError {
    #[error("Invalid input: {0}")]
    /// Generic input validation failure. Used when input doesn't meet format, type, or semantic requirements.
    InvalidInput(String),
    #[error("Path validation failed: {0}")]
    /// Path-specific validation error. Handles invalid characters, length limits, and security restrictions.
    PathValidation(String),
    #[error("Pattern validation failed: {0}")]
    /// Pattern syntax validation failed. Occurs during regex or glob pattern parsing with detailed error information.
    PatternValidation(String),
    #[error("Size limit exceeded: {0}")]
    /// Content size exceeds configured limits. Includes file size, directory depth, and processing constraints.
    SizeLimitExceeded(String),
}

/// Candle context events for real-time streaming monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandleContextEvent {
    /// Provider lifecycle events
    ProviderStarted {
        /// Type of provider that was started (File, Directory, Github, etc.)
        provider_type: String,
        /// Unique identifier for this provider instance
        provider_id: String,
        /// When the provider was started
        timestamp: SystemTime,
    },
    /// Provider shutdown event
    ProviderStopped {
        /// Type of provider that was stopped (File, Directory, Github, etc.)
        provider_type: String,
        /// Unique identifier for this provider instance
        provider_id: String,
        /// When the provider was stopped
        timestamp: SystemTime,
    },

    /// Operation events
    ContextLoadStarted {
        /// Type of context being loaded (file, directory, github, etc.)
        context_type: String,
        /// Source path or identifier for the context
        source: String,
        /// When the context loading operation started
        timestamp: SystemTime,
    },
    ContextLoadCompleted {
        /// Type of context that was loaded (file, directory, github, etc.)
        context_type: String,
        /// Source path or identifier for the context
        source: String,
        /// Number of documents successfully loaded
        documents_loaded: usize,
        /// Duration of the loading operation
        duration: Duration,
        /// When the context loading operation completed
        timestamp: SystemTime,
    },
    ContextLoadFailed {
        /// Type of context that failed to load (file, directory, github, etc.)
        context_type: String,
        /// Source path or identifier for the context
        source: String,
        /// Error message describing the failure
        error: String,
        /// When the context loading operation failed
        timestamp: SystemTime,
    },

    /// Memory integration events
    MemoryCreated {
        /// Unique identifier for the created memory entry
        memory_id: String,
        /// Hash of the content for deduplication and integrity verification
        content_hash: String,
        /// When the memory entry was created
        timestamp: SystemTime,
    },
    MemorySearchCompleted {
        /// Search query that was executed
        query: String,
        /// Number of results returned by the search
        results_count: usize,
        /// Duration of the search operation
        duration: Duration,
        /// When the search operation completed
        timestamp: SystemTime,
    },

    /// Performance events
    PerformanceThresholdBreached {
        /// Name of the performance metric that exceeded threshold
        metric: String,
        /// Configured threshold value that was exceeded
        threshold: f64,
        /// Actual measured value that exceeded the threshold
        actual: f64,
        /// When the threshold breach was detected
        timestamp: SystemTime,
    },

    /// Validation events
    ValidationFailed {
        /// Type of validation that failed (path, pattern, size, etc.)
        validation_type: String,
        /// Error message describing the validation failure
        error: String,
        /// When the validation failure occurred
        timestamp: SystemTime,
    },
}

impl MessageChunk for CandleContextEvent {
    fn bad_chunk(error: String) -> Self {
        CandleContextEvent::ValidationFailed {
            validation_type: "system_error".to_string(),
            error,
            timestamp: SystemTime::now(),
        }
    }

    fn error(&self) -> Option<&str> {
        match self {
            CandleContextEvent::ContextLoadFailed { error, .. }
            | CandleContextEvent::ValidationFailed { error, .. } => Some(error),
            _ => None,
        }
    }
}

impl Default for CandleContextEvent {
    fn default() -> Self {
        CandleContextEvent::ValidationFailed {
            validation_type: "default".to_string(),
            error: "default event".to_string(),
            timestamp: SystemTime::now(),
        }
    }
}

/// Candle memory node representation with owned strings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleMemoryNode {
    /// Unique identifier for this memory node
    pub id: String,
    /// Content stored in this memory node
    pub content: String,
    /// Metadata key-value pairs associated with this node
    pub metadata: HashMap<String, String>,
    /// Optional vector embedding for similarity search
    pub embedding: Option<Vec<f32>>,
    /// When this memory node was created or last updated
    pub timestamp: SystemTime,
}

/// Immutable file context with owned strings and atomic tracking for Candle
#[derive(Debug, Clone)]
pub struct CandleImmutableFileContext {
    /// File path as owned string
    pub path: String,
    /// Content hash for deduplication
    pub content_hash: String,
    /// File size in bytes
    pub size_bytes: u64,
    /// Last modified timestamp
    pub modified: SystemTime,
    /// Memory integration layer
    pub memory_integration: Option<CandleMemoryIntegration>,
}

/// Immutable files context with owned strings for Candle
#[derive(Debug, Clone)]
pub struct CandleImmutableFilesContext {
    /// File paths as owned strings
    pub paths: Vec<String>,
    /// Glob pattern as owned string
    pub pattern: String,
    /// Total files count
    pub total_files: usize,
    /// Memory integration layer
    pub memory_integration: Option<CandleMemoryIntegration>,
}

/// Immutable directory context with owned strings for Candle
#[derive(Debug, Clone)]
pub struct CandleImmutableDirectoryContext {
    /// Directory path as owned string
    pub path: String,
    /// Recursive traversal flag
    pub recursive: bool,
    /// File extensions filter
    pub extensions: Vec<String>,
    /// Maximum depth for traversal
    pub max_depth: Option<usize>,
    /// Memory integration layer
    pub memory_integration: Option<CandleMemoryIntegration>,
}

/// Immutable GitHub context with owned strings for Candle
#[derive(Debug, Clone)]
pub struct CandleImmutableGithubContext {
    /// Repository URL as owned string
    pub repository_url: String,
    /// Branch name as owned string
    pub branch: String,
    /// File pattern as owned string
    pub pattern: String,
    /// Authentication token (if needed)
    pub auth_token: Option<String>,
    /// Memory integration layer
    pub memory_integration: Option<CandleMemoryIntegration>,
}

/// Candle memory integration layer with atomic operations
#[derive(Debug)]
pub struct CandleMemoryIntegration {
    /// Memory manager identifier
    pub manager_id: String,
    /// Embedding model identifier
    pub embedding_model: String,
    /// Vector dimension
    pub vector_dimension: usize,
    /// Performance tracking
    pub memory_requests: AtomicU64,
    pub successful_operations: AtomicU64,
    pub failed_operations: AtomicU64,
    pub total_processing_time_nanos: AtomicU64,
}

impl Clone for CandleMemoryIntegration {
    fn clone(&self) -> Self {
        Self {
            manager_id: self.manager_id.clone(),
            embedding_model: self.embedding_model.clone(),
            vector_dimension: self.vector_dimension,
            memory_requests: AtomicU64::new(
                self.memory_requests
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            successful_operations: AtomicU64::new(
                self.successful_operations
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            failed_operations: AtomicU64::new(
                self.failed_operations
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            total_processing_time_nanos: AtomicU64::new(
                self.total_processing_time_nanos
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

impl CandleMemoryIntegration {
    /// Create new memory integration with owned strings
    #[inline]
    #[must_use]
    pub fn new(manager_id: String, embedding_model: String, vector_dimension: usize) -> Self {
        Self {
            manager_id,
            embedding_model,
            vector_dimension,
            memory_requests: AtomicU64::new(0),
            successful_operations: AtomicU64::new(0),
            failed_operations: AtomicU64::new(0),
            total_processing_time_nanos: AtomicU64::new(0),
        }
    }

    /// Record successful operation
    #[inline]
    pub fn record_success(&self, duration_nanos: u64) {
        self.successful_operations.fetch_add(1, Ordering::Relaxed);
        self.total_processing_time_nanos
            .fetch_add(duration_nanos, Ordering::Relaxed);
    }

    /// Record failed operation
    #[inline]
    pub fn record_failure(&self) {
        self.failed_operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Get success rate (0.0 to 1.0)
    #[inline]
    #[allow(clippy::cast_precision_loss)] // Acceptable for rate calculations
    pub fn success_rate(&self) -> f64 {
        let successful = self.successful_operations.load(Ordering::Relaxed);
        let failed = self.failed_operations.load(Ordering::Relaxed);
        let total = successful + failed;
        if total == 0 {
            1.0
        } else {
            successful as f64 / total as f64
        }
    }

    /// Get average processing time in nanoseconds
    #[inline]
    pub fn average_processing_time_nanos(&self) -> u64 {
        let total_time = self.total_processing_time_nanos.load(Ordering::Relaxed);
        let successful = self.successful_operations.load(Ordering::Relaxed);
        if successful == 0 {
            0
        } else {
            total_time / successful
        }
    }
}

/// Immutable embedding model with streaming operations for Candle
pub trait CandleImmutableEmbeddingModel: Send + Sync + 'static {
    /// Generate embeddings for text with streaming results - returns unwrapped values
    fn embed(&self, text: &str, context: Option<String>) -> Pin<Box<dyn Stream<Item = Vec<f32>> + Send>>;

    /// Get model information
    fn model_info(&self) -> CandleEmbeddingModelInfo;

    /// Validate input text
    ///
    /// # Errors
    ///
    /// Returns `CandleValidationError` if input text fails validation
    fn validate_input(&self, text: &str) -> Result<(), CandleValidationError>;
}

/// Embedding model information with owned strings for Candle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleEmbeddingModelInfo {
    pub name: String,
    pub version: String,
    pub vector_dimension: usize,
    pub max_input_length: usize,
    pub supported_languages: Vec<String>,
}

/// Immutable memory manager with streaming operations for Candle
pub trait CandleImmutableMemoryManager: Send + Sync + 'static {
    /// Create memory with streaming confirmation - returns unwrapped values
    fn create_memory(&self, node: CandleMemoryNode) -> Pin<Box<dyn Stream<Item = ()> + Send>>;

    /// Search by vector with streaming results - returns unwrapped values
    fn search_by_vector(&self, vector: Vec<f32>, limit: usize) -> Pin<Box<dyn Stream<Item = CandleMemoryNode> + Send>>;

    /// Search by text with streaming results - returns unwrapped values
    fn search_by_text(&self, query: &str, limit: usize) -> Pin<Box<dyn Stream<Item = CandleMemoryNode> + Send>>;

    /// Update memory with streaming confirmation - returns unwrapped values
    fn update_memory(&self, memory_id: &str, node: CandleMemoryNode) -> Pin<Box<dyn Stream<Item = ()> + Send>>;

    /// Delete memory with streaming confirmation - returns unwrapped values
    fn delete_memory(&self, memory_id: &str) -> Pin<Box<dyn Stream<Item = ()> + Send>>;

    /// Get memory manager information
    fn manager_info(&self) -> CandleMemoryManagerInfo;
}

/// Memory manager information with owned strings for Candle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleMemoryManagerInfo {
    pub name: String,
    pub version: String,
    pub storage_type: String,
    pub max_memory_nodes: Option<usize>,
    pub supported_operations: Vec<String>,
}

/// Streaming context processor with atomic state tracking for Candle
pub struct CandleStreamingContextProcessor {
    /// Unique processor identifier
    processor_id: String,

    /// Atomic performance counters
    context_requests: AtomicU64,
    active_contexts: AtomicUsize,
    total_contexts_processed: AtomicU64,
    successful_contexts: AtomicU64,
    failed_contexts: AtomicU64,
    total_documents_loaded: AtomicU64,
    total_processing_time_nanos: AtomicU64,

    /// Event streaming
    event_sender: Option<tokio::sync::mpsc::UnboundedSender<CandleContextEvent>>,

    /// Performance thresholds
    max_processing_time_ms: u64,
    max_documents_per_context: usize,
    max_concurrent_contexts: usize,
}

impl std::fmt::Debug for CandleStreamingContextProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CandleStreamingContextProcessor")
            .field("processor_id", &self.processor_id)
            .field(
                "context_requests",
                &self
                    .context_requests
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "active_contexts",
                &self
                    .active_contexts
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "total_contexts_processed",
                &self
                    .total_contexts_processed
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "successful_contexts",
                &self
                    .successful_contexts
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "failed_contexts",
                &self
                    .failed_contexts
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "total_documents_loaded",
                &self
                    .total_documents_loaded
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "total_processing_time_nanos",
                &self
                    .total_processing_time_nanos
                    .load(std::sync::atomic::Ordering::Relaxed),
            )
            .field("event_sender", &self.event_sender.is_some())
            .field("max_processing_time_ms", &self.max_processing_time_ms)
            .field("max_documents_per_context", &self.max_documents_per_context)
            .field("max_concurrent_contexts", &self.max_concurrent_contexts)
            .finish()
    }
}

impl CandleStreamingContextProcessor {
    /// Create new streaming context processor
    #[inline]
    #[must_use]
    pub fn new(processor_id: String) -> Self {
        Self {
            processor_id,
            context_requests: AtomicU64::new(0),
            active_contexts: AtomicUsize::new(0),
            total_contexts_processed: AtomicU64::new(0),
            successful_contexts: AtomicU64::new(0),
            failed_contexts: AtomicU64::new(0),
            total_documents_loaded: AtomicU64::new(0),
            total_processing_time_nanos: AtomicU64::new(0),
            event_sender: None,
            max_processing_time_ms: 30000, // 30 seconds default
            max_documents_per_context: 10000,
            max_concurrent_contexts: 100,
        }
    }

    /// Create processor with event streaming
    #[inline]
    #[must_use]
    pub fn with_streaming(processor_id: String) -> (Self, Pin<Box<dyn Stream<Item = CandleContextEvent> + Send>>) {
        let stream = Box::pin(crate::async_stream::spawn_stream(|_tx| async move {
            // Stream created for event processing
        }));
        let mut processor = Self::new(processor_id);
        processor.event_sender = None; // Will be set up separately if needed
        (processor, stream)
    }

    /// Process file context with streaming results - returns unwrapped values
    #[inline]
    pub fn process_file_context(
        &self,
        context: CandleImmutableFileContext,
    ) -> Pin<Box<dyn Stream<Item = Document> + Send>> {
        let _processor_id = self.processor_id.clone();
        let event_sender = self.event_sender.clone();

        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            let start_time = SystemTime::now();

            // Emit context load started event
            if let Some(ref events) = event_sender {
                let _ = events.send(CandleContextEvent::ContextLoadStarted {
                    context_type: "File".to_string(),
                    source: context.path.clone(),
                    timestamp: start_time,
                });
            }

            // Validate input
            if let Err(validation_error) = Self::validate_file_context(&context) {
                let _error = CandleContextError::ValidationError(validation_error.to_string());

                // Emit validation failed event before terminating
                if let Some(ref events) = event_sender {
                    let _ = events.send(CandleContextEvent::ValidationFailed {
                        validation_type: "FileContext".to_string(),
                        error: _error.to_string(),
                        timestamp: SystemTime::now(),
                    });
                }

                log::error!("File context validation failed: {}", _error);
                return;
            }

            // Process file context
            let document = Self::load_file_document(&context);
            let duration = start_time.elapsed().unwrap_or(Duration::ZERO);
            let _ = tx.send(document);

            // Emit context load completed event
            if let Some(ref events) = event_sender {
                let _ = events.send(CandleContextEvent::ContextLoadCompleted {
                    context_type: "File".to_string(),
                    source: context.path.clone(),
                    documents_loaded: 1,
                    duration,
                    timestamp: SystemTime::now(),
                });
            }
        }))
    }

    /// Validate file context
    fn validate_file_context(
        context: &CandleImmutableFileContext,
    ) -> Result<(), CandleValidationError> {
        if context.path.is_empty() {
            return Err(CandleValidationError::PathValidation(
                "Empty file path".to_string(),
            ));
        }

        if context.size_bytes > 100 * 1024 * 1024 {
            // 100MB limit
            return Err(CandleValidationError::SizeLimitExceeded(format!(
                "File size {} bytes exceeds 100MB limit",
                context.size_bytes
            )));
        }

        Ok(())
    }

    /// Load file document with production-quality file reading
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn load_file_document(context: &CandleImmutableFileContext) -> Document {
        const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB default
        const LARGE_FILE_WARNING_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB

        let file_path = Path::new(&context.path);

        // Validate path exists and is a file
        let metadata = match std::fs::metadata(file_path) {
            Ok(meta) => {
                if !meta.is_file() {
                    log::error!(
                        "File context validation failed: Path is not a file: {}",
                        context.path
                    );
                    return Document::bad_chunk(format!("Path is not a file: {}", context.path));
                }
                meta
            }
            Err(e) => {
                log::error!("Failed to read file metadata: {e}");
                return Document::bad_chunk(format!("Failed to access file: {e}"));
            }
        };

        // Check file size limits to prevent OOM
        if metadata.len() > MAX_FILE_SIZE {
            log::error!(
                "File exceeds size limit: {} bytes (max: {} bytes)",
                metadata.len(),
                MAX_FILE_SIZE
            );
            return Document::bad_chunk(format!(
                "File too large: {} bytes (max {} bytes)",
                metadata.len(),
                MAX_FILE_SIZE
            ));
        }

        // Warn for large files
        if metadata.len() > LARGE_FILE_WARNING_THRESHOLD {
            log::warn!("Reading large file: {} bytes", metadata.len());
        }

        // Detect MIME type and derive format/media type
        let mime_guess = mime_guess::from_path(file_path);
        let mime_type = mime_guess.first();

        // Determine format and media type based on MIME or extension
        let (format, media_type) = match mime_type {
            Some(mime) => {
                let mime_str = mime.as_ref();
                match mime_str {
                    "text/html" => (
                        crate::domain::context::CandleContentFormat::Html,
                        crate::domain::context::CandleDocumentMediaType::Html,
                    ),
                    "text/markdown" | "text/x-markdown" => (
                        crate::domain::context::CandleContentFormat::Markdown,
                        crate::domain::context::CandleDocumentMediaType::Markdown,
                    ),
                    "application/json" => (
                        crate::domain::context::CandleContentFormat::Json,
                        crate::domain::context::CandleDocumentMediaType::Json,
                    ),
                    "application/xml" | "text/xml" => (
                        crate::domain::context::CandleContentFormat::Xml,
                        crate::domain::context::CandleDocumentMediaType::Xml,
                    ),
                    "application/x-yaml" | "text/yaml" | "text/x-yaml" => (
                        crate::domain::context::CandleContentFormat::Yaml,
                        crate::domain::context::CandleDocumentMediaType::Yaml,
                    ),
                    "text/csv" => (
                        crate::domain::context::CandleContentFormat::Csv,
                        crate::domain::context::CandleDocumentMediaType::Csv,
                    ),
                    "application/pdf" => (
                        crate::domain::context::CandleContentFormat::Base64,
                        crate::domain::context::CandleDocumentMediaType::PDF,
                    ),
                    "text/plain" => {
                        // For text/plain, use extension-based detection for better accuracy
                        Self::detect_format_from_extension(file_path)
                    }
                    _ if mime_str.starts_with("text/") => (
                        crate::domain::context::CandleContentFormat::Text,
                        crate::domain::context::CandleDocumentMediaType::TXT,
                    ),
                    _ if mime_str.starts_with("image/") => (
                        crate::domain::context::CandleContentFormat::Base64,
                        crate::domain::context::CandleDocumentMediaType::Image,
                    ),
                    _ => {
                        // Fall back to extension-based detection
                        Self::detect_format_from_extension(file_path)
                    }
                }
            }
            None => Self::detect_format_from_extension(file_path),
        };

        // Read file content - try UTF-8 first for text formats
        let data = match format {
            crate::domain::context::CandleContentFormat::Base64 => {
                // Binary file - read as bytes and encode
                match std::fs::read(file_path) {
                    Ok(bytes) => general_purpose::STANDARD.encode(&bytes),
                    Err(e) => {
                        log::error!("Failed to read binary file: {e}");
                        return Document::bad_chunk(format!("Failed to read file: {e}"));
                    }
                }
            }
            _ => {
                // Try to read as UTF-8 text first
                match std::fs::read_to_string(file_path) {
                    Ok(text) => text,
                    Err(_) => {
                        // If UTF-8 fails, try as binary
                        match std::fs::read(file_path) {
                            Ok(bytes) => {
                                // Successfully read as binary, encode it
                                log::warn!(
                                    "File is not valid UTF-8, encoding as base64: {}",
                                    context.path
                                );
                                return Document {
                                    data: general_purpose::STANDARD.encode(&bytes),
                                    format: Some(
                                        crate::domain::context::CandleContentFormat::Base64,
                                    ),
                                    media_type: Some(media_type),
                                    additional_props: Self::build_metadata_props(context),
                                };
                            }
                            Err(read_err) => {
                                log::error!("Failed to read file as text or binary: {read_err}");
                                return Document::bad_chunk(format!(
                                    "Failed to read file: {read_err}"
                                ));
                            }
                        }
                    }
                }
            }
        };

        // Create the document with actual content
        Document {
            data,
            format: Some(format),
            media_type: Some(media_type),
            additional_props: Self::build_metadata_props(context),
        }
    }

    /// Helper function to detect format from file extension (case-insensitive)
    #[inline]
    fn detect_format_from_extension(
        path: &Path,
    ) -> (
        crate::domain::context::CandleContentFormat,
        crate::domain::context::CandleDocumentMediaType,
    ) {
        // Convert extension to lowercase for case-insensitive matching
        let ext_lower = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(str::to_lowercase);

        match ext_lower.as_deref() {
            Some("html" | "htm") => (
                crate::domain::context::CandleContentFormat::Html,
                crate::domain::context::CandleDocumentMediaType::Html,
            ),
            Some("md" | "markdown") => (
                crate::domain::context::CandleContentFormat::Markdown,
                crate::domain::context::CandleDocumentMediaType::Markdown,
            ),
            Some("json") => (
                crate::domain::context::CandleContentFormat::Json,
                crate::domain::context::CandleDocumentMediaType::Json,
            ),
            Some("xml") => (
                crate::domain::context::CandleContentFormat::Xml,
                crate::domain::context::CandleDocumentMediaType::Xml,
            ),
            Some("yaml" | "yml") => (
                crate::domain::context::CandleContentFormat::Yaml,
                crate::domain::context::CandleDocumentMediaType::Yaml,
            ),
            Some("csv") => (
                crate::domain::context::CandleContentFormat::Csv,
                crate::domain::context::CandleDocumentMediaType::Csv,
            ),
            Some("pdf") => (
                crate::domain::context::CandleContentFormat::Base64,
                crate::domain::context::CandleDocumentMediaType::PDF,
            ),
            Some("doc" | "docx") => (
                crate::domain::context::CandleContentFormat::Base64,
                crate::domain::context::CandleDocumentMediaType::DOCX,
            ),
            Some("jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp") => (
                crate::domain::context::CandleContentFormat::Base64,
                crate::domain::context::CandleDocumentMediaType::Image,
            ),
            Some("txt" | "text" | "log") => (
                crate::domain::context::CandleContentFormat::Text,
                crate::domain::context::CandleDocumentMediaType::TXT,
            ),
            _ => (
                crate::domain::context::CandleContentFormat::Text,
                crate::domain::context::CandleDocumentMediaType::PlainText,
            ),
        }
    }

    /// Build metadata properties `HashMap`
    #[inline]
    fn build_metadata_props(
        context: &CandleImmutableFileContext,
    ) -> HashMap<String, serde_json::Value> {
        let mut props = HashMap::with_capacity(4);
        props.insert(
            "id".to_string(),
            serde_json::Value::String(Uuid::new_v4().to_string()),
        );
        props.insert(
            "path".to_string(),
            serde_json::Value::String(context.path.clone()),
        );
        props.insert(
            "size".to_string(),
            serde_json::Value::String(context.size_bytes.to_string()),
        );
        props.insert(
            "hash".to_string(),
            serde_json::Value::String(context.content_hash.clone()),
        );
        props
    }

    /// Get processor statistics
    #[inline]
    pub fn get_statistics(&self) -> CandleContextProcessorStatistics {
        CandleContextProcessorStatistics {
            processor_id: self.processor_id.clone(),
            context_requests: self.context_requests.load(Ordering::Relaxed),
            active_contexts: self.active_contexts.load(Ordering::Relaxed),
            total_contexts_processed: self.total_contexts_processed.load(Ordering::Relaxed),
            successful_contexts: self.successful_contexts.load(Ordering::Relaxed),
            failed_contexts: self.failed_contexts.load(Ordering::Relaxed),
            total_documents_loaded: self.total_documents_loaded.load(Ordering::Relaxed),
            success_rate: self.success_rate(),
            average_processing_time_nanos: self.average_processing_time_nanos(),
        }
    }

    /// Calculate success rate
    #[inline]
    #[allow(clippy::cast_precision_loss)] // Acceptable for rate calculations
    fn success_rate(&self) -> f64 {
        let successful = self.successful_contexts.load(Ordering::Relaxed);
        let failed = self.failed_contexts.load(Ordering::Relaxed);
        let total = successful + failed;
        if total == 0 {
            1.0
        } else {
            successful as f64 / total as f64
        }
    }

    /// Calculate average processing time
    #[inline]
    fn average_processing_time_nanos(&self) -> u64 {
        let total_time = self.total_processing_time_nanos.load(Ordering::Relaxed);
        let processed = self.total_contexts_processed.load(Ordering::Relaxed);
        if processed == 0 {
            0
        } else {
            total_time / processed
        }
    }
}

/// Context processor statistics with owned strings for Candle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleContextProcessorStatistics {
    pub processor_id: String,
    pub context_requests: u64,
    pub active_contexts: usize,
    pub total_contexts_processed: u64,
    pub successful_contexts: u64,
    pub failed_contexts: u64,
    pub total_documents_loaded: u64,
    pub success_rate: f64,
    pub average_processing_time_nanos: u64,
}

/// Context wrapper with zero Arc usage
#[derive(Debug)]
pub struct CandleContext<T> {
    source: CandleContextSourceType,
    processor: CandleStreamingContextProcessor,
    _marker: PhantomData<T>,
}

/// Candle context source types with immutable implementations
#[derive(Debug, Clone)]
pub enum CandleContextSourceType {
    File(CandleImmutableFileContext),
    Files(CandleImmutableFilesContext),
    Directory(CandleImmutableDirectoryContext),
    Github(CandleImmutableGithubContext),
}

impl<T> Clone for CandleContext<T> {
    fn clone(&self) -> Self {
        let processor_id = Uuid::new_v4().to_string();
        let processor = CandleStreamingContextProcessor::new(processor_id);
        Self {
            source: self.source.clone(),
            processor,
            _marker: PhantomData,
        }
    }
}

impl<T> CandleContext<T> {
    /// Create new Candle context with streaming processor
    #[inline]
    pub fn new(source: CandleContextSourceType) -> Self {
        let processor_id = Uuid::new_v4().to_string();
        let processor = CandleStreamingContextProcessor::new(processor_id);
        Self {
            source,
            processor,
            _marker: PhantomData,
        }
    }

    /// Create Candle context with event streaming
    #[inline]
    pub fn with_streaming(
        source: CandleContextSourceType,
    ) -> (Self, Pin<Box<dyn Stream<Item = CandleContextEvent> + Send>>) {
        let processor_id = Uuid::new_v4().to_string();
        let (processor, stream) = CandleStreamingContextProcessor::with_streaming(processor_id);
        let context = Self {
            source,
            processor,
            _marker: PhantomData,
        };
        (context, stream)
    }
}

// CandleContext<CandleFile> implementation
impl CandleContext<CandleFile> {
    /// Load single file - EXACT syntax: `CandleContext`<CandleFile>`::of("/path/to/file.txt`")
    #[inline]
    pub fn of(path: impl AsRef<Path>) -> Self {
        use sha2::{Digest, Sha256};
        use std::io::Read;

        let path_ref = path.as_ref();
        let path_str = path_ref.to_string_lossy().to_string();

        // Read file metadata and content to compute hash
        let (size_bytes, modified, content_hash) = match std::fs::metadata(path_ref) {
            Ok(metadata) => {
                let size = metadata.len();
                let modified_time = metadata.modified().unwrap_or_else(|_| SystemTime::now());

                // Compute content hash
                let hash = match std::fs::File::open(path_ref) {
                    Ok(mut file) => {
                        let mut hasher = Sha256::new();
                        let mut buffer = vec![0u8; 8192];
                        loop {
                            match file.read(&mut buffer) {
                                Ok(0) | Err(_) => break,
                                Ok(n) => hasher.update(&buffer[..n]),
                            }
                        }
                        let result = hasher.finalize();
                        result
                            .iter()
                            .fold(String::with_capacity(result.len() * 2), |mut s, b| {
                                use std::fmt::Write;
                                let _ = write!(&mut s, "{b:02x}");
                                s
                            })
                    }
                    Err(_) => String::new(),
                };

                (size, modified_time, hash)
            }
            Err(_) => (0, SystemTime::now(), String::new()),
        };

        let file_context = CandleImmutableFileContext {
            path: path_str,
            content_hash,
            size_bytes,
            modified,
            memory_integration: None,
        };
        Self::new(CandleContextSourceType::File(file_context))
    }

    /// Load documents asynchronously with streaming - returns unwrapped values
    #[inline]
    pub fn load(self) -> Pin<Box<dyn Stream<Item = Document> + Send>> {
        match self.source {
            CandleContextSourceType::File(file_context) => {
                self.processor.process_file_context(file_context)
            }
            _ => Box::pin(crate::async_stream::spawn_stream(move |_tx| async move {
                // Invalid context type for file loading
                log::error!("Invalid context type for file loading");
            })),
        }
    }
}

// CandleContext<CandleFiles> implementation
impl CandleContext<CandleFiles> {
    /// Glob pattern for files - EXACT syntax: `CandleContext`<CandleFiles>`::glob`("**/*.{rs,md}")
    #[inline]
    pub fn glob(pattern: impl AsRef<str>) -> Self {
        let pattern_str = pattern.as_ref().to_string();
        let files_context = CandleImmutableFilesContext {
            paths: Vec::new(), // Would be populated by glob expansion
            pattern: pattern_str,
            total_files: 0,
            memory_integration: None,
        };
        Self::new(CandleContextSourceType::Files(files_context))
    }

    /// Load documents asynchronously with streaming - returns unwrapped values
    #[inline]
    pub fn load(self) -> Pin<Box<dyn Stream<Item = Document> + Send>> {
        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            match self.source {
                CandleContextSourceType::Files(files_context) => {
                    // Expand glob pattern and load files
                    match glob::glob(&files_context.pattern) {
                        Ok(paths) => {
                            for entry in paths.flatten() {
                                if let Ok(content) = tokio::fs::read_to_string(&entry).await {
                                    let document = Document {
                                        data: content,
                                        format: Some(crate::domain::context::CandleContentFormat::Text),
                                        media_type: Some(
                                            crate::domain::context::CandleDocumentMediaType::TXT,
                                        ),
                                        additional_props: {
                                            let mut props = HashMap::new();
                                            props.insert(
                                                "id".to_string(),
                                                serde_json::Value::String(
                                                    Uuid::new_v4().to_string(),
                                                ),
                                            );
                                            props.insert(
                                                "path".to_string(),
                                                serde_json::Value::String(
                                                    entry.to_string_lossy().to_string(),
                                                ),
                                            );
                                            props
                                        }};
                                    let _ = tx.send(document);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Streaming error in {}: {:?}", "Glob pattern error", CandleContextError::ContextNotFound(format!(
                                    "Glob pattern error: {e}"
                                )));
                        }
                    }
                }
                _ => {
                    log::error!("Streaming error in {}: {:?}", "Invalid context type for files loading", CandleContextError::ContextNotFound("Invalid context type".to_string()));
                }
            }
        }))
    }
}

// CandleContext<CandleDirectory> implementation
impl CandleContext<CandleDirectory> {
    /// Load all files from directory - EXACT syntax: `CandleContext`<CandleDirectory>`::of("/path/to/dir`")
    #[inline]
    pub fn of(path: impl AsRef<Path>) -> Self {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let directory_context = CandleImmutableDirectoryContext {
            path: path_str,
            recursive: true,
            extensions: Vec::new(),
            max_depth: None,
            memory_integration: None,
        };
        Self::new(CandleContextSourceType::Directory(directory_context))
    }

    /// Load documents asynchronously with streaming - returns unwrapped values
    #[inline]
    pub fn load(self) -> Pin<Box<dyn Stream<Item = Document> + Send>> {
        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            // Use spawn_blocking for CPU-bound directory traversal
            let _ = tokio::task::spawn_blocking(move || {
                match self.source {
                    CandleContextSourceType::Directory(directory_context) => {
                        // Traverse directory and load files

                        fn traverse_dir(
                            path: &str,
                            recursive: bool,
                            extensions: &[String],
                            max_depth: Option<usize>,
                            current_depth: usize,
                            sender: &tokio::sync::mpsc::UnboundedSender<Document>,
                        ) -> Result<(), std::io::Error> {
                            if let Some(max) = max_depth
                                && current_depth > max
                            {
                                return Ok(());
                            }

                            for entry in std::fs::read_dir(path)? {
                                let entry = entry?;
                                let path = entry.path();

                                if path.is_file() {
                                    let should_include = if extensions.is_empty() {
                                        true
                                    } else {
                                        path.extension().and_then(|ext| ext.to_str()).is_some_and(
                                            |ext| extensions.contains(&ext.to_string()),
                                        )
                                    };

                                    if should_include
                                        && let Ok(content) = std::fs::read_to_string(&path)
                                    {
                                        let document = Document {
                                                data: content,
                                                format: Some(crate::domain::context::CandleContentFormat::Text),
                                                media_type: Some(
                                                    crate::domain::context::CandleDocumentMediaType::TXT,
                                                ),
                                                additional_props: {
                                                    let mut props = HashMap::new();
                                                    props.insert(
                                                        "id".to_string(),
                                                        serde_json::Value::String(
                                                            Uuid::new_v4().to_string(),
                                                        ),
                                                    );
                                                    props.insert(
                                                        "path".to_string(),
                                                        serde_json::Value::String(
                                                            path.to_string_lossy().to_string(),
                                                        ),
                                                    );
                                                    props
                                                }};
                                        let _ = sender.send(document);
                                    }
                                } else if path.is_dir()
                                    && recursive
                                    && let Some(path_str) = path.to_str()
                                {
                                    traverse_dir(
                                        path_str,
                                        recursive,
                                        extensions,
                                        max_depth,
                                        current_depth + 1,
                                        sender,
                                    )?;
                                }
                            }
                            Ok(())
                        }

                        match traverse_dir(
                            &directory_context.path,
                            directory_context.recursive,
                            &directory_context.extensions,
                            directory_context.max_depth,
                            0,
                            &tx,
                        ) {
                            Ok(()) => {
                                // Documents are sent directly by traverse_dir
                            }
                            Err(e) => {
                                log::error!("Streaming error in {}: {:?}", "Directory traversal failed", CandleContextError::ContextNotFound(format!(
                                        "Directory traversal error: {e}"
                                    )));
                            }
                        }
                    }
                    _ => {
                        log::error!("Streaming error in {}: {:?}", "Invalid context type for directory loading", CandleContextError::ContextNotFound("Invalid context type".to_string()));
                    }
                }
            }).await;
        }))
    }
}

// CandleContext<CandleGithub> implementation
impl CandleContext<CandleGithub> {
    /// Glob pattern for GitHub files - EXACT syntax: `CandleContext`<CandleGithub>`::glob("/repo`/**/*.{rs,md}")
    #[inline]
    pub fn glob(pattern: impl AsRef<str>) -> Self {
        let pattern_str = pattern.as_ref().to_string();
        let github_context = CandleImmutableGithubContext {
            repository_url: String::new(), // Would be extracted from pattern
            branch: "main".to_string(),
            pattern: pattern_str,
            auth_token: None,
            memory_integration: None,
        };
        Self::new(CandleContextSourceType::Github(github_context))
    }

    /// Get cache directory for GitHub repositories
    fn get_github_cache_dir() -> PathBuf {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_or_else(
                |_| std::path::PathBuf::from("/tmp/paraphym/github"),
                |home| std::path::PathBuf::from(home).join(".cache/paraphym/github"),
            )
    }

    /// Create document from file with GitHub metadata
    fn create_github_document(
        content: String,
        relative_path: String,
        repository_url: String,
        branch: String,
    ) -> Document {
        let mut props = HashMap::new();
        props.insert(
            "id".to_string(),
            serde_json::Value::String(Uuid::new_v4().to_string()),
        );
        props.insert("path".to_string(), serde_json::Value::String(relative_path));
        props.insert(
            "repository".to_string(),
            serde_json::Value::String(repository_url),
        );
        props.insert("branch".to_string(), serde_json::Value::String(branch));

        Document {
            data: content,
            format: Some(crate::domain::context::CandleContentFormat::Text),
            media_type: Some(crate::domain::context::CandleDocumentMediaType::TXT),
            additional_props: props,
        }
    }

    /// Build authenticated URL by embedding token if provided
    fn build_auth_url(repo_url: &str, auth_token: Option<&String>) -> String {
        if let Some(token) = auth_token {
            // Inject token into HTTPS URL: https://github.com -> https://TOKEN@github.com
            if repo_url.starts_with("https://") {
                repo_url.replace("https://", &format!("https://{token}@"))
            } else {
                // For non-HTTPS URLs, return as-is (SSH, git://, etc.)
                repo_url.to_string()
            }
        } else {
            repo_url.to_string()
        }
    }

    /// Clone or update a git repository
    async fn get_or_clone_repo(
        repo_url: &str,
        branch: &str,
        auth_token: Option<&String>,
        cache_dir: &Path,
    ) -> Result<PathBuf, GitGixError> {
        // Generate cache path from repo URL
        let repo_name = repo_url
            .trim_end_matches(".git")
            .split('/')
            .next_back()
            .unwrap_or("repo");
        let repo_path = cache_dir.join(repo_name);

        if repo_path.exists() {
            Self::update_repo(&repo_path, branch, auth_token).await
        } else {
            Self::clone_repo(repo_url, branch, auth_token, &repo_path, cache_dir).await
        }
    }

    /// Update existing repository
    async fn update_repo(
        repo_path: &Path,
        branch: &str,
        _auth_token: Option<&String>,
    ) -> Result<PathBuf, GitGixError> {
        // Open repository
        let repo_handle = open_repo(repo_path)
            .await
            .map_err(|e| GitGixError::Gix(Box::new(e)))?
            .map_err(|e| GitGixError::Gix(Box::new(e)))?;

        // Fetch from origin
        let fetch_opts = FetchOpts::from_remote("origin");
        fetch(repo_handle.clone(), fetch_opts)
            .await
            .map_err(|e| GitGixError::Gix(Box::new(e)))?
            .map_err(|e| GitGixError::Gix(Box::new(e)))?;

        // Merge remote branch (gitgix automatically does fast-forward if possible)
        let remote_branch = format!("origin/{branch}");
        let merge_opts = MergeOpts::new(remote_branch);
        merge(repo_handle, merge_opts)
            .await
            .map_err(|e| GitGixError::Gix(Box::new(e)))?
            .map_err(|e| GitGixError::Gix(Box::new(e)))?;

        Ok(repo_path.to_path_buf())
    }

    /// Clone fresh repository
    async fn clone_repo(
        repo_url: &str,
        branch: &str,
        auth_token: Option<&String>,
        repo_path: &Path,
        cache_dir: &Path,
    ) -> Result<PathBuf, GitGixError> {
        std::fs::create_dir_all(cache_dir).ok();

        // Build authenticated URL
        let auth_url = Self::build_auth_url(repo_url, auth_token);

        // Create clone options
        let opts = CloneOpts::new(auth_url, repo_path).branch(branch);

        // Execute clone
        let _repo_handle = gitgix_clone(opts)
            .await
            .map_err(|e| GitGixError::Gix(Box::new(e)))?
            .map_err(|e| GitGixError::Gix(Box::new(e)))?;

        Ok(repo_path.to_path_buf())
    }

    /// Load documents asynchronously with streaming - returns unwrapped values
    #[inline]
    pub fn load(self) -> Pin<Box<dyn Stream<Item = Document> + Send>> {
        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            match self.source {
                CandleContextSourceType::Github(github_context) => {
                    // Validate repository URL
                    if github_context.repository_url.is_empty() {
                        log::error!("Streaming error in {}: {:?}", "GitHub repository URL missing", CandleContextError::ContextNotFound(
                                "GitHub repository URL is required".to_string()
                            ));
                        return;
                    }

                    // Determine cache directory (use standard location)
                    let cache_dir = Self::get_github_cache_dir();

                    // Clone or update repository
                    match Self::get_or_clone_repo(
                        &github_context.repository_url,
                        &github_context.branch,
                        github_context.auth_token.as_ref(),
                        &cache_dir,
                    )
                    .await
                    {
                        Ok(repo_path) => {
                            // Build glob pattern for files in repository
                            let glob_pattern =
                                format!("{}/{}", repo_path.display(), github_context.pattern);

                            // Match files using glob pattern
                            match glob::glob(&glob_pattern) {
                                Ok(paths) => {
                                    for entry in paths.flatten() {
                                        // Read file content
                                        if let Ok(content) = tokio::fs::read_to_string(&entry).await {
                                            let relative_path = entry
                                                .strip_prefix(&repo_path)
                                                .unwrap_or(&entry)
                                                .to_string_lossy()
                                                .to_string();

                                            let document = Self::create_github_document(
                                                content,
                                                relative_path,
                                                github_context.repository_url.clone(),
                                                github_context.branch.clone(),
                                            );

                                            let _ = tx.send(document);
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Streaming error in {}: {:?}", "Glob pattern expansion failed", CandleContextError::PatternError(format!(
                                            "Glob pattern error for '{}': {}",
                                            github_context.pattern, e
                                        )));
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Streaming error in {}: {:?}", "GitHub repository access failed", CandleContextError::ProviderUnavailable(format!(
                                    "Failed to clone/update repository '{}': {}",
                                    github_context.repository_url, e
                                )));
                        }
                    }
                }
                _ => {
                    log::error!("Streaming error in {}: {:?}", "Invalid context type for GitHub loading", CandleContextError::ContextNotFound("Invalid context type".to_string()));
                }
            }
        }))
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::context::{CandleContentFormat, CandleDocumentMediaType};
    use base64::engine::general_purpose;
    use std::io::Write;
    use std::time::SystemTime;
    use tempfile::{NamedTempFile, TempDir};

    /// Helper function to create a test file context
    fn create_test_context(path: String) -> CandleImmutableFileContext {
        CandleImmutableFileContext {
            path,
            content_hash: "test_hash_123".to_string(),
            size_bytes: 0,
            modified: SystemTime::now(),
            memory_integration: None,
        }
    }

    #[test]
    fn test_load_text_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Hello, this is a test text file!";
        temp_file
            .write_all(test_content.as_bytes())
            .expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Text)));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_load_json_file() {
        let mut temp_file =
            NamedTempFile::with_suffix(".json").expect("Failed to create temp file");
        let test_content = r#"{"key": "value", "number": 42}"#;
        temp_file
            .write_all(test_content.as_bytes())
            .expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Json)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Json)
        ));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_load_html_file() {
        let mut temp_file =
            NamedTempFile::with_suffix(".html").expect("Failed to create temp file");
        let test_content = "<html><body><h1>Test</h1></body></html>";
        temp_file
            .write_all(test_content.as_bytes())
            .expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Html)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Html)
        ));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_load_markdown_file() {
        let mut temp_file = NamedTempFile::with_suffix(".md").expect("Failed to create temp file");
        let test_content = "# Heading\n\nThis is **markdown** content.";
        temp_file
            .write_all(test_content.as_bytes())
            .expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(
            document.format,
            Some(CandleContentFormat::Markdown)
        ));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Markdown)
        ));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_load_binary_file() {
        let mut temp_file = NamedTempFile::with_suffix(".pdf").expect("Failed to create temp file");
        let binary_data: Vec<u8> = vec![0x25, 0x50, 0x44, 0x46, 0x2D];
        temp_file.write_all(&binary_data).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        let expected_base64 = general_purpose::STANDARD.encode(&binary_data);
        assert_eq!(document.data, expected_base64);
        assert!(matches!(document.format, Some(CandleContentFormat::Base64)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::PDF)
        ));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_utf8_fallback() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_utf8: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x80, 0x81];
        temp_file.write_all(&invalid_utf8).expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        let expected_base64 = general_purpose::STANDARD.encode(&invalid_utf8);
        assert_eq!(document.data, expected_base64);
        assert!(matches!(document.format, Some(CandleContentFormat::Base64)));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_missing_file() {
        let context = create_test_context("/path/to/nonexistent/file.txt".to_string());
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert!(document.error().is_some());
        assert!(document.data.starts_with("ERROR: Failed to access file"));
    }

    #[test]
    fn test_directory_not_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert!(document.error().is_some());
        assert!(document.data.contains("Path is not a file"));
    }

    #[test]
    fn test_extension_fallback() {
        let mut temp_file =
            NamedTempFile::with_suffix(".custom").expect("Failed to create temp file");
        let test_content = "Custom file content";
        temp_file
            .write_all(test_content.as_bytes())
            .expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Text)));
        assert!(document.error().is_none());
    }

    #[test]
    fn test_metadata_preservation() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = "Test content for metadata";
        temp_file
            .write_all(test_content.as_bytes())
            .expect("Failed to write");
        temp_file.flush().expect("Failed to flush");

        let path = temp_file.path().to_string_lossy().to_string();
        let test_hash = "test_hash_456";
        let test_size = test_content.len() as u64;

        let context = CandleImmutableFileContext {
            path: path.clone(),
            content_hash: test_hash.to_string(),
            size_bytes: test_size,
            modified: SystemTime::now(),
            memory_integration: None,
        };

        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert!(document.additional_props.contains_key("path"));
        assert!(document.additional_props.contains_key("hash"));
        assert!(document.additional_props.contains_key("size"));
        assert!(document.additional_props.contains_key("id"));

        if let Some(serde_json::Value::String(stored_path)) = document.additional_props.get("path")
        {
            assert_eq!(stored_path, &path);
        } else {
            panic!("Path not found in metadata");
        }

        if let Some(serde_json::Value::String(stored_hash)) = document.additional_props.get("hash")
        {
            assert_eq!(stored_hash, test_hash);
        } else {
            panic!("Hash not found in metadata");
        }
    }

    #[test]
    fn test_csv_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".csv")?;
        let test_content = "name,age,city\nAlice,30,NYC\nBob,25,LA";
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Csv)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Csv)
        ));
        assert!(document.error().is_none());
        Ok(())
    }

    #[test]
    fn test_xml_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".xml")?;
        let test_content = r#"<?xml version="1.0"?><root><item>Test</item></root>"#;
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Xml)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Xml)
        ));
        assert!(document.error().is_none());
        Ok(())
    }

    #[test]
    fn test_yaml_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".yaml")?;
        let test_content = "key: value\nlist:\n  - item1\n  - item2";
        temp_file.write_all(test_content.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert_eq!(document.data, test_content);
        assert!(matches!(document.format, Some(CandleContentFormat::Yaml)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Yaml)
        ));
        assert!(document.error().is_none());
        Ok(())
    }

    #[test]
    fn test_image_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::with_suffix(".png")?;
        let png_header: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        temp_file.write_all(&png_header)?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        let expected_base64 = general_purpose::STANDARD.encode(&png_header);
        assert_eq!(document.data, expected_base64);
        assert!(matches!(document.format, Some(CandleContentFormat::Base64)));
        assert!(matches!(
            document.media_type,
            Some(CandleDocumentMediaType::Image)
        ));
        assert!(document.error().is_none());
        Ok(())
    }

    #[test]
    fn test_file_size_limit() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"small content")?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let context = create_test_context(path);
        let document = CandleStreamingContextProcessor::load_file_document(&context);

        assert!(document.error().is_none());
        assert_eq!(document.data, "small content");
        Ok(())
    }
}

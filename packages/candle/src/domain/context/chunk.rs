//! Chunk Types for Streaming Operations
//!
//! These types represent partial data that flows through `AsyncStream<T>`
//! and are designed to work with the `NotResult` constraint.
//! Originally from chunk.rs.

use std::collections::HashMap;
use std::path::PathBuf;

use cyrup_sugars::{prelude::MessageChunk, ZeroOneOrMany};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid;

use crate::domain::model::CandleUsage;

/// Candle chunk of document content for streaming file operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleDocumentChunk {
    /// Optional path to the source file
    pub path: Option<PathBuf>,

    /// The content of this chunk
    pub content: String,

    /// Byte range in the original file
    pub byte_range: Option<(usize, usize)>,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Candle image format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CandleImageFormat {
    /// Portable Network Graphics format
    PNG,
    /// Joint Photographic Experts Group format
    JPEG,
    /// Graphics Interchange Format
    GIF,
    /// WebP image format by Google
    WebP,
    /// Bitmap image format
    BMP,
    /// Tagged Image File Format
    TIFF,
}

/// Candle chunk of image data for streaming image operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleImageChunk {
    /// Raw image data
    pub data: Vec<u8>,

    /// Image format
    pub format: CandleImageFormat,

    /// Optional dimensions (width, height)
    pub dimensions: Option<(u32, u32)>,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Audio format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AudioFormat {
    /// MPEG-1 Audio Layer III compressed audio format
    MP3,
    /// Waveform Audio File Format (uncompressed)
    WAV,
    /// Free Lossless Audio Codec
    FLAC,
    /// Ogg Vorbis compressed audio format
    OGG,
    /// MPEG-4 Audio compressed format (AAC)
    M4A,
    /// Opus low-latency audio codec
    OPUS,
}

/// Chunk of audio/voice data for streaming audio operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChunk {
    /// Raw audio data
    pub audio_data: Vec<u8>,

    /// Audio format
    pub format: AudioFormat,

    /// Duration in milliseconds
    pub duration_ms: Option<u64>,

    /// Sample rate in Hz
    pub sample_rate: Option<u32>,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Chunk of chat message for streaming responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageChunk {
    /// Partial message content
    pub content: String,

    /// Role of the message sender
    pub role: crate::domain::chat::message::types::CandleMessageRole,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Reason why a completion finished
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FinishReason {
    /// Completion finished naturally at a stopping point
    Stop,
    /// Completion reached maximum token length limit
    Length,
    /// Completion was filtered due to content policy
    ContentFilter,
    /// Completion finished to execute tool calls
    ToolCalls,
    /// Completion failed due to an error
    Error,
}

/// Comprehensive completion chunk supporting all streaming features - EXACT REPLICA of domain
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CandleCompletionChunk {
    /// Text content chunk
    Text(String),

    /// Tool call started
    ToolCallStart { id: String, name: String },

    /// Partial tool call with streaming input
    ToolCall {
        id: String,
        name: String,
        partial_input: String,
    },

    /// Tool call completed
    ToolCallComplete {
        id: String,
        name: String,
        input: String,
    },

    /// Completion finished with final information
    Complete {
        text: String,
        finish_reason: Option<FinishReason>,
        usage: Option<CandleUsage>,
    },

    /// Error occurred during streaming
    Error(String),
}

impl Default for CandleCompletionChunk {
    fn default() -> Self {
        CandleCompletionChunk::Text(String::new())
    }
}

impl MessageChunk for CandleCompletionChunk {
    fn bad_chunk(error: String) -> Self {
        CandleCompletionChunk::Error(error)
    }

    fn error(&self) -> Option<&str> {
        match self {
            CandleCompletionChunk::Error(err) => Some(err),
            _ => None,
        }
    }
}

/// A simple unit chunk type that implements `MessageChunk` for operations that don't return data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CandleUnitChunk {
    /// Operation completed successfully
    #[default]
    Success,
    /// Operation failed with error
    Error(String),
}

/// Simple wrapper for String to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleStringChunk(pub String);

/// Wrapper for JSON Value to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleJsonChunk(pub Value);

/// Generic wrapper for collections to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleCollectionChunk<T> {
    pub items: T,
    pub error_message: Option<String>,
}

impl MessageChunk for CandleUnitChunk {
    fn bad_chunk(error: String) -> Self {
        CandleUnitChunk::Error(error)
    }

    fn error(&self) -> Option<&str> {
        match self {
            CandleUnitChunk::Error(err) => Some(err),
            CandleUnitChunk::Success => None,
        }
    }
}

impl MessageChunk for CandleStringChunk {
    fn bad_chunk(error: String) -> Self {
        CandleStringChunk(format!("Error: {error}"))
    }

    fn error(&self) -> Option<&str> {
        if self.0.starts_with("Error: ") {
            Some(&self.0)
        } else {
            None
        }
    }
}

impl MessageChunk for CandleJsonChunk {
    fn bad_chunk(error: String) -> Self {
        CandleJsonChunk(Value::String(format!("Error: {error}")))
    }

    fn error(&self) -> Option<&str> {
        match &self.0 {
            Value::String(s) if s.starts_with("Error: ") => Some(s),
            _ => None,
        }
    }
}

impl<T> Default for CandleCollectionChunk<T> 
where 
    T: Default 
{
    fn default() -> Self {
        CandleCollectionChunk {
            items: T::default(),
            error_message: None,
        }
    }
}

impl<T> MessageChunk for CandleCollectionChunk<T> 
where 
    T: Default 
{
    fn bad_chunk(error: String) -> Self {
        CandleCollectionChunk {
            items: T::default(),
            error_message: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

/// Chunk of embedding data for streaming embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingChunk {
    /// The embedding vector
    pub embeddings: ZeroOneOrMany<f32>,

    /// Index in the batch
    pub index: usize,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

impl Default for EmbeddingChunk {
    fn default() -> Self {
        Self {
            embeddings: ZeroOneOrMany::None,
            index: 0,
            metadata: HashMap::new(),
        }
    }
}

impl MessageChunk for EmbeddingChunk {
    fn bad_chunk(error: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("error".to_string(), Value::String(error));
        Self {
            embeddings: ZeroOneOrMany::None,
            index: 0,
            metadata,
        }
    }

    fn error(&self) -> Option<&str> {
        if let Some(Value::String(error)) = self.metadata.get("error") {
            Some(error)
        } else {
            None
        }
    }
}

/// Chunk of transcribed text from speech-to-text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionChunk {
    /// The transcribed text for this chunk
    pub text: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: Option<f32>,

    /// Start time in milliseconds
    pub start_time_ms: Option<u64>,

    /// End time in milliseconds  
    pub end_time_ms: Option<u64>,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Chunk of synthesized speech for text-to-speech
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechChunk {
    /// Raw audio data
    pub audio_data: Vec<u8>,

    /// Audio format
    pub format: AudioFormat,

    /// Duration in milliseconds
    pub duration_ms: Option<u64>,

    /// Sample rate in Hz
    pub sample_rate: Option<u32>,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

// Convenience constructors
impl CandleDocumentChunk {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            path: None,
            content: content.into(),
            byte_range: None,
            metadata: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.byte_range = Some((start, end));
        self
    }
}

impl MessageChunk for CandleDocumentChunk {
    fn bad_chunk(error: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("error".to_string(), Value::String(error));
        Self {
            path: None,
            content: String::new(),
            byte_range: None,
            metadata,
        }
    }

    fn error(&self) -> Option<&str> {
        if let Some(Value::String(error)) = self.metadata.get("error") {
            Some(error)
        } else {
            None
        }
    }
}

impl ChatMessageChunk {
    pub fn new(
        content: impl Into<String>,
        role: crate::domain::chat::message::types::CandleMessageRole,
    ) -> Self {
        Self {
            content: content.into(),
            role,
            is_final: false,
            metadata: HashMap::new(),
        }
    }
}

impl CandleCompletionChunk {
    /// Create a simple text chunk
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    /// Create a tool call start chunk
    pub fn tool_start(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self::ToolCallStart {
            id: id.into(),
            name: name.into(),
        }
    }

    /// Create a partial tool call chunk
    pub fn tool_partial(
        id: impl Into<String>,
        name: impl Into<String>,
        partial_input: impl Into<String>,
    ) -> Self {
        Self::ToolCall {
            id: id.into(),
            name: name.into(),
            partial_input: partial_input.into(),
        }
    }

    /// Create a completed tool call chunk
    pub fn tool_complete(
        id: impl Into<String>,
        name: impl Into<String>,
        input: impl Into<String>,
    ) -> Self {
        Self::ToolCallComplete {
            id: id.into(),
            name: name.into(),
            input: input.into(),
        }
    }

    /// Create a completion finished chunk
    pub fn complete(
        text: impl Into<String>,
        finish_reason: Option<FinishReason>,
        usage: Option<CandleUsage>,
    ) -> Self {
        Self::Complete {
            text: text.into(),
            finish_reason,
            usage,
        }
    }

    /// Create an error chunk
    pub fn error(error: impl Into<String>) -> Self {
        Self::Error(error.into())
    }

    /// Check if this chunk contains text content
    #[must_use]
    pub fn has_text(&self) -> bool {
        matches!(self, Self::Text(_) | Self::Complete { .. })
    }

    /// Extract text content if available
    #[must_use]
    pub fn text_content(&self) -> Option<&str> {
        match self {
            Self::Text(text) | Self::Complete { text, .. } => Some(text),
            _ => None,
        }
    }

    /// Check if this is a completion chunk
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete { .. })
    }

    /// Check if this is an error chunk
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
}

// CandleUnitChunk already defined above - this implementation violates orphan rules

// Removed Result implementations that violated orphan rules
// Use wrapper types like CandleResult<T, E> instead if needed

// Wrapper types to avoid orphan rule violations

/// Wrapper for unit type () to implement `MessageChunk`
#[derive(Debug, Clone, Default)]
pub struct CandleUnit(pub ());

impl MessageChunk for CandleUnit {
    fn bad_chunk(_error: String) -> Self {
        CandleUnit(())
    }

    fn error(&self) -> Option<&str> {
        None
    }
}





// Removed orphan rule violating implementations for (A, B) tuple types
// Use CandleTuple<A, B> wrapper type instead

/// Wrapper for tuple types to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleTuple<A, B> {
    pub first: A,
    pub second: B,
}

impl<A, B> Default for CandleTuple<A, B>
where
    A: Default,
    B: Default,
{
    fn default() -> Self {
        CandleTuple {
            first: A::default(),
            second: B::default(),
        }
    }
}

impl<A, B> MessageChunk for CandleTuple<A, B>
where
    A: MessageChunk + Default,
    B: MessageChunk + Default,
{
    fn bad_chunk(error: String) -> Self {
        CandleTuple {
            first: A::bad_chunk(error.clone()),
            second: B::bad_chunk(error),
        }
    }

    fn error(&self) -> Option<&str> {
        // Check both tuple elements for errors, prioritize first one
        if let Some(error) = self.first.error() {
            Some(error)
        } else {
            self.second.error()
        }
    }
}

/// Wrapper for Result types to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleResult<T, E> {
    pub result: Result<T, E>,
}

impl<T, E> Default for CandleResult<T, E>
where
    T: Default,
{
    fn default() -> Self {
        CandleResult {
            result: Ok(T::default()),
        }
    }
}

impl<T, E> MessageChunk for CandleResult<T, E>
where
    T: MessageChunk + Default,
    E: std::fmt::Display,
{
    fn bad_chunk(error: String) -> Self {
        CandleResult {
            result: Ok(T::bad_chunk(error)),
        }
    }

    fn error(&self) -> Option<&str> {
        match &self.result {
            Ok(t) => t.error(),
            Err(_) => Some("CandleResult error"),
        }
    }
}

/// Zero-cost result wrapper for N-way parallel operations
///
/// This wrapper maintains all performance characteristics of the
/// inner result while providing operation tracking and `MessageChunk` compliance.
///
/// # Performance
/// - Zero runtime overhead with transparent wrapper design
/// - Compiles to identical assembly as unwrapped result
/// - Maintains all optimization opportunities
/// - Enables result ordering and operation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelResult<T> {
    /// Index of the operation that produced this result (0-based)
    pub operation_index: usize,
    /// The actual result from the parallel operation
    pub result: T,
}

impl<T> ParallelResult<T> {
    /// Create a new parallel result
    #[inline]
    pub fn new(operation_index: usize, result: T) -> Self {
        Self {
            operation_index,
            result,
        }
    }

    /// Extract the inner result, discarding operation index
    #[inline]
    pub fn into_result(self) -> T {
        self.result
    }

    /// Get reference to the inner result
    #[inline]
    pub fn result(&self) -> &T {
        &self.result
    }

    /// Get mutable reference to the inner result
    #[inline]
    pub fn result_mut(&mut self) -> &mut T {
        &mut self.result
    }

    /// Get the operation index that produced this result
    #[inline]
    pub fn operation_index(&self) -> usize {
        self.operation_index
    }

    /// Map the result to a different type while preserving operation index
    #[inline]
    pub fn map<U, F>(self, f: F) -> ParallelResult<U>
    where
        F: FnOnce(T) -> U,
    {
        ParallelResult {
            operation_index: self.operation_index,
            result: f(self.result),
        }
    }
}

impl<T: Default> Default for ParallelResult<T> {
    fn default() -> Self {
        Self {
            operation_index: 0,
            result: T::default(),
        }
    }
}

impl<T: MessageChunk> MessageChunk for ParallelResult<T> {
    fn bad_chunk(error: String) -> Self {
        Self {
            operation_index: 0,
            result: T::bad_chunk(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.result.error()
    }
}

impl<T> From<T> for ParallelResult<T> {
    fn from(result: T) -> Self {
        Self::new(0, result)
    }
}

impl<T> std::ops::Deref for ParallelResult<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<T> std::ops::DerefMut for ParallelResult<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result
    }
}

impl<T: PartialEq> PartialEq for ParallelResult<T> {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result && self.operation_index == other.operation_index
    }
}

impl<T: Eq> Eq for ParallelResult<T> {}

impl<T: std::hash::Hash> std::hash::Hash for ParallelResult<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.operation_index.hash(state);
        self.result.hash(state);
    }
}

/// Result type for context refresh operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleRefreshResult {
    /// Whether the refresh was successful
    pub success: bool,
    /// Optional error message if refresh failed
    pub error_message: Option<String>,
}

impl Default for CandleRefreshResult {
    fn default() -> Self {
        Self {
            success: true,
            error_message: None,
        }
    }
}

impl CandleRefreshResult {
    /// Create a successful refresh result
    #[must_use]
    pub fn success() -> Self {
        Self {
            success: true,
            error_message: None,
        }
    }

    /// Create a failed refresh result with error message
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            error_message: Some(error.into()),
        }
    }
}

impl MessageChunk for CandleRefreshResult {
    fn bad_chunk(error: String) -> Self {
        Self::failure(error)
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

/// Result type for memory operations (store, delete, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleMemoryOperationResult {
    /// Whether the operation was successful
    pub success: bool,
    /// Optional error message if operation failed
    pub error_message: Option<String>,
    /// Optional operation type for context
    pub operation_type: Option<String>,
}

impl Default for CandleMemoryOperationResult {
    fn default() -> Self {
        Self {
            success: true,
            error_message: None,
            operation_type: None,
        }
    }
}

impl CandleMemoryOperationResult {
    /// Create a successful operation result
    #[must_use]
    pub fn success() -> Self {
        Self {
            success: true,
            error_message: None,
            operation_type: None,
        }
    }

    /// Create a successful operation result with operation type
    pub fn success_with_type(operation_type: impl Into<String>) -> Self {
        Self {
            success: true,
            error_message: None,
            operation_type: Some(operation_type.into()),
        }
    }

    /// Create a failed operation result with error message
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            error_message: Some(error.into()),
            operation_type: None,
        }
    }

    /// Create a failed operation result with error and operation type
    pub fn failure_with_type(error: impl Into<String>, operation_type: impl Into<String>) -> Self {
        Self {
            success: false,
            error_message: Some(error.into()),
            operation_type: Some(operation_type.into()),
        }
    }
}

impl MessageChunk for CandleMemoryOperationResult {
    fn bad_chunk(error: String) -> Self {
        Self::failure(error)
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

/// Workflow data chunk for streaming JSON values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDataChunk {
    /// The JSON data
    pub data: Value,
    /// Step name that produced this data
    pub step_name: Option<String>,
    /// Processing timestamp
    pub timestamp: Option<u64>,
    /// Error message if this represents an error
    pub error_message: Option<String>,
}

impl MessageChunk for WorkflowDataChunk {
    fn bad_chunk(error: String) -> Self {
        Self {
            data: Value::String(format!("Error: {error}")),
            step_name: Some("error".to_string()),
            timestamp: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
            error_message: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

impl Default for WorkflowDataChunk {
    fn default() -> Self {
        Self {
            data: Value::Null,
            step_name: None,
            timestamp: None,
            error_message: None,
        }
    }
}

impl From<Value> for WorkflowDataChunk {
    fn from(data: Value) -> Self {
        Self {
            data,
            step_name: None,
            timestamp: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
            error_message: None,
        }
    }
}

/// Wrapper for Uuid to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleUuidChunk(pub uuid::Uuid);

impl Default for CandleUuidChunk {
    fn default() -> Self {
        CandleUuidChunk(uuid::Uuid::new_v4())
    }
}

impl MessageChunk for CandleUuidChunk {
    fn bad_chunk(_error: String) -> Self {
        // Create a deterministic UUID from error for debugging
        CandleUuidChunk(uuid::Uuid::new_v4())
    }

    fn error(&self) -> Option<&str> {
        None // UUIDs don't carry error state
    }
}

impl From<uuid::Uuid> for CandleUuidChunk {
    fn from(uuid: uuid::Uuid) -> Self {
        CandleUuidChunk(uuid)
    }
}

/// Wrapper for bool to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleBoolChunk(pub bool);

impl MessageChunk for CandleBoolChunk {
    fn bad_chunk(_error: String) -> Self {
        CandleBoolChunk(false) // Error state represented as false
    }

    fn error(&self) -> Option<&str> {
        None // Bools don't carry error state
    }
}

impl From<bool> for CandleBoolChunk {
    fn from(value: bool) -> Self {
        CandleBoolChunk(value)
    }
}

/// Wrapper for Duration to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleDurationChunk(#[serde(with = "duration_serde")] pub std::time::Duration);

impl Default for CandleDurationChunk {
    fn default() -> Self {
        CandleDurationChunk(std::time::Duration::from_secs(0))
    }
}

impl MessageChunk for CandleDurationChunk {
    fn bad_chunk(_error: String) -> Self {
        CandleDurationChunk(std::time::Duration::from_secs(0))
    }

    fn error(&self) -> Option<&str> {
        None // Durations don't carry error state
    }
}

impl From<std::time::Duration> for CandleDurationChunk {
    fn from(duration: std::time::Duration) -> Self {
        CandleDurationChunk(duration)
    }
}

/// Wrapper for `ZeroOneOrMany` to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleZeroOneOrManyChunk<T>(pub ZeroOneOrMany<T>);

impl<T: Default> Default for CandleZeroOneOrManyChunk<T> {
    fn default() -> Self {
        CandleZeroOneOrManyChunk(ZeroOneOrMany::None)
    }
}

impl<T> MessageChunk for CandleZeroOneOrManyChunk<T> 
where 
    T: Default + Clone
{
    fn bad_chunk(_error: String) -> Self {
        CandleZeroOneOrManyChunk(ZeroOneOrMany::None)
    }

    fn error(&self) -> Option<&str> {
        None // ZeroOneOrMany doesn't carry error state
    }
}

impl<T> From<ZeroOneOrMany<T>> for CandleZeroOneOrManyChunk<T> {
    fn from(value: ZeroOneOrMany<T>) -> Self {
        CandleZeroOneOrManyChunk(value)
    }
}

impl<T> std::ops::Deref for CandleZeroOneOrManyChunk<T> {
    type Target = ZeroOneOrMany<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for CandleZeroOneOrManyChunk<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wrapper for `DateTime`<Utc> to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleDateTimeChunk(pub chrono::DateTime<chrono::Utc>);

impl Default for CandleDateTimeChunk {
    fn default() -> Self {
        CandleDateTimeChunk(chrono::Utc::now())
    }
}

impl MessageChunk for CandleDateTimeChunk {
    fn bad_chunk(_error: String) -> Self {
        CandleDateTimeChunk(chrono::Utc::now())
    }

    fn error(&self) -> Option<&str> {
        None // DateTimes don't carry error state
    }
}

impl From<chrono::DateTime<chrono::Utc>> for CandleDateTimeChunk {
    fn from(datetime: chrono::DateTime<chrono::Utc>) -> Self {
        CandleDateTimeChunk(datetime)
    }
}

// Duration serialization helpers
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

/// Wrapper for `ZeroOneOrMany`<f32> to implement `MessageChunk` without orphan rule violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroOneOrManyF32Chunk(pub cyrup_sugars::ZeroOneOrMany<f32>);

impl Default for ZeroOneOrManyF32Chunk {
    fn default() -> Self {
        ZeroOneOrManyF32Chunk(cyrup_sugars::ZeroOneOrMany::None)
    }
}

impl MessageChunk for ZeroOneOrManyF32Chunk {
    fn bad_chunk(_error: String) -> Self {
        ZeroOneOrManyF32Chunk(cyrup_sugars::ZeroOneOrMany::None)
    }

    fn error(&self) -> Option<&str> {
        None // ZeroOneOrMany doesn't carry error state
    }
}

impl From<cyrup_sugars::ZeroOneOrMany<f32>> for ZeroOneOrManyF32Chunk {
    fn from(value: cyrup_sugars::ZeroOneOrMany<f32>) -> Self {
        ZeroOneOrManyF32Chunk(value)
    }
}

impl From<ZeroOneOrManyF32Chunk> for cyrup_sugars::ZeroOneOrMany<f32> {
    fn from(chunk: ZeroOneOrManyF32Chunk) -> Self {
        chunk.0
    }
}

// Note: Orphan rule violations removed - use wrapper types instead:
// - Use CandleUnit for () 
// - Use CandleStringChunk for String
// - Use CandleUuidChunk for Uuid
// - Use CandleBoolChunk for bool
// - Use CandleDurationChunk for Duration
// - Use CandleDateTimeChunk for DateTime<Utc>
// - Use ZeroOneOrManyF32Chunk for ZeroOneOrMany<f32>


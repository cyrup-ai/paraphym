//! Result Wrapper Types
//!
//! MessageChunk-compliant result wrappers for async operations.

use serde::{Deserialize, Serialize};
use cyrup_sugars::prelude::MessageChunk;

/// Zero-cost result wrapper that implements `MessageChunk` without requiring T to implement it
///
/// This wrapper enables `AsyncStream<Result<T, E>>` to work with the `NotResult` constraint
/// by lifting any `Result` into a `MessageChunk`-compliant wrapper. The key insight is that
/// the wrapper itself implements `MessageChunk`, not the inner type.
///
/// # Performance
/// - Zero runtime overhead with transparent wrapper design
/// - Compiles to identical assembly as unwrapped Result
/// - Maintains all Result methods through Deref/DerefMut
/// - No allocations beyond what Result itself requires
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleResult<T> {
    /// The wrapped result
    pub inner: Result<T, String>,
}

impl<T> CandleResult<T> {
    /// Create a success result
    #[inline]
    pub fn ok(value: T) -> Self {
        Self { inner: Ok(value) }
    }

    /// Create an error result
    #[inline]
    pub fn err(error: impl Into<String>) -> Self {
        Self {
            inner: Err(error.into()),
        }
    }

    /// Convert from a standard Result
    #[inline]
    pub fn from_result(result: Result<T, impl Into<String>>) -> Self {
        Self {
            inner: result.map_err(|e| e.into()),
        }
    }

    /// Extract the inner Result, consuming the wrapper
    #[inline]
    pub fn into_result(self) -> Result<T, String> {
        self.inner
    }

    /// Get a reference to the inner Result
    #[inline]
    pub fn as_result(&self) -> &Result<T, String> {
        &self.inner
    }

    /// Check if this is Ok
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    /// Check if this is Err
    #[inline]
    pub fn is_err(&self) -> bool {
        self.inner.is_err()
    }
}

impl<T: Default> Default for CandleResult<T> {
    fn default() -> Self {
        Self::ok(T::default())
    }
}

impl<T> MessageChunk for CandleResult<T> {
    fn bad_chunk(error: String) -> Self {
        Self::err(error)
    }

    fn error(&self) -> Option<&str> {
        match &self.inner {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }
}

impl<T> From<Result<T, String>> for CandleResult<T> {
    fn from(result: Result<T, String>) -> Self {
        Self { inner: result }
    }
}

impl<T, E: Into<String>> From<Result<T, E>> for CandleResult<T> {
    fn from(result: Result<T, E>) -> Self {
        Self::from_result(result)
    }
}

impl<T> std::ops::Deref for CandleResult<T> {
    type Target = Result<T, String>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for CandleResult<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: PartialEq> PartialEq for CandleResult<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for CandleResult<T> {}

impl<T: std::hash::Hash> std::hash::Hash for CandleResult<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

// Also support extracting errors from nested CandleResult<MessageChunk> types
impl<T: MessageChunk> CandleResult<T> {
    /// Check for errors in both the outer Result and inner MessageChunk
    pub fn nested_error(&self) -> Option<&str> {
        match &self.inner {
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

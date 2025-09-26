//! Core completion traits - EXACT REPLICA of domain with Candle prefixes
//!
//! Contains CandleCompletionModel and CandleCompletionBackend traits that exactly match
//! domain/src/completion/core.rs with zero over-engineering.

use ystream::AsyncStream;

use super::types::CandleCompletionParams;
use crate::domain::completion::CandleCompletionChunk;
use crate::domain::completion::{CandleCompletionRequest, CandleCompletionResponse};
use crate::domain::prompt::CandlePrompt;

/// Core trait for completion models - EXACT REPLICA of domain CompletionModel
pub trait CandleCompletionModel: Send + Sync + 'static {
    /// Generate completion from prompt
    ///
    /// # Arguments
    /// * `prompt` - The input prompt for generation
    /// * `params` - Generation parameters
    ///
    /// # Returns
    /// Stream of completion chunks
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> AsyncStream<CandleCompletionChunk>;
}

/// Backend for completion processing - EXACT REPLICA of domain CompletionBackend
pub trait CandleCompletionBackend: Send + Sync + 'static {
    /// Submit a completion request
    ///
    /// # Arguments
    /// * `request` - The completion request
    ///
    /// # Returns
    /// Async task that resolves to the completion result
    fn submit_completion<'a>(
        &'a self,
        request: CandleCompletionRequest,
    ) -> ystream::AsyncTask<CandleCompletionResponse<'a>>;
}

// Backward compatibility trait alias for existing code
pub trait CandleCompletionProvider: CandleCompletionModel {}

// Blanket implementation
impl<T: CandleCompletionModel> CandleCompletionProvider for T {}

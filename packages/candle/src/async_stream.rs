//! Candle Async Stream - Using existing ystream with Candle type aliases

// Re-export ystream types with Candle prefixes
pub use ystream::{AsyncStream as CandleAsyncStream, AsyncTask as CandleAsyncTask, AsyncStreamSender as CandleAsyncStreamSender, spawn_task as candle_spawn_task};
//! Live message streaming system with zero-allocation patterns
//!
//! This module provides high-performance message streaming using lock-free queues,
//! atomic counters, and tokio Stream patterns for blazing-fast real-time updates.

mod types;
mod subscriber;
mod results;
mod stats;
mod streamer;
mod processing;

// Public API exports
pub use types::{LiveUpdateMessage, MessagePriority};
pub use streamer::LiveMessageStreamer;

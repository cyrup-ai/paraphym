//! Tool Interface
//!
//! This module provides tool routing and execution using `SweetMCP`.
//! Tools are executed via WASM plugins and Cylo backends.
//! Users never directly call tools - they prompt naturally and the LLM
//! decides which tools to call, similar to `OpenAI` function calling.
//!
//! Key components:
//! - `SweetMcpRouter`: Tool routing and execution via WASM/Cylo
//! - OpenAI-style function calling experience
//! - Full `tokio_stream::Stream` compatibility

pub mod router;

// Re-export the SweetMCP router - NEW PREFERRED APPROACH
pub use router::{RouterError, SweetMcpRouter, ToolRoute};

// Re-export SweetMCP types for external compatibility
pub use mcp_client_traits::{McpClient, McpToolOperations};
pub use sweet_mcp_type::ToolInfo;

//! Unified Tool Interface
//!
//! This module provides a single, unified interface for all tool execution
//! that transparently handles both MCP server tools and native code execution.
//! Users never directly call tools - they prompt naturally and the LLM
//! decides which tools to call, similar to `OpenAI` function calling.
//!
//! Key components:
//! - `UnifiedToolExecutor`: Single interface for all tool execution
//! - Automatic routing between MCP servers and Cylo secure execution
//! - OpenAI-style function calling experience
//! - Full `ystream::AsyncStream` compatibility

pub mod unified;
pub mod router;

// Re-export the SweetMCP router - NEW PREFERRED APPROACH
pub use router::{SweetMcpRouter, RouterError, ToolRoute};

// Legacy unified tool interface - DEPRECATED in favor of SweetMcpRouter
pub use unified::{UnifiedToolExecutor, ToolError};

// Re-export SweetMCP types for external compatibility
pub use sweet_mcp_type::ToolInfo;
pub use mcp_client_traits::{McpClient, McpToolOperations};
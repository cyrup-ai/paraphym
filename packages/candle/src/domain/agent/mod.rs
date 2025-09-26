//! Agent domain types and role implementations
//!
//! This module consolidates agent data structures and role definitions with automatic memory tool injection.

pub mod chat;
pub mod core;
pub mod role;
pub mod types;

// Re-export commonly used types with explicit imports to avoid conflicts
pub use role::McpServerConfig as CandleMcpServer;
pub use role::{
    CandleAgentConversation, CandleAgentConversationMessage, CandleAgentRoleAgent, CandleStdio,
};
pub use role::{CandleAgentRole, CandleAgentRoleImpl, McpServerConfig};
pub use types::{AgentConfig, CandleAdditionalParams, CandleAgent};

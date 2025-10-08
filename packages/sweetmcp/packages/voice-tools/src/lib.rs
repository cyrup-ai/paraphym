//! MCP Voice Tools - Tool definitions for voice operations
//!
//! This crate provides the MCP tool definitions for text-to-speech (TTS)
//! and speech-to-text (STT) operations, enabling LLMs to interact with
//! voice capabilities through a clean, intuitive interface.

use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod protocol;
pub mod tools;
pub mod types;

// Re-export commonly used types
pub use error::{VoiceError, VoiceResult};
pub use protocol::{VoiceRequest, VoiceResponse};
pub use tools::{listen_tool, speak_tool};
pub use types::{ListenParams, ListenResult, SpeakParams, VoiceConfig};

/// MCP Tool definition structure (matching sweetmcp-axum types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: ToolInputSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    #[serde(rename = "type")]
    pub type_name: String,
    pub properties: HashMap<String, ToolInputSchemaProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchemaProperty {
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    pub description: Option<String>,
}

/// Voice service trait that implementations must provide
///
/// Note: Methods return `impl Future` instead of using async-trait.
/// Implementors should use `async fn` or return boxed futures.
pub trait VoiceService: Send + Sync {
    /// Synthesize speech from text
    fn speak(&self, params: SpeakParams) -> impl std::future::Future<Output = VoiceResult<()>> + Send;

    /// Listen for speech and transcribe to text
    fn listen(&self, params: ListenParams) -> impl std::future::Future<Output = VoiceResult<ListenResult>> + Send;

    /// Get available voice IDs
    fn list_voices(&self) -> impl std::future::Future<Output = VoiceResult<Vec<String>>> + Send;

    /// Get available microphone devices
    fn list_microphones(&self) -> impl std::future::Future<Output = VoiceResult<Vec<String>>> + Send;
}

/// Tool registry helper
pub fn register_voice_tools() -> Vec<Tool> {
    info!("Registering voice tools");
    vec![speak_tool(), listen_tool()]
}

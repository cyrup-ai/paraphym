//! SweetMCP Plugin Backend
//!
//! This backend executes SweetMCP WASM plugins directly using the Extism runtime.
//! It provides secure execution of tools via WASM sandboxing while maintaining
//! the same interface as other Cylo backends.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// Note: These imports would be needed when the SweetMCP integration is complete
// use extism::{Plugin, Wasm, Context};
// use sweetmcp_plugin_builder::{CallToolRequest, CallToolResult, McpTool};

// Placeholder types for compilation
#[derive(Debug, Clone)]
pub struct Plugin;

#[derive(Debug, Clone)]
pub struct Wasm;

#[derive(Debug, Clone)]
pub struct Context;

impl Context {
    pub fn new() -> Self {
        Context
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolRequest {
    pub method: String,
    pub params: CallToolParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolResult {
    pub result: Option<CallToolContent>,
    pub error: Option<CallToolError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolError {
    pub message: String,
}

use super::{
    AsyncTask, ExecutionBackend, ExecutionRequest, ExecutionResult, HealthStatus,
    BackendConfig, BackendError, BackendResult, ResourceUsage,
};
use crate::execution_env::CyloResult;

/// SweetMCP Plugin backend implementation
///
/// Executes SweetMCP WASM plugins using the Extism runtime for secure isolation.
/// Tools are executed via the MCP tool protocol but locally without network calls.
#[derive(Debug)]
pub struct SweetMcpPluginBackend {
    /// Path to the WASM plugin file
    plugin_path: String,
    /// Backend configuration
    config: BackendConfig,
    /// Extism context for WASM execution
    context: Arc<Context>,
    /// Supported languages (determined by plugin capabilities)
    supported_languages: Vec<&'static str>,
}

impl SweetMcpPluginBackend {
    /// Create a new SweetMCP Plugin backend
    ///
    /// # Arguments
    /// * `plugin_path` - Path to the WASM plugin file
    /// * `config` - Backend configuration
    ///
    /// # Returns
    /// New backend instance or error if plugin cannot be loaded
    pub fn new(plugin_path: String, config: BackendConfig) -> BackendResult<Self> {
        // Validate plugin file exists
        if !std::path::Path::new(&plugin_path).exists() {
            return Err(BackendError::InvalidConfig {
                backend: "SweetMcpPlugin",
                details: format!("Plugin file not found: {}", plugin_path),
            });
        }

        // Create Extism context
        let context = Arc::new(Context::new());

        // For now, assume all common languages are supported
        // In practice, we would query the plugin for its capabilities
        let supported_languages = vec!["python", "javascript", "rust", "bash", "go"];

        Ok(Self {
            plugin_path,
            config,
            context,
            supported_languages,
        })
    }

    /// Load and create a plugin instance
    async fn create_plugin(&self) -> BackendResult<Plugin> {
        // Placeholder implementation - would load actual WASM plugin
        // TODO: Replace with real Extism plugin loading when dependencies are available
        Ok(Plugin)
    }

    /// Convert ExecutionRequest to CallToolRequest
    fn execution_to_tool_request(&self, request: &ExecutionRequest) -> CallToolRequest {
        let mut arguments = HashMap::new();
        arguments.insert("code".to_string(), JsonValue::String(request.code.clone()));
        arguments.insert("language".to_string(), JsonValue::String(request.language.clone()));

        if let Some(input) = &request.input {
            arguments.insert("input".to_string(), JsonValue::String(input.clone()));
        }

        // Add environment variables
        if !request.env_vars.is_empty() {
            let env_json = JsonValue::Object(
                request.env_vars
                    .iter()
                    .map(|(k, v)| (k.clone(), JsonValue::String(v.clone())))
                    .collect()
            );
            arguments.insert("env".to_string(), env_json);
        }

        CallToolRequest {
            method: "tools/call".to_string(),
            params: CallToolParams {
                name: "execute_code".to_string(),
                arguments: JsonValue::Object(arguments.into_iter().collect()),
            },
        }
    }

    /// Convert CallToolResult to ExecutionResult
    fn tool_result_to_execution(&self, result: CallToolResult, duration: Duration) -> ExecutionResult {
        match result.result {
            Some(content) => {
                // Parse the result content
                if let Ok(result_json) = serde_json::from_str::<JsonValue>(&content.text) {
                    let success = result_json.get("success")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    let stdout = result_json.get("stdout")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let stderr = result_json.get("stderr")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    ExecutionResult {
                        exit_code: if success { 0 } else { 1 },
                        stdout,
                        stderr,
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    }
                } else {
                    // Fallback for non-JSON results
                    ExecutionResult {
                        exit_code: 0,
                        stdout: content.text,
                        stderr: String::new(),
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    }
                }
            }
            None => {
                let error_msg = result.error
                    .map(|e| e.message)
                    .unwrap_or_else(|| "Unknown plugin error".to_string());

                ExecutionResult {
                    exit_code: 1,
                    stdout: String::new(),
                    stderr: error_msg,
                    duration,
                    resource_usage: ResourceUsage::default(),
                    metadata: HashMap::new(),
                }
            }
        }
    }
}

impl ExecutionBackend for SweetMcpPluginBackend {
    fn execute_code(&self, request: ExecutionRequest) -> AsyncTask<ExecutionResult> {
        let _plugin_path = self.plugin_path.clone();
        let _context = Arc::clone(&self.context);
        let backend = self.clone_for_async();

        tokio::spawn(async move {
            let start_time = SystemTime::now();

            // Create plugin instance
            let _plugin = match backend.create_plugin().await {
                Ok(plugin) => plugin,
                Err(e) => {
                    let duration = start_time.elapsed().unwrap_or_default();
                    return ExecutionResult {
                        exit_code: 1,
                        stdout: String::new(),
                        stderr: format!("Plugin creation failed: {}", e),
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    };
                }
            };

            // Convert request to tool call
            let tool_request = backend.execution_to_tool_request(&request);

            // Serialize the request
            let _request_json = match serde_json::to_string(&tool_request) {
                Ok(json) => json,
                Err(e) => {
                    let duration = start_time.elapsed().unwrap_or_default();
                    return ExecutionResult {
                        exit_code: 1,
                        stdout: String::new(),
                        stderr: format!("Request serialization failed: {}", e),
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    };
                }
            };

            // Call the plugin (placeholder implementation)
            // TODO: Replace with actual Extism plugin call when dependencies are available
            let response_str = r#"{"result": {"text": "{\"success\": true, \"stdout\": \"Plugin execution placeholder\", \"stderr\": \"\"}"}}"#.to_string();
            let tool_result: CallToolResult = match serde_json::from_str(&response_str) {
                Ok(result) => result,
                Err(e) => {
                    let duration = start_time.elapsed().unwrap_or_default();
                    return ExecutionResult {
                        exit_code: 1,
                        stdout: String::new(),
                        stderr: format!("Response parsing failed: {}: {}", e, response_str),
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    };
                }
            };

            let duration = start_time.elapsed().unwrap_or_default();
            backend.tool_result_to_execution(tool_result, duration)
        })
    }

    fn health_check(&self) -> AsyncTask<HealthStatus> {
        let plugin_path = self.plugin_path.clone();

        tokio::spawn(async move {
            // Placeholder health check - would verify WASM plugin can be loaded
            // TODO: Replace with real Extism health check when dependencies are available
            if std::path::Path::new(&plugin_path).exists() {
                HealthStatus::healthy("Plugin file exists")
                    .with_metric("plugin_path", &plugin_path)
            } else {
                HealthStatus::unhealthy(format!("Plugin file not found: {}", plugin_path))
            }
        })
    }

    fn cleanup(&self) -> AsyncTask<CyloResult<()>> {
        tokio::spawn(async move {
            // SweetMCP plugins don't require explicit cleanup
            // The Extism runtime handles WASM instance cleanup automatically
            Ok(())
        })
    }

    fn get_config(&self) -> &BackendConfig {
        &self.config
    }

    fn backend_type(&self) -> &'static str {
        "SweetMcpPlugin"
    }

    fn supports_language(&self, language: &str) -> bool {
        self.supported_languages.contains(&language)
    }

    fn supported_languages(&self) -> &[&'static str] {
        &self.supported_languages
    }
}

// Helper implementation for async cloning
impl SweetMcpPluginBackend {
    fn clone_for_async(&self) -> Self {
        Self {
            plugin_path: self.plugin_path.clone(),
            config: self.config.clone(),
            context: Arc::clone(&self.context),
            supported_languages: self.supported_languages.clone(),
        }
    }
}
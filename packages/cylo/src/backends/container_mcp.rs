//! Container MCP Backend
//!
//! This backend executes MCP tools running in containers via direct invocation
//! rather than over the MCP protocol. It provides secure execution while maintaining
//! the same interface as other Cylo backends.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// Note: These imports would be needed when the SweetMCP integration is complete
// use sweetmcp_json_client::JsonClient;
// use sweet_mcp_type::ToolInfo;
// use mcp_client_traits::McpClient;

// Placeholder types for compilation
#[derive(Debug, Clone)]
pub struct JsonClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub result: Option<JsonValue>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Clone)]
pub struct ResponseError {
    pub message: String,
}

impl JsonClient {
    pub fn new(_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(JsonClient)
    }

    pub async fn list_tools(&self) -> Result<Vec<ToolInfo>, Box<dyn std::error::Error>> {
        // Placeholder implementation
        Ok(vec![
            ToolInfo {
                name: "execute_code".to_string(),
                description: Some("Execute code in container".to_string()),
            }
        ])
    }

    pub async fn call_tool(&self, _name: &str, _args: JsonValue) -> Result<Response, Box<dyn std::error::Error>> {
        // Placeholder implementation
        let result = serde_json::json!({
            "success": true,
            "stdout": "Container execution placeholder",
            "stderr": ""
        });
        Ok(Response {
            result: Some(result),
            error: None,
        })
    }
}

use super::{
    AsyncTask, ExecutionBackend, ExecutionRequest, ExecutionResult, HealthStatus,
    BackendConfig, BackendError, BackendResult, ResourceUsage,
};
use crate::execution_env::CyloResult;

/// Container MCP backend implementation
///
/// Executes MCP tools running in containers via direct invocation.
/// This provides secure execution while bypassing network protocol overhead.
#[derive(Debug)]
pub struct ContainerMcpBackend {
    /// Server URL for the containerized MCP tools
    server_url: String,
    /// Backend configuration
    config: BackendConfig,
    /// MCP client for tool discovery and execution
    client: Arc<JsonClient>,
    /// Cached tool information
    available_tools: Arc<tokio::sync::RwLock<Vec<ToolInfo>>>,
    /// Supported languages (determined by available tools)
    supported_languages: Vec<&'static str>,
}

impl ContainerMcpBackend {
    /// Create a new Container MCP backend
    ///
    /// # Arguments
    /// * `server_url` - URL of the containerized MCP server
    /// * `config` - Backend configuration
    ///
    /// # Returns
    /// New backend instance or error if server cannot be reached
    pub fn new(server_url: String, config: BackendConfig) -> BackendResult<Self> {
        // Create MCP client
        let client = JsonClient::new(&server_url)
            .map_err(|e| BackendError::NetworkFailed {
                details: format!("Failed to create MCP client: {}", e),
            })?;

        // For now, assume all common languages are supported
        // In practice, we would query the available tools to determine capabilities
        let supported_languages = vec!["python", "javascript", "rust", "bash", "go"];

        Ok(Self {
            server_url,
            config,
            client: Arc::new(client),
            available_tools: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            supported_languages,
        })
    }

    /// Initialize and discover available tools
    pub async fn initialize(&self) -> BackendResult<()> {
        let tools = self.client.list_tools().await
            .map_err(|e| BackendError::NetworkFailed {
                details: format!("Failed to list tools: {}", e),
            })?;

        let mut available_tools = self.available_tools.write().await;
        *available_tools = tools;

        Ok(())
    }

    /// Convert ExecutionRequest to MCP tool arguments
    fn execution_to_mcp_args(&self, request: &ExecutionRequest) -> JsonValue {
        let mut arguments = std::collections::HashMap::new();
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

        // Add working directory
        if let Some(working_dir) = &request.working_dir {
            arguments.insert("working_dir".to_string(), JsonValue::String(working_dir.clone()));
        }

        // Add timeout
        arguments.insert("timeout".to_string(), JsonValue::Number(
            serde_json::Number::from(request.timeout.as_secs())
        ));

        JsonValue::Object(arguments.into_iter().collect())
    }

    /// Convert MCP response to ExecutionResult
    fn mcp_response_to_execution(&self, response: Response, duration: Duration) -> ExecutionResult {
        match response.result {
            Some(result_data) => {
                // Since we're using serde_json::Value directly in placeholder, no conversion needed
                let result_json = result_data;

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
            }
            None => {
                let error_msg = response.error
                    .map(|e| e.message)
                    .unwrap_or_else(|| "Unknown MCP error".to_string());

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

    // Conversion method not needed with placeholder types using serde_json::Value directly

    /// Find the best tool for code execution
    async fn find_execution_tool(&self) -> Option<String> {
        let tools = self.available_tools.read().await;

        // Look for standard execution tools
        for tool in tools.iter() {
            if tool.name == "execute_code" ||
               tool.name == "code_execution" ||
               tool.name.starts_with("execute_") {
                return Some(tool.name.clone());
            }
        }

        // Fallback: look for any tool that mentions code execution in description
        for tool in tools.iter() {
            if let Some(description) = &tool.description {
                if description.to_lowercase().contains("execute code") ||
                   description.to_lowercase().contains("run code") {
                    return Some(tool.name.clone());
                }
            }
        }

        None
    }
}

impl ExecutionBackend for ContainerMcpBackend {
    fn execute_code(&self, request: ExecutionRequest) -> AsyncTask<ExecutionResult> {
        let client: Arc<JsonClient> = Arc::clone(&self.client);
        let backend = self.clone_for_async();

        tokio::spawn(async move {
            let start_time = SystemTime::now();

            // Find an appropriate tool for execution
            let tool_name = match backend.find_execution_tool().await {
                Some(name) => name,
                None => {
                    let duration = start_time.elapsed().unwrap_or_default();
                    return ExecutionResult {
                        exit_code: 1,
                        stdout: String::new(),
                        stderr: "No suitable execution tool found in container".to_string(),
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    };
                }
            };

            // Convert request to MCP arguments
            let args = backend.execution_to_mcp_args(&request);

            // Execute the tool via MCP
            let response = match client.call_tool(&tool_name, args).await {
                Ok(response) => response,
                Err(e) => {
                    let duration = start_time.elapsed().unwrap_or_default();
                    return ExecutionResult {
                        exit_code: 1,
                        stdout: String::new(),
                        stderr: format!("MCP tool execution failed: {}", e),
                        duration,
                        resource_usage: ResourceUsage::default(),
                        metadata: HashMap::new(),
                    };
                }
            };

            let duration = start_time.elapsed().unwrap_or_default();
            backend.mcp_response_to_execution(response, duration)
        })
    }

    fn health_check(&self) -> AsyncTask<HealthStatus> {
        let client: Arc<JsonClient> = Arc::clone(&self.client);
        let server_url = self.server_url.clone();

        tokio::spawn(async move {
            // Try to list tools to verify server is responsive
            match client.list_tools().await {
                Ok(tools) => {
                    let tool_count = tools.len();
                    HealthStatus::healthy(format!("Container MCP server responsive with {} tools", tool_count))
                        .with_metric("server_url", &server_url)
                        .with_metric("tool_count", &tool_count.to_string())
                }
                Err(e) => {
                    HealthStatus::unhealthy(format!("Failed to connect to container MCP server: {}", e))
                        .with_metric("server_url", &server_url)
                }
            }
        })
    }

    fn cleanup(&self) -> AsyncTask<CyloResult<()>> {
        tokio::spawn(async move {
            // Container MCP backends don't require explicit cleanup
            // The container runtime handles resource cleanup
            Ok(())
        })
    }

    fn get_config(&self) -> &BackendConfig {
        &self.config
    }

    fn backend_type(&self) -> &'static str {
        "ContainerMcp"
    }

    fn supports_language(&self, language: &str) -> bool {
        self.supported_languages.contains(&language)
    }

    fn supported_languages(&self) -> &[&'static str] {
        &self.supported_languages
    }
}

// Helper implementation for async cloning
impl ContainerMcpBackend {
    fn clone_for_async(&self) -> Self {
        Self {
            server_url: self.server_url.clone(),
            config: self.config.clone(),
            client: Arc::clone(&self.client),
            available_tools: Arc::clone(&self.available_tools),
            supported_languages: self.supported_languages.clone(),
        }
    }
}
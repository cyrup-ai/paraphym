//! Unified Tool Execution Interface
//!
//! This module provides a single, unified interface for tool execution that
//! transparently handles both MCP server tools and native code execution.
//! Users never directly call tools - they prompt naturally and the LLM
//! decides which tools to call, similar to OpenAI function calling.

use std::collections::HashMap;
use std::sync::Arc;

use sweet_mcp_type::{ToolInfo, JsonValue, Response};
use sweetmcp_json_client::JsonClient;
use sweetmcp_stdio_client::StdioClient;
use mcp_client_traits::{ClientError, McpClient};
use ystream::AsyncStream;
use serde_json::Value;
use serde_json;
use value_trait::derived::ValueObjectAccess;
use cylo::ExecutionResult;
use tokio::task::JoinError;
use crate::builders::agent_role::McpServerConfig;
use crate::domain::context::chunk::CandleJsonChunk;
use crate::domain::tool::router::SweetMcpRouter;

/// Unified tool execution interface that handles both MCP and native tools
pub struct UnifiedToolExecutor {
    /// MCP clients for multiple servers (server_id -> client)
    mcp_clients: Arc<tokio::sync::RwLock<HashMap<String, Arc<dyn McpClient + Send + Sync>>>>,
    /// Available tools discovered from MCP servers and native capabilities
    available_tools: Arc<tokio::sync::RwLock<Vec<ToolInfo>>>,
    /// Mapping of tool name to server ID for O(1) tool routing
    tool_server_map: Arc<tokio::sync::RwLock<HashMap<String, String>>>,
    /// Server configurations for connection management
    mcp_server_configs: Vec<McpServerConfig>,
    /// Native WASM tool router for direct execution (not MCP protocol)
    native_router: Option<Arc<SweetMcpRouter>>,
}

/// Tool execution error types
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("MCP client error: {0}")]
    McpError(#[from] ClientError),
    #[error("Code execution error: {0}")]
    CyloError(String),
    #[error("Task execution error: {0}")]
    JoinError(#[from] JoinError),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("General error: {0}")]
    Other(#[from] anyhow::Error),
}

impl UnifiedToolExecutor {
    /// Create a new unified tool executor
    pub fn new(mcp_client: Option<Arc<dyn McpClient + Send + Sync>>) -> Self {
        Self {
            mcp_clients: Arc::new(tokio::sync::RwLock::new(if let Some(client) = mcp_client {
                let mut clients = HashMap::new();
                clients.insert("default".to_string(), client);
                clients
            } else {
                HashMap::new()
            })),
            available_tools: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            tool_server_map: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            mcp_server_configs: Vec::new(),
            native_router: None,
        }
    }

    /// Create a new unified tool executor with MCP server URL
    pub fn with_mcp_server(server_url: Option<String>) -> Result<Self, ToolError> {
        let mcp_client = if let Some(url) = server_url {
            let client = JsonClient::new(&url)?;
            let client_arc: Arc<dyn McpClient + Send + Sync> = Arc::new(client);
            Some(client_arc)
        } else {
            None
        };

        Ok(Self::new(mcp_client))
    }

    /// Create unified tool executor with multiple MCP servers
    pub fn with_mcp_servers(server_configs: Vec<McpServerConfig>) -> Self {
        Self {
            mcp_clients: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            available_tools: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            tool_server_map: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            mcp_server_configs: server_configs,
            native_router: None,
        }
    }

    /// Initialize the executor by discovering available tools
    pub async fn initialize(&self) -> Result<(), ToolError> {
        let mut tools = Vec::new();

        // Connect to and discover tools from all MCP servers
        for (i, server_config) in self.mcp_server_configs.iter().enumerate() {
            let server_id = format!("server_{}", i);
            
            // Parse init_command properly handling quoted arguments
            let command_parts = match shell_words::split(&server_config.init_command) {
                Ok(parts) => parts,
                Err(e) => {
                    tracing::warn!("Failed to parse init_command for server {}: {}", server_id, e);
                    continue;
                }
            };
            
            if command_parts.is_empty() {
                tracing::warn!("Empty init_command for server {}", server_id);
                continue;
            }

            let binary = server_config.binary_path.as_deref().unwrap_or(&command_parts[0]);
            let args: Vec<String> = if command_parts.len() > 1 {
                command_parts[1..].to_vec()
            } else {
                Vec::new()
            };
            
            // Spawn stdio MCP server subprocess
            match StdioClient::new(binary, &args, &[]).await {
                Ok(client) => {
                    let client_arc: Arc<dyn McpClient + Send + Sync> = Arc::new(client);
                    
                    // Discover tools from this server
                    match client_arc.list_tools().await {
                        Ok(server_tools) => {
                            // Populate tool_server_map for O(1) routing
                            {
                                let mut tool_map = self.tool_server_map.write().await;
                                for tool in &server_tools {
                                    if let Some(existing_server) = tool_map.get(&tool.name) {
                                        tracing::warn!(
                                            "Tool '{}' from server '{}' conflicts with server '{}', using latest",
                                            tool.name, server_id, existing_server
                                        );
                                    }
                                    tool_map.insert(tool.name.clone(), server_id.clone());
                                }
                            }
                            
                            tools.extend(server_tools);
                            let mut clients = self.mcp_clients.write().await;
                            clients.insert(server_id.clone(), client_arc);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to discover tools from server {}: {}", server_id, e);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to spawn MCP server {}: {}", server_id, e);
                }
            }
        }

        // Add native code execution tools (Cylo is always enabled)
        tools.extend(self.create_native_code_tools());

        // Store discovered tools
        let mut available_tools = self.available_tools.write().await;
        *available_tools = tools;

        Ok(())
    }

    /// Execute a tool by name with arguments - OpenAI-style function calling
    /// Users never call this directly - the LLM calls tools automatically
    pub async fn call_tool(&self, tool_name: &str, args: JsonValue) -> Result<Response, ToolError> {
        // Find the tool
        let tools = self.available_tools.read().await;
        let tool_info = tools
            .iter()
            .find(|t| t.name == tool_name)
            .ok_or_else(|| ToolError::ToolNotFound(tool_name.to_string()))?;

        // Route based on tool type - implementation detail hidden from user
        if self.is_mcp_tool(tool_info) {
            self.execute_mcp_tool(tool_name, args).await
        } else {
            self.execute_native_tool(tool_info, args)
        }
    }

    /// Execute tool and return ystream for compatibility with existing architecture
    pub fn call_tool_stream(&self, tool_name: &str, args: JsonValue) -> AsyncStream<CandleJsonChunk> {
        let tool_name = tool_name.to_string();
        let executor = self.clone_for_async();

        // BLOCKING CODE APPROVED BY DAVID ON 2025-01-29: Using shared_runtime().block_on() for async operations within ystream closure
        AsyncStream::with_channel(move |sender| {
            match crate::runtime::shared_runtime().block_on(executor.call_tool(&tool_name, args)) {
                Ok(response) => {
                    // Convert Response to Value for ystream compatibility
                    let value = response_to_value(response);
                    ystream::emit!(sender, CandleJsonChunk(value));
                }
                Err(e) => {
                    let error_value = Value::Object([
                        ("error".to_string(), Value::String(e.to_string()))
                    ].into_iter().collect::<serde_json::Map<String, Value>>());
                    ystream::emit!(sender, CandleJsonChunk(error_value));
                }
            }
        })
    }

    /// Get all available tools for LLM function calling
    pub async fn get_available_tools(&self) -> Vec<ToolInfo> {
        let tools = self.available_tools.read().await;
        tools.clone()
    }

    /// Check if this is an MCP tool or native tool
    fn is_mcp_tool(&self, tool_info: &ToolInfo) -> bool {
        // MCP tools typically have more complex schemas and descriptions
        // Native code execution tools have simpler, code-focused schemas
        !self.is_code_execution_tool(tool_info)
    }

    /// Check if this is a code execution tool
    fn is_code_execution_tool(&self, tool_info: &ToolInfo) -> bool {
        tool_info.name.starts_with("execute_") ||
        tool_info.name == "code_execution" ||
        tool_info.description.as_ref().map_or(false, |d| d.contains("execute code"))
    }

    /// Execute MCP server tool
    async fn execute_mcp_tool(&self, tool_name: &str, args: JsonValue) -> Result<Response, ToolError> {
        // For now, try each MCP client until one succeeds
        // In a production system, we'd maintain a mapping of tool_name -> server_id
        let clients = self.mcp_clients.read().await;
        for (server_id, client) in clients.iter() {
            match client.call_tool(tool_name, args.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    tracing::debug!("Server {} doesn't have tool {}: {}", server_id, tool_name, e);
                }
            }
        }
        
        Err(ToolError::ToolNotFound(format!("Tool '{}' not found in any MCP server", tool_name)))
    }

    /// Execute native tool via cylo secure execution
    fn execute_native_tool(&self, _tool_info: &ToolInfo, args: JsonValue) -> Result<Response, ToolError> {
        // Extract code and language from arguments
        let _code = args.get("code")
            .and_then(|v| match v {
                JsonValue::String(s) => Some(s.as_str()),
                _ => None,
            })
            .ok_or_else(|| ToolError::InvalidArguments("Missing 'code' parameter".to_string()))?;

        let _language = args.get("language")
            .and_then(|v| match v {
                JsonValue::String(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("python"); // Default to Python

        // TODO: Replace with proper SweetMCP plugin execution
        // This is a placeholder until UnifiedToolExecutor is replaced by SweetMcpRouter
        Err(ToolError::CyloError("execute_code_auto removed - use SweetMcpRouter".to_string()))
    }

    /// Create native code execution tool definitions
    fn create_native_code_tools(&self) -> Vec<ToolInfo> {
        vec![
            ToolInfo {
                name: "execute_code".to_string(),
                description: Some(
                    "Execute code securely in various programming languages. \
                     Use this tool when you need to run Python, JavaScript, Rust, \
                     Bash, or Go code safely in a sandboxed environment.".to_string()
                ),
                input_schema: {
                    let schema = serde_json::json!({
                        "type": "object",
                        "properties": {
                            "code": {
                                "type": "string",
                                "description": "The code to execute"
                            },
                            "language": {
                                "type": "string",
                                "enum": ["python", "javascript", "rust", "bash", "go"],
                                "default": "python",
                                "description": "Programming language"
                            }
                        },
                        "required": ["code"]
                    });
                    serde_json::from_value(schema).unwrap_or(JsonValue::Static(simd_json::StaticNode::Null))
                },
            },
            ToolInfo {
                name: "execute_python".to_string(),
                description: Some(
                    "Execute Python code securely. Perfect for data analysis, \
                     calculations, and scientific computing.".to_string()
                ),
                input_schema: create_code_only_schema("Python code to execute"),
            },
            ToolInfo {
                name: "execute_javascript".to_string(),
                description: Some(
                    "Execute JavaScript code securely. Perfect for web-related \
                     logic, JSON processing, and Node.js operations.".to_string()
                ),
                input_schema: create_code_only_schema("JavaScript code to execute"),
            },
        ]
    }

    /// Create a clone for async operations - Arc<JsonClient> allows cheap cloning
    fn clone_for_async(&self) -> Self {
        Self {
            mcp_clients: self.mcp_clients.clone(), // HashMap of Arc allows cheap cloning
            available_tools: self.available_tools.clone(),
            mcp_server_configs: self.mcp_server_configs.clone(),
        }
    }
}

/// Helper function to create a code-only input schema
fn create_code_only_schema(description: &str) -> JsonValue {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "code": {
                "type": "string",
                "description": description
            }
        },
        "required": ["code"]
    });
    serde_json::from_value(schema).unwrap_or(JsonValue::Static(simd_json::StaticNode::Null))
}

/// Convert cylo ExecutionResult to MCP Response
#[allow(dead_code)]
fn create_tool_response(result: ExecutionResult) -> Response {
    
    // Create response data as JsonValue using serde conversion
    let response_data = if result.exit_code == 0 {
        let obj = serde_json::json!({
            "success": true,
            "stdout": result.stdout,
            "stderr": result.stderr,
        });
        serde_json::from_value(obj).unwrap_or(JsonValue::Static(simd_json::StaticNode::Null))
    } else {
        let obj = serde_json::json!({
            "success": false,
            "stdout": result.stdout, 
            "stderr": result.stderr,
            "error": format!("Process exited with code {}", result.exit_code),
        });
        serde_json::from_value(obj).unwrap_or(JsonValue::Static(simd_json::StaticNode::Null))
    };

    // Return Response with result data
    Response {
        id: sweet_mcp_type::RequestId::Str("tool_execution".to_string()),
        result: Some(response_data),
        error: None,
    }
}

/// Convert sweet-mcp-type Response to serde_json::Value for ystream compatibility
fn response_to_value(response: Response) -> Value {
    // Convert the response to a generic Value format
    if let Some(result_data) = response.result {
        // Convert JsonValue to serde_json::Value
        convert_sweet_json_to_serde(result_data)
    } else if let Some(error) = response.error {
        Value::Object([
            ("error".to_string(), Value::String(error.message)),
            ("code".to_string(), Value::Number(error.code.into())),
        ].into_iter().collect::<serde_json::Map<String, Value>>())
    } else {
        Value::Object([
            ("type".to_string(), Value::String("response".to_string())),
            ("data".to_string(), Value::String("Empty response".to_string())),
        ].into_iter().collect::<serde_json::Map<String, Value>>())
    }
}

/// Convert sweet_mcp_type::JsonValue to serde_json::Value
fn convert_sweet_json_to_serde(value: JsonValue) -> Value {
    match value {
        JsonValue::Static(node) => match node {
            simd_json::StaticNode::Null => Value::Null,
            simd_json::StaticNode::Bool(b) => Value::Bool(b),
            simd_json::StaticNode::I64(n) => Value::Number(n.into()),
            simd_json::StaticNode::U64(n) => Value::Number(n.into()),
            simd_json::StaticNode::F64(f) => {
                Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| 0.into()))
            }
        },
        JsonValue::String(s) => Value::String(s),
        JsonValue::Array(arr) => {
            Value::Array(arr.into_iter().map(convert_sweet_json_to_serde).collect())
        }
        JsonValue::Object(obj) => {
            Value::Object(obj.into_iter().map(|(k, v)| (k, convert_sweet_json_to_serde(v))).collect())
        }
    }
}
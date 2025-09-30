//! SweetMCP Tool Router
//!
//! This module provides the unified tool routing interface described in TOOL_CALLING.md.
//! It implements Stage 3 (Function Calling) of the chat loop architecture, providing
//! transparent routing between SweetMCP plugins, container MCP tools, and Cylo execution.

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use ystream::AsyncStream;

use sweet_mcp_type::{ToolInfo, JsonValue};
use cylo::{Cylo, create_backend, BackendConfig, ExecutionRequest, ExecutionResult};
use crate::domain::context::chunk::CandleJsonChunk;

/// SweetMCP Tool Router
///
/// Provides transparent tool routing for the 5-stage chat loop architecture.
/// Tools appear identical via ToolInfo interface regardless of execution method.
#[derive(Debug)]
pub struct SweetMcpRouter {
    /// Available tools discovered from all sources
    available_tools: Arc<tokio::sync::RwLock<Vec<ToolInfo>>>,
    /// Tool routing map: tool_name -> execution strategy
    tool_routes: Arc<tokio::sync::RwLock<HashMap<String, ToolRoute>>>,
    /// WASM plugin configurations (user-provided)
    plugin_configs: Vec<PluginConfig>,
    /// Cylo backend configuration (optional)
    cylo_config: Option<CyloBackendConfig>,
}

/// Tool execution route strategy
#[derive(Debug, Clone)]
pub enum ToolRoute {
    /// Execute via SweetMCP WASM plugin
    SweetMcpPlugin {
        plugin_path: String,
    },
    /// Execute directly via Cylo backend
    CyloExecution {
        backend_type: String,
        config: String,
    },
}

/// Tool execution error types
#[derive(Debug, thiserror::Error)]
pub enum RouterError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Backend error: {0}")]
    BackendError(String),
}

impl SweetMcpRouter {
    /// Create a new SweetMCP router
    pub fn new() -> Self {
        Self {
            available_tools: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            tool_routes: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            backend_configs: HashMap::new(),
        }
    }

    /// Initialize router by discovering available tools
    ///
    /// Stage 1 (Discovery) - Scan all available tool sources:
    /// - SweetMCP WASM plugins
    /// - Container MCP servers
    /// - Native Cylo execution capabilities
    pub async fn initialize(&mut self) -> Result<(), RouterError> {
        let mut tools = Vec::new();
        let mut routes = HashMap::new();

        // Discover SweetMCP plugins from configuration
        self.discover_sweetmcp_plugins(&mut tools, &mut routes)?;

        // Add native code execution tools
        self.add_native_execution_tools(&mut tools, &mut routes)?;

        // Store discovered tools and routes
        {
            let mut available_tools = self.available_tools.write().await;
            *available_tools = tools;
        }
        {
            let mut tool_routes = self.tool_routes.write().await;
            *tool_routes = routes;
        }

        Ok(())
    }

    /// Execute a tool by name with arguments
    ///
    /// Stage 3 (Function Calling) - Transparent tool execution via routing.
    /// The LLM calls tools naturally without knowing execution method.
    pub async fn call_tool(&self, tool_name: &str, args: JsonValue) -> Result<Value, RouterError> {
        // Find the tool route
        let route = {
            let routes = self.tool_routes.read().await;
            routes.get(tool_name).cloned()
                .ok_or_else(|| RouterError::ToolNotFound(tool_name.to_string()))?
        };

        // Execute via appropriate route
        match route {
            ToolRoute::SweetMcpPlugin { plugin_path } => {
                self.execute_sweetmcp_plugin(&plugin_path, args).await
            }
            ToolRoute::CyloExecution { backend_type, config } => {
                self.execute_cylo_backend(&backend_type, &config, args).await
            }
        }
    }

    /// Get all available tools for LLM function calling
    ///
    /// Stage 2 (Selection) - Provide ToolInfo for LLM tool selection.
    /// All tools appear identical regardless of execution method.
    pub async fn get_available_tools(&self) -> Vec<ToolInfo> {
        let tools = self.available_tools.read().await;
        tools.clone()
    }

    /// Execute tool and return ystream for compatibility
    pub fn call_tool_stream(&self, tool_name: &str, args: JsonValue) -> AsyncStream<CandleJsonChunk> {
        let tool_name = tool_name.to_string();
        let router = self.clone_for_async();

        // BLOCKING CODE APPROVED BY DAVID ON 2025-01-29: Using shared_runtime().block_on() for async operations within ystream closure
        AsyncStream::with_channel(move |sender| {
            match crate::runtime::shared_runtime().block_on(router.call_tool(&tool_name, args)) {
                Ok(result) => {
                    ystream::emit!(sender, CandleJsonChunk(result));
                }
                Err(e) => {
                    let error_value = Value::Object(
                        [("error".to_string(), Value::String(e.to_string()))]
                        .into_iter()
                        .collect::<serde_json::Map<_, _>>()
                    );
                    ystream::emit!(sender, CandleJsonChunk(error_value));
                }
            }
        })
    }

    /// Discover SweetMCP WASM plugins from user configuration
    fn discover_sweetmcp_plugins(
        &self,
        tools: &mut Vec<ToolInfo>,
        routes: &mut HashMap<String, ToolRoute>,
    ) -> Result<(), RouterError> {
        // Create tools from user-provided plugin configurations
        for plugin_config in &self.plugin_configs {
            let tool_info = ToolInfo {
                name: plugin_config.tool_name.clone(),
                description: Some(plugin_config.description.clone()),
                input_schema: plugin_config.input_schema.clone(),
            };
            
            tools.push(tool_info);
            routes.insert(
                plugin_config.tool_name.clone(),
                ToolRoute::SweetMcpPlugin {
                    plugin_path: plugin_config.wasm_path.clone(),
                },
            );
        }

        Ok(())
    }

    /// Add native code execution tools via Cylo (if configured)
    fn add_native_execution_tools(
        &self,
        tools: &mut Vec<ToolInfo>,
        routes: &mut HashMap<String, ToolRoute>,
    ) -> Result<(), RouterError> {
        // Only add native execution tools if Cylo backend is configured
        let cylo_config = match &self.cylo_config {
            Some(config) => config,
            None => return Ok(()), // No Cylo configured, skip native execution tools
        };

        // Add native execution tools for different languages
        let languages = vec![
            ("execute_python", "Python"),
            ("execute_javascript", "JavaScript"),
            ("execute_rust", "Rust"),
            ("execute_bash", "Bash"),
            ("execute_go", "Go"),
        ];

        for (tool_name, language) in languages {
            let tool = ToolInfo {
                name: tool_name.to_string(),
                description: Some(format!("Execute {} code securely via Cylo", language)),
                input_schema: self.create_code_execution_schema(&format!("{} code to execute", language)),
            };

            tools.push(tool);

            // Use user-configured Cylo backend
            routes.insert(
                tool_name.to_string(),
                ToolRoute::CyloExecution {
                    backend_type: cylo_config.backend_type.clone(),
                    config: cylo_config.config_value.clone(),
                },
            );
        }

        Ok(())
    }

    /// Execute SweetMCP WASM plugin
    async fn execute_sweetmcp_plugin(&self, plugin_path: &str, args: JsonValue) -> Result<Value, RouterError> {
        // Create Cylo backend for SweetMCP plugin execution
        let cylo_env = Cylo::SweetMcpPlugin(plugin_path.to_string());
        let config = BackendConfig::new("sweetmcp_plugin");

        let backend = create_backend(&cylo_env, config)
            .map_err(|e| RouterError::BackendError(e.to_string()))?;

        // Convert JsonValue args to ExecutionRequest
        let request = self.json_args_to_execution_request(args)?;

        // Execute via backend
        let result_handle = backend.execute_code(request);
        let result = result_handle.await
            .map_err(|e| RouterError::ExecutionFailed(e.to_string()))?;

        // Convert ExecutionResult to JSON Value
        self.execution_result_to_json(result)
    }

    /// Execute via Cylo backend directly
    async fn execute_cylo_backend(&self, backend_type: &str, config: &str, args: JsonValue) -> Result<Value, RouterError> {
        // Create appropriate Cylo environment
        let cylo_env = match backend_type {
            "Apple" => Cylo::Apple(config.to_string()),
            "LandLock" => Cylo::LandLock(config.to_string()),
            "FireCracker" => Cylo::FireCracker(config.to_string()),
            "SweetMcpPlugin" => Cylo::SweetMcpPlugin(config.to_string()),
            _ => return Err(RouterError::BackendError(format!("Unknown backend type: {}", backend_type))),
        };

        let backend_config = BackendConfig::new(backend_type);
        let backend = create_backend(&cylo_env, backend_config)
            .map_err(|e| RouterError::BackendError(e.to_string()))?;

        // Convert JsonValue args to ExecutionRequest
        let request = self.json_args_to_execution_request(args)?;

        // Execute via backend
        let result_handle = backend.execute_code(request);
        let result = result_handle.await
            .map_err(|e| RouterError::ExecutionFailed(e.to_string()))?;

        // Convert ExecutionResult to JSON Value
        self.execution_result_to_json(result)
    }

    /// Convert JsonValue arguments to ExecutionRequest
    fn json_args_to_execution_request(&self, args: JsonValue) -> Result<ExecutionRequest, RouterError> {
        // Convert sweet_mcp_type::JsonValue to serde_json::Value first
        let args_value = self.convert_sweet_json_to_serde(args);

        let code = args_value.get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| RouterError::InvalidArguments("Missing 'code' parameter".to_string()))?;

        let language = args_value.get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("python");

        let mut request = ExecutionRequest::new(code, language);

        // Add optional parameters
        if let Some(input) = args_value.get("input").and_then(|v| v.as_str()) {
            request = request.with_input(input);
        }

        if let Some(env_obj) = args_value.get("env").and_then(|v| v.as_object()) {
            for (key, value) in env_obj {
                if let Some(val_str) = value.as_str() {
                    request = request.with_env(key.clone(), val_str);
                }
            }
        }

        Ok(request)
    }

    /// Convert ExecutionResult to JSON Value
    fn execution_result_to_json(&self, result: ExecutionResult) -> Result<Value, RouterError> {
        let result_json = serde_json::json!({
            "success": result.exit_code == 0,
            "exit_code": result.exit_code,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "duration_ms": result.duration.as_millis(),
            "resource_usage": {
                "peak_memory": result.resource_usage.peak_memory,
                "cpu_time_ms": result.resource_usage.cpu_time_ms,
                "process_count": result.resource_usage.process_count,
            }
        });

        Ok(result_json)
    }

    /// Convert sweet_mcp_type::JsonValue to serde_json::Value
    fn convert_sweet_json_to_serde(&self, value: JsonValue) -> serde_json::Value {
        use simd_json::StaticNode;
        match value {
            JsonValue::Static(StaticNode::Null) => serde_json::Value::Null,
            JsonValue::Static(StaticNode::Bool(b)) => serde_json::Value::Bool(b),
            JsonValue::Static(StaticNode::I64(n)) => serde_json::Value::Number(n.into()),
            JsonValue::Static(StaticNode::U64(n)) => serde_json::Value::Number(n.into()),
            JsonValue::Static(StaticNode::F64(f)) => {
                serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| 0.into()))
            }
            JsonValue::String(s) => serde_json::Value::String(s),
            JsonValue::Array(arr) => {
                serde_json::Value::Array(arr.into_iter().map(|v| self.convert_sweet_json_to_serde(v)).collect())
            }
            JsonValue::Object(obj) => {
                serde_json::Value::Object(obj.into_iter().map(|(k, v)| (k, self.convert_sweet_json_to_serde(v))).collect())
            }
        }
    }

    /// Create a code execution input schema
    fn create_code_execution_schema(&self, description: &str) -> JsonValue {
        use crate::domain::agent::role::convert_serde_to_sweet_json;
        
        // Build schema using serde_json macro for clean, type-safe construction
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": description
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
        
        // Convert serde_json::Value to sweet_mcp_type::JsonValue (simd_json::Value)
        convert_serde_to_sweet_json(schema)
    }

    /// Create a clone for async operations
    fn clone_for_async(&self) -> Self {
        Self {
            available_tools: Arc::clone(&self.available_tools),
            tool_routes: Arc::clone(&self.tool_routes),
            plugin_configs: self.plugin_configs.clone(),
            cylo_config: self.cylo_config.clone(),
        }
    }
}

impl Default for SweetMcpRouter {
    fn default() -> Self {
        Self::new()
    }
}
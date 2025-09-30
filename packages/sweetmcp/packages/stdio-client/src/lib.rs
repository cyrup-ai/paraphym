//! stdio-client: MCP JSON-RPC client over subprocess stdin/stdout
//!
//! Implements newline-delimited JSON-RPC protocol for MCP stdio transport.

use serde_json::Value;
use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use std::sync::Arc;

use mcp_client_traits::{ClientError, McpClient};
use sweet_mcp_type::{JsonValue, Response, ToolInfo, RequestId, Implementation};

#[derive(Debug, Error)]
pub enum StdioClientError {
    #[error("Failed to spawn subprocess: {0}")]
    SpawnError(#[from] std::io::Error),
    
    #[error("Failed to serialize JSON-RPC request: {0}")]
    SerializeError(#[from] serde_json::Error),
    
    #[error("Subprocess terminated unexpectedly")]
    ProcessTerminated,
    
    #[error("Failed to send request: {0}")]
    SendError(String),
    
    #[error("Failed to receive response: {0}")]
    ReceiveError(String),
}

/// MCP client that communicates via subprocess stdin/stdout
#[derive(Debug)]
pub struct StdioClient {
    stdin: Arc<Mutex<ChildStdin>>,
    stdout: Arc<Mutex<BufReader<ChildStdout>>>,    _child: Arc<Mutex<Child>>,
}

impl StdioClient {
    /// Create a new stdio client by spawning a subprocess
    ///
    /// # Arguments
    /// * `command` - Command to execute
    /// * `args` - Command arguments
    /// * `env` - Environment variables as array of tuples
    pub async fn new(
        command: &str,
        args: &[String],
        env: &[(&str, &str)],
    ) -> Result<Self, StdioClientError> {
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);  // Ensure subprocess is killed when client is dropped
        
        for (key, value) in env {
            cmd.env(key, value);
        }
        
        let mut child = cmd.spawn()?;
        
        let stdin = child.stdin.take()
            .ok_or_else(|| StdioClientError::SendError("Failed to capture stdin".to_string()))?;        
        let stdout = child.stdout.take()
            .ok_or_else(|| StdioClientError::ReceiveError("Failed to capture stdout".to_string()))?;
        
        Ok(Self {
            stdin: Arc::new(Mutex::new(stdin)),
            stdout: Arc::new(Mutex::new(BufReader::new(stdout))),
            _child: Arc::new(Mutex::new(child)),
        })
    }
    
    /// Send a JSON-RPC request and receive response
    pub async fn send_request(&self, method: &str, params: Value) -> Result<Value, StdioClientError> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });
        
        let request_str = serde_json::to_string(&request)?;
        
        // Send request with newline delimiter
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(request_str.as_bytes()).await
            .map_err(|e| StdioClientError::SendError(e.to_string()))?;
        stdin.write_all(b"\n").await
            .map_err(|e| StdioClientError::SendError(e.to_string()))?;
        stdin.flush().await
            .map_err(|e| StdioClientError::SendError(e.to_string()))?;        
        drop(stdin);
        
        // Read response (newline-delimited)
        let mut stdout = self.stdout.lock().await;
        let mut response_line = String::new();
        stdout.read_line(&mut response_line).await
            .map_err(|e| StdioClientError::ReceiveError(e.to_string()))?;
        
        if response_line.is_empty() {
            return Err(StdioClientError::ProcessTerminated);
        }
        
        let response: Value = serde_json::from_str(&response_line)?;
        
        // Check for JSON-RPC error
        if let Some(error) = response.get("error") {
            return Err(StdioClientError::ReceiveError(
                format!("JSON-RPC error: {}", error)
            ));
        }
        
        // Return result field
        response.get("result")
            .cloned()
            .ok_or_else(|| StdioClientError::ReceiveError("Missing result field".to_string()))
    }
}

impl McpClient for StdioClient {
    fn list_tools(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<ToolInfo>, ClientError>> + Send + '_>> {
        Box::pin(async move {
            let result = self.send_request("tools/list", Value::Null).await
                .map_err(|e| ClientError::RequestBuild(e.to_string()))?;
            
            let tools_value = result.get("tools")
                .ok_or_else(|| ClientError::response_parse("Missing 'tools' field", "list_tools response"))?;
            
            let tools: Vec<ToolInfo> = serde_json::from_value(tools_value.clone())
                .map_err(|e| ClientError::response_parse(format!("Failed to parse tools: {}", e), "ToolInfo deserialization"))?;
            
            Ok(tools)
        })
    }
    
    fn call_tool(&self, name: &str, arguments: JsonValue) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, ClientError>> + Send + '_>> {
        let name = name.to_string();
        Box::pin(async move {
            let args_serde = convert_sweet_to_serde(arguments);
            
            let params = serde_json::json!({
                "name": &name,
                "arguments": args_serde
            });
            
            let result = self.send_request("tools/call", params).await
                .map_err(|e| ClientError::RequestBuild(e.to_string()))?;
            
            let response_data = convert_serde_to_sweet(result);
            
            Ok(Response {
                id: RequestId::Str(format!("stdio_{}", uuid::Uuid::new_v4())),
                result: Some(response_data),
                error: None,
            })
        })
    }
    
    fn initialize(&self, client_capabilities: JsonValue, client_info: Implementation) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, ClientError>> + Send + '_>> {
        Box::pin(async move {
            // Build parameters for initialize request
            let params = serde_json::json!({
                "protocolVersion": "2025-03-26",
                "capabilities": convert_sweet_to_serde(client_capabilities),
                "clientInfo": {
                    "name": client_info.name,
                    "version": client_info.version,
                }
            });
            
            // Send initialize request to MCP server
            let result = self.send_request("initialize", params).await
                .map_err(|e| ClientError::RequestBuild(e.to_string()))?;
            
            // Convert result to Response
            let response_data = convert_serde_to_sweet(result);
            
            Ok(Response {
                id: RequestId::Str(format!("initialize_{}", uuid::Uuid::new_v4())),
                result: Some(response_data),
                error: None,
            })
        })
    }
    
    fn ping(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, ClientError>> + Send + '_>> {
        Box::pin(async move {
            // Send ping request to MCP server
            let result = self.send_request("ping", serde_json::Value::Null).await
                .map_err(|e| ClientError::RequestBuild(e.to_string()))?;
            
            // Convert result to Response
            let response_data = convert_serde_to_sweet(result);
            
            Ok(Response {
                id: RequestId::Str(format!("ping_{}", uuid::Uuid::new_v4())),
                result: Some(response_data),
                error: None,
            })
        })
    }
}

fn convert_sweet_to_serde(value: JsonValue) -> Value {
    use simd_json::StaticNode;
    match value {
        JsonValue::Static(StaticNode::Null) => Value::Null,
        JsonValue::Static(StaticNode::Bool(b)) => Value::Bool(b),
        JsonValue::Static(StaticNode::I64(n)) => Value::Number(n.into()),
        JsonValue::Static(StaticNode::U64(n)) => Value::Number(n.into()),
        JsonValue::Static(StaticNode::F64(f)) => {
            Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| 0.into()))
        }
        JsonValue::String(s) => Value::String(s),
        JsonValue::Array(arr) => {
            Value::Array(arr.into_iter().map(convert_sweet_to_serde).collect())
        }
        JsonValue::Object(obj) => {
            Value::Object(obj.into_iter().map(|(k, v)| (k, convert_sweet_to_serde(v))).collect())
        }
    }
}

fn convert_serde_to_sweet(value: Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Static(simd_json::StaticNode::Null),
        Value::Bool(b) => JsonValue::Static(simd_json::StaticNode::Bool(b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                JsonValue::from(i)
            } else if let Some(f) = n.as_f64() {
                JsonValue::from(f)
            } else {
                JsonValue::Static(simd_json::StaticNode::Null)
            }
        }
        Value::String(s) => JsonValue::String(s),
        Value::Array(arr) => {
            let vec: Vec<JsonValue> = arr.into_iter().map(convert_serde_to_sweet).collect();
            JsonValue::Array(vec.into())
        }
        Value::Object(obj) => {
            let map: simd_json::value::owned::Object = obj.into_iter()
                .map(|(k, v)| (k, convert_serde_to_sweet(v)))
                .collect();
            JsonValue::Object(map.into())
        }
    }
}
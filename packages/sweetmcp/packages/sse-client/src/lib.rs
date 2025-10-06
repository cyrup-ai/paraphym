//! sse-client: MCP JSON-RPC client over Server-Sent Events
//!
//! Implements Streamable HTTP transport for MCP SSE connections.

use log::{debug, info, warn};
use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use thiserror::Error;

use mcp_client_traits::{ClientError, McpClient};
use sweet_mcp_type::{JsonValue, Response as McpResponse, ToolInfo, RequestId, Implementation};

#[derive(Debug, Error)]
pub enum SseClientError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Failed to serialize JSON-RPC request: {0}")]
    SerializeError(#[from] serde_json::Error),
    
    #[error("Failed to parse SSE event: {0}")]
    ParseError(String),
    
    #[error("JSON-RPC error: {0}")]
    JsonRpcError(String),
    
    #[error("Missing result field in response")]
    MissingResult,
}

/// MCP client that communicates via Server-Sent Events
#[derive(Debug, Clone)]
pub struct SseClient {
    base_url: String,
    http_client: Client,    headers: HashMap<String, String>,
}

impl SseClient {
    /// Create a new SSE client
    pub fn new(base_url: &str) -> Result<Self, SseClientError> {
        Ok(Self {
            base_url: base_url.to_string(),
            http_client: Client::new(),
            headers: HashMap::new(),
        })
    }
    
    /// Add custom header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Send JSON-RPC request via POST and receive response
    pub async fn send_request(&self, method: &str, params: Value) -> Result<Value, SseClientError> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });
        
        let mut request_builder = self.http_client
            .post(format!("{}/mcp", self.base_url))
            .header("Content-Type", "application/json");        
        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }
        
        let response = request_builder
            .json(&request)
            .send()
            .await?;
        
        let response_json: Value = response.json().await?;
        
        // Check for JSON-RPC error
        if let Some(error) = response_json.get("error") {
            let error_msg = error.to_string();
            warn!("SSE error: {}, attempting reconnect", error_msg);
            return Err(SseClientError::JsonRpcError(error_msg));
        }
        
        // Return result field
        response_json.get("result")
            .cloned()
            .ok_or(SseClientError::MissingResult)
    }
    
    /// Open SSE stream for bidirectional communication
    pub async fn open_stream(&self) -> Result<SseStream, SseClientError> {
        info!("SSE stream connected to {}", self.base_url);

        let mut request_builder = self.http_client
            .get(format!("{}/sse", self.base_url))
            .header("Accept", "text/event-stream");

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }

        let response = request_builder.send().await?;
        debug!("SSE event received: {:?}", response);
        Ok(SseStream {
            response,
            base_url: self.base_url.clone(),
        })
    }
}

/// SSE stream wrapper that logs lifecycle events
pub struct SseStream {
    response: Response,
    base_url: String,
}

impl Drop for SseStream {
    fn drop(&mut self) {
        info!("SSE stream disconnected from {}", self.base_url);
    }
}

impl Deref for SseStream {
    type Target = Response;
    
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl DerefMut for SseStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.response
    }
}

impl McpClient for SseClient {
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
    
    fn call_tool(&self, name: &str, arguments: JsonValue) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<McpResponse, ClientError>> + Send + '_>> {
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
            
            Ok(McpResponse {
                id: RequestId::Str(format!("sse_{}", uuid::Uuid::new_v4())),
                result: Some(response_data),
                error: None,
            })
        })
    }
    
    fn initialize(&self, client_capabilities: JsonValue, client_info: Implementation) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<McpResponse, ClientError>> + Send + '_>> {
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
            
            Ok(McpResponse {
                id: RequestId::Str(format!("initialize_{}", uuid::Uuid::new_v4())),
                result: Some(response_data),
                error: None,
            })
        })
    }
    
    fn ping(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<McpResponse, ClientError>> + Send + '_>> {
        Box::pin(async move {
            // Send ping request to MCP server
            let result = self.send_request("ping", serde_json::Value::Null).await
                .map_err(|e| ClientError::RequestBuild(e.to_string()))?;
            
            // Convert result to Response
            let response_data = convert_serde_to_sweet(result);
            
            Ok(McpResponse {
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
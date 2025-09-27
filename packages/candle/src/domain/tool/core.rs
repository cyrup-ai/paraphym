//! Core Tool Implementation - EXACT REPLICA of domain with Candle prefixes
//!
//! This module provides CandleTool and CandleMcpTool traits that exactly match
//! domain/src/tool/traits.rs with zero over-engineering.
//!
//! Supports ARCHITECTURE.md syntax: CandleTool<CandlePerplexity>::new([("citations", "true")])

use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use ystream::AsyncStream;
use serde_json::Value;

/// Wrapper for JSON values that implements MessageChunk
#[derive(Debug, Clone, Default)]
pub struct JsonValueResult {
    /// The JSON value
    pub value: Value,
}

impl JsonValueResult {
    /// Create a result with a JSON value
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl cyrup_sugars::prelude::MessageChunk for JsonValueResult {
    fn bad_chunk(_error: String) -> Self {
        Self::default()
    }

    fn error(&self) -> Option<&str> {
        None
    }
}

/// Candle marker type for Perplexity tool
#[derive(Debug, Clone)]
pub struct CandlePerplexity;

/// Marker type for Calculator tool
#[derive(Debug, Clone)]
pub struct CalculatorTool;

/// Core tool trait - EXACT REPLICA of domain Tool trait
pub trait CandleTool: Send + Sync + fmt::Debug {
    /// Get the name of the tool
    fn name(&self) -> &str;

    /// Get the description of the tool
    fn description(&self) -> &str;

    /// Get the JSON schema for the tool's input parameters
    fn parameters(&self) -> &Value;

    /// Execute the tool with given arguments - returns AsyncStream<JsonValueResult>
    fn execute(&self, args: Value) -> AsyncStream<JsonValueResult>;
}

/// MCP tool trait - EXACT REPLICA of domain McpTool trait
pub trait CandleMcpTool: CandleTool {
    /// Get the optional server identifier this tool belongs to
    fn server(&self) -> Option<&str>;

    /// Create a new MCP tool with the given name, description, and parameters
    fn new(name: impl Into<String>, description: impl Into<String>, parameters: Value) -> Self;
}

/// Generic Candle Tool implementation with type parameter for ARCHITECTURE.md syntax
#[derive(Debug, Clone)]
pub struct CandleToolImpl<T> {
    /// Type marker for compile-time tool differentiation (CandlePerplexity, etc.)
    _phantom: PhantomData<T>,
    /// Tool name
    name: String,
    /// Tool description
    description: String,
    /// Tool parameters as JSON schema
    parameters: Value,
    /// Tool configuration from constructor
    config: HashMap<String, Value>,
}

impl<T> CandleToolImpl<T> {
    /// Create new tool with config - EXACT ARCHITECTURE.md syntax: CandleTool<CandlePerplexity>::new([("citations", "true")])
    #[inline]
    pub fn new<P>(config: P) -> Self
    where
        P: Into<HashMap<&'static str, &'static str>>,
    {
        let config_map = config.into();
        let mut params_map = HashMap::with_capacity(config_map.len());

        for (k, v) in config_map {
            params_map.insert(k.to_string(), Value::String(v.to_string()));
        }

        let parameters = Value::Object(
            params_map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        );

        Self {
            _phantom: PhantomData,
            name: std::any::type_name::<T>()
                .split("::")
                .last()
                .unwrap_or("UnknownTool")
                .to_string(),
            description: format!("Tool for {}", std::any::type_name::<T>()),
            parameters,
            config: params_map,
        }
    }
}

impl<T> CandleTool for CandleToolImpl<T>
where
    T: Send + Sync + 'static + std::fmt::Debug + Clone,
{
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn description(&self) -> &str {
        &self.description
    }

    #[inline]
    fn parameters(&self) -> &Value {
        &self.parameters
    }

    fn execute(&self, args: Value) -> AsyncStream<JsonValueResult> {
        let config = self.config.clone();

        AsyncStream::with_channel(move |sender| {
            // Merge config with execution args
            let mut result_data = config;
            if let Value::Object(arg_map) = args {
                for (key, value) in arg_map {
                    result_data.insert(key, value);
                }
            }

            let result = Value::Object(result_data.into_iter().collect());
            let _ = sender.send(JsonValueResult::new(result));
        })
    }
}

/// Named tool implementation for ARCHITECTURE.md syntax: CandleTool::named("cargo").bin("~/.cargo/bin").description(...)
#[derive(Debug, Clone)]
pub struct CandleNamedTool {
    /// Tool name for identification
    name: String,
    /// Tool binary executable path
    bin_path: Option<String>,
    /// Tool functionality description
    description: String,
    /// Tool parameters schema
    parameters: Value,
}

impl CandleToolImpl<()> {
    /// Create named tool - EXACT ARCHITECTURE.md syntax: CandleTool::named("cargo")
    #[inline]
    pub fn named(name: impl Into<String>) -> CandleNamedTool {
        let name_str = name.into();
        CandleNamedTool {
            description: format!("Named tool: {}", name_str),
            name: name_str,
            bin_path: None,
            parameters: Value::Object(serde_json::Map::new()),
        }
    }
}

impl CandleNamedTool {
    /// Set binary path - EXACT ARCHITECTURE.md syntax: .bin("~/.cargo/bin")
    #[inline]
    pub fn bin(mut self, path: impl Into<String>) -> Self {
        self.bin_path = Some(path.into());
        self
    }

    /// Set description - EXACT ARCHITECTURE.md syntax: .description("cargo --help".exec_to_text())
    #[inline]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

impl CandleTool for CandleNamedTool {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn description(&self) -> &str {
        &self.description
    }

    #[inline]
    fn parameters(&self) -> &Value {
        &self.parameters
    }

    fn execute(&self, args: Value) -> AsyncStream<JsonValueResult> {
        let name = self.name.clone();
        let bin_path = self.bin_path.clone();

        AsyncStream::with_channel(move |sender| {
            // Execute named tool using configured path
            let command = bin_path.as_deref().unwrap_or(&name);

            // Build command args from JSON value
            let mut cmd_args = Vec::new();
            if let Value::Object(arg_map) = args {
                for (key, value) in arg_map {
                    cmd_args.push(format!("--{}", key));
                    if let Value::String(s) = value {
                        cmd_args.push(s);
                    } else {
                        cmd_args.push(value.to_string());
                    }
                }
            }

            // Execute command and return output as JSON
            let result = match std::process::Command::new(command).args(&cmd_args).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                    if output.status.success() {
                        Value::String(stdout)
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                        Value::String(format!("Error: {}", stderr))
                    }
                }
                Err(e) => Value::String(format!("Failed to execute {}: {}", command, e)),
            };

            let _ = sender.send(JsonValueResult::new(result));
        })
    }
}

/// Trait for converting command execution results to text
/// This is used in the tooling system to convert command outputs to strings
pub trait CandleExecToText {
    /// Convert the command execution result to a string
    fn exec_to_text(&self) -> String;
}

impl CandleExecToText for &str {
    fn exec_to_text(&self) -> String {
        self.to_string()
    }
}

impl CandleExecToText for String {
    fn exec_to_text(&self) -> String {
        self.as_str().exec_to_text()
    }
}

/// Re-export for ARCHITECTURE.md syntax compatibility
pub type CandleToolType<T> = CandleToolImpl<T>;

/// From implementations for transparent [("key", "value")] syntax in ARCHITECTURE.md
/// Helper function for transparent [("key", "value")] syntax in ARCHITECTURE.md
pub fn candle_tool_params_from_array<const N: usize>(
    arr: [(&'static str, &'static str); N],
) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for (k, v) in arr {
        map.insert(k, v);
    }
    map
}

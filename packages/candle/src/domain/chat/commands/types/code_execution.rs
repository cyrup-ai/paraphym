//! Code execution tool definitions with zero allocation patterns
//!
//! Provides comprehensive tool type definitions and structures for secure code execution
//! across multiple programming languages with owned strings allocated once for maximum
//! performance. No Arc usage, no locking.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::metadata::ResourceUsage;

/// Code execution tool for secure multi-language code execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionTool {
    /// Tool name (owned string allocated once)
    pub name: String,
    /// Programming language for execution
    pub language: CodeLanguage,
    /// Tool description (owned string allocated once)
    pub description: String,
    /// Example usage demonstrating the tool (owned string allocated once)
    pub example_usage: String,
    /// Security validation settings
    pub validation_config: ValidationConfig,
    /// Resource limits for execution
    pub resource_limits: ResourceLimits,
    /// Tool version for compatibility tracking
    pub version: String,
    /// Whether tool is experimental
    pub experimental: bool,
}

impl CodeExecutionTool {
    /// Create a new code execution tool with essential fields
    #[inline]
    pub fn new(
        name: impl Into<String>,
        language: CodeLanguage,
        description: impl Into<String>,
        example_usage: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            language,
            description: description.into(),
            example_usage: example_usage.into(),
            validation_config: ValidationConfig::default(),
            resource_limits: ResourceLimits::default(),
            version: "1.0.0".to_string(),
            experimental: false,
        }
    }

    /// Python analysis tool constructor for data analysis and calculations
    #[inline]
    pub fn python_analysis() -> Self {
        Self::new(
            "python_analysis",
            CodeLanguage::Python,
            "Execute Python code for data analysis, calculations, and scientific computing",
            r#"import pandas as pd
import numpy as np

# Load and analyze data
df = pd.read_csv('data.csv')
print(df.describe())
print(f"Average: {df['column'].mean()}")"#,
        )
        .with_validation_config(ValidationConfig::python_secure())
        .with_resource_limits(ResourceLimits::analysis_workload())
    }

    /// JavaScript processing tool constructor for JSON processing and API calls
    #[inline]
    pub fn javascript_processing() -> Self {
        Self::new(
            "javascript_processing",
            CodeLanguage::JavaScript,
            "Execute JavaScript code for JSON processing, API calls, and data transformation",
            r#"const data = {
    users: [
        { name: "Alice", age: 30 },
        { name: "Bob", age: 25 }
    ]
};

const averageAge = data.users.reduce((sum, user) => sum + user.age, 0) / data.users.length;
console.log(`Average age: ${averageAge}`);"#,
        )
        .with_validation_config(ValidationConfig::javascript_secure())
        .with_resource_limits(ResourceLimits::processing_workload())
    }

    /// Rust computation tool constructor for performance-critical calculations
    #[inline]
    pub fn rust_computation() -> Self {
        Self::new(
            "rust_computation",
            CodeLanguage::Rust,
            "Execute Rust code for performance-critical calculations and system programming",
            r#"fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let result = fibonacci(20);
    println!("Fibonacci(20) = {}", result);
}"#,
        )
        .with_validation_config(ValidationConfig::rust_secure())
        .with_resource_limits(ResourceLimits::computation_workload())
    }

    /// Go processing tool constructor for concurrent processing tasks
    #[inline]
    pub fn go_processing() -> Self {
        Self::new(
            "go_processing",
            CodeLanguage::Go,
            "Execute Go code for concurrent processing, networking, and system tasks",
            r#"package main

import (
    "fmt"
    "sync"
    "time"
)

func worker(id int, wg *sync.WaitGroup) {
    defer wg.Done()
    fmt.Printf("Worker %d starting\n", id)
    time.Sleep(time.Second)
    fmt.Printf("Worker %d done\n", id)
}

func main() {
    var wg sync.WaitGroup
    for i := 1; i <= 3; i++ {
        wg.Add(1)
        go worker(i, &wg)
    }
    wg.Wait()
    fmt.Println("All workers completed")
}"#,
        )
        .with_validation_config(ValidationConfig::go_secure())
        .with_resource_limits(ResourceLimits::processing_workload())
    }

    /// Bash automation tool constructor for system automation and file operations
    #[inline]
    pub fn bash_automation() -> Self {
        Self::new(
            "bash_automation",
            CodeLanguage::Bash,
            "Execute Bash scripts for system automation, file operations, and command orchestration",
            r#"#!/bin/bash

# Process files in directory
for file in *.txt; do
    if [ -f "$file" ]; then
        echo "Processing: $file"
        wc -l "$file"
    fi
done

echo "File processing complete""#,
        )
        .with_validation_config(ValidationConfig::bash_secure())
        .with_resource_limits(ResourceLimits::automation_workload())
    }

    /// Set validation configuration - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_validation_config(mut self, config: ValidationConfig) -> Self {
        self.validation_config = config;
        self
    }

    /// Set resource limits - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = limits;
        self
    }

    /// Set version - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Mark as experimental - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn experimental(mut self) -> Self {
        self.experimental = true;
        self
    }

    /// Validate code against tool's security configuration
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if code contains prohibited patterns or violates security rules
    #[inline]
    pub fn validate_code(&self, code: &str) -> Result<(), ValidationError> {
        self.validation_config.validate_code(code, &self.language)
    }

    /// Check if code execution would exceed resource limits
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if resource limits would be exceeded
    #[inline]
    pub fn check_resource_limits(&self, request: &CodeExecutionRequest) -> Result<(), ValidationError> {
        self.resource_limits.validate_request(request)
    }
}

/// Programming language enumeration for code execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodeLanguage {
    /// Python programming language
    Python,
    /// JavaScript programming language
    JavaScript,
    /// Rust programming language
    Rust,
    /// Go programming language
    Go,
    /// Bash shell scripting
    Bash,
}

impl CodeLanguage {
    /// Get language name as static string for zero allocation
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::Rust => "rust",
            Self::Go => "go",
            Self::Bash => "bash",
        }
    }

    /// Get file extension for the language
    #[inline]
    pub const fn file_extension(&self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::Rust => "rs",
            Self::Go => "go",
            Self::Bash => "sh",
        }
    }

    /// Get MIME type for the language
    #[inline]
    pub const fn mime_type(&self) -> &'static str {
        match self {
            Self::Python => "text/x-python",
            Self::JavaScript => "text/javascript",
            Self::Rust => "text/x-rust",
            Self::Go => "text/x-go",
            Self::Bash => "text/x-shellscript",
        }
    }
}

/// Code execution request with type-safe communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionRequest {
    /// Programming language for execution
    pub language: CodeLanguage,
    /// Source code to execute (owned string allocated once)
    pub code: String,
    /// Reasoning for code execution (owned string allocated once)
    pub reasoning: String,
    /// Execution timeout in seconds
    pub timeout_seconds: u64,
    /// Memory limit in bytes
    pub memory_limit_bytes: u64,
    /// CPU limit as percentage (0-100)
    pub cpu_limit_percent: u8,
    /// Network access permission
    pub network_access: bool,
    /// File system access permission
    pub filesystem_access: bool,
    /// Environment variables to set
    pub environment_variables: std::collections::HashMap<String, String>,
    /// Working directory for execution
    pub working_directory: Option<String>,
    /// Input data to provide to the executing code
    pub input_data: Option<String>,
    /// Expected output format
    pub expected_output_format: OutputFormat,
}

impl CodeExecutionRequest {
    /// Create a new code execution request with essential fields
    #[inline]
    pub fn new(
        language: CodeLanguage,
        code: impl Into<String>,
        reasoning: impl Into<String>,
    ) -> Self {
        Self {
            language,
            code: code.into(),
            reasoning: reasoning.into(),
            timeout_seconds: 30,
            memory_limit_bytes: 128 * 1024 * 1024, // 128MB
            cpu_limit_percent: 80,
            network_access: false,
            filesystem_access: false,
            environment_variables: std::collections::HashMap::new(),
            working_directory: None,
            input_data: None,
            expected_output_format: OutputFormat::Text,
        }
    }

    /// Set timeout - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Set memory limit - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_memory_limit(mut self, bytes: u64) -> Self {
        self.memory_limit_bytes = bytes;
        self
    }

    /// Set CPU limit - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_cpu_limit(mut self, percent: u8) -> Self {
        self.cpu_limit_percent = percent.min(100);
        self
    }

    /// Enable network access - builder pattern for fluent API
    #[inline]
    #[must_use]
    pub fn with_network_access(mut self) -> Self {
        self.network_access = true;
        self
    }

    /// Enable filesystem access - builder pattern for fluent API
    #[must_use]
    #[inline]
    pub fn with_filesystem_access(mut self) -> Self {
        self.filesystem_access = true;
        self
    }

    /// Set working directory - builder pattern for fluent API
    #[must_use]
    #[inline]
    pub fn with_working_directory(mut self, dir: impl Into<String>) -> Self {
        self.working_directory = Some(dir.into());
        self
    }

    /// Set input data - builder pattern for fluent API
    #[must_use]
    #[inline]
    pub fn with_input_data(mut self, data: impl Into<String>) -> Self {
        self.input_data = Some(data.into());
        self
    }

    /// Set expected output format - builder pattern for fluent API
    #[must_use]
    #[inline]
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.expected_output_format = format;
        self
    }

    /// Add environment variable - builder pattern for fluent API
    #[must_use]
    #[inline]
    pub fn with_env_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment_variables.insert(key.into(), value.into());
        self
    }
}

/// Expected output format for code execution results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Plain text output
    Text,
    /// JSON structured output
    Json,
    /// CSV tabular output
    Csv,
    /// Binary output (base64 encoded)
    Binary,
    /// Image output (base64 encoded)
    Image,
}

impl OutputFormat {
    /// Get format name as static string for zero allocation
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Binary => "binary",
            Self::Image => "image",
        }
    }
}

/// Code execution result with comprehensive information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionResult {
    /// Exit code from the executed process
    pub exit_code: i32,
    /// Standard output from execution (owned string allocated once)
    pub stdout: String,
    /// Standard error from execution (owned string allocated once)
    pub stderr: String,
    /// Execution duration in microseconds
    pub duration_us: u64,
    /// Resource usage during execution
    pub resource_usage: ResourceUsage,
    /// Execution status
    pub status: ExecutionStatus,
    /// Error information if execution failed
    pub error: Option<ExecutionError>,
    /// Validation warnings generated during execution
    pub warnings: Vec<String>,
    /// Output format of the result
    pub output_format: OutputFormat,
    /// Additional metadata about execution
    pub metadata: std::collections::HashMap<String, String>,
}

impl CodeExecutionResult {
    /// Create a successful execution result
    #[inline]
    pub fn success(
        exit_code: i32,
        stdout: impl Into<String>,
        stderr: impl Into<String>,
        duration_us: u64,
        resource_usage: ResourceUsage,
    ) -> Self {
        Self {
            exit_code,
            stdout: stdout.into(),
            stderr: stderr.into(),
            duration_us,
            resource_usage,
            status: ExecutionStatus::Success,
            error: None,
            warnings: Vec::new(),
            output_format: OutputFormat::Text,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a failed execution result
    #[inline]
    pub fn failure(
        exit_code: i32,
        stderr: impl Into<String>,
        duration_us: u64,
        error: ExecutionError,
    ) -> Self {
        Self {
            exit_code,
            stdout: String::new(),
            stderr: stderr.into(),
            duration_us,
            resource_usage: ResourceUsage::default(),
            status: ExecutionStatus::Failed,
            error: Some(error),
            warnings: Vec::new(),
            output_format: OutputFormat::Text,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a timeout execution result
    #[inline]
    pub fn timeout(duration_us: u64) -> Self {
        Self {
            exit_code: -1,
            stdout: String::new(),
            stderr: "Execution timed out".to_string(),
            duration_us,
            resource_usage: ResourceUsage::default(),
            status: ExecutionStatus::Timeout,
            error: Some(ExecutionError::Timeout),
            warnings: Vec::new(),
            output_format: OutputFormat::Text,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Check if execution was successful
    #[inline]
    pub fn is_success(&self) -> bool {
        matches!(self.status, ExecutionStatus::Success) && self.exit_code == 0
    }

    /// Check if execution failed
    #[inline]
    pub fn is_failure(&self) -> bool {
        matches!(self.status, ExecutionStatus::Failed)
    }

    /// Check if execution timed out
    #[inline]
    pub fn is_timeout(&self) -> bool {
        matches!(self.status, ExecutionStatus::Timeout)
    }

    /// Get execution duration as human readable string
    #[inline]
    pub fn duration_human(&self) -> String {
        let duration_us = self.duration_us;

        if duration_us < 1000 {
            format!("{duration_us}μs")
        } else if duration_us < 1_000_000 {
            format!("{:.1}ms", duration_us as f64 / 1000.0)
        } else if duration_us < 60_000_000 {
            format!("{:.2}s", duration_us as f64 / 1_000_000.0)
        } else {
            let minutes = duration_us / 60_000_000;
            let seconds = (duration_us % 60_000_000) / 1_000_000;
            format!("{minutes}m{seconds}s")
        }
    }

    /// Add warning message
    #[inline]
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Add metadata entry
    #[inline]
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// Execution status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// Execution completed successfully
    Success,
    /// Execution failed with error
    Failed,
    /// Execution timed out
    Timeout,
    /// Execution was cancelled
    Cancelled,
    /// Execution is still running
    Running,
}

impl ExecutionStatus {
    /// Get status name as static string for zero allocation
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::Running => "running",
        }
    }
}

/// Execution error types for comprehensive error handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionError {
    /// Execution timed out
    Timeout,
    /// Memory limit exceeded
    MemoryLimitExceeded,
    /// CPU limit exceeded
    CpuLimitExceeded,
    /// Syntax error in code
    SyntaxError { message: String },
    /// Runtime error during execution
    RuntimeError { message: String },
    /// Security violation detected
    SecurityViolation { message: String },
    /// Resource unavailable
    ResourceUnavailable { resource: String },
    /// Network access denied
    NetworkAccessDenied,
    /// Filesystem access denied
    FilesystemAccessDenied,
    /// Compilation failed
    CompilationFailed { message: String },
    /// Invalid language configuration
    InvalidLanguageConfig { message: String },
    /// System error
    SystemError { message: String },
}

impl ExecutionError {
    /// Get error message as string
    #[inline]
    pub fn message(&self) -> String {
        match self {
            Self::Timeout => "Execution timed out".to_string(),
            Self::MemoryLimitExceeded => "Memory limit exceeded during execution".to_string(),
            Self::CpuLimitExceeded => "CPU limit exceeded during execution".to_string(),
            Self::SyntaxError { message } => format!("Syntax error: {message}"),
            Self::RuntimeError { message } => format!("Runtime error: {message}"),
            Self::SecurityViolation { message } => format!("Security violation: {message}"),
            Self::ResourceUnavailable { resource } => format!("Resource unavailable: {resource}"),
            Self::NetworkAccessDenied => "Network access denied".to_string(),
            Self::FilesystemAccessDenied => "Filesystem access denied".to_string(),
            Self::CompilationFailed { message } => format!("Compilation failed: {message}"),
            Self::InvalidLanguageConfig { message } => format!("Invalid language configuration: {message}"),
            Self::SystemError { message } => format!("System error: {message}"),
        }
    }
}

/// Validation error for code execution with comprehensive error handling
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    /// Syntax validation failed
    #[error("Syntax validation failed: {message}")]
    SyntaxValidationFailed { message: String },
    
    /// Security validation failed
    #[error("Security validation failed: {message}")]
    SecurityValidationFailed { message: String },
    
    /// Resource limit validation failed
    #[error("Resource limit validation failed: {message}")]
    ResourceLimitValidationFailed { message: String },
    
    /// Language not supported
    #[error("Language not supported: {language}")]
    LanguageNotSupported { language: String },
    
    /// Code contains prohibited patterns
    #[error("Code contains prohibited patterns: {patterns:?}")]
    ProhibitedPatterns { patterns: Vec<String> },
    
    /// Code exceeds size limits
    #[error("Code exceeds size limit: {size} bytes (max: {limit} bytes)")]
    CodeSizeExceeded { size: usize, limit: usize },
    
    /// Invalid encoding detected
    #[error("Invalid encoding detected: {encoding}")]
    InvalidEncoding { encoding: String },
    
    /// Timeout value invalid
    #[error("Invalid timeout value: {timeout} seconds (max: {max_timeout} seconds)")]
    InvalidTimeout { timeout: u64, max_timeout: u64 },
    
    /// Memory limit invalid
    #[error("Invalid memory limit: {memory} bytes (max: {max_memory} bytes)")]
    InvalidMemoryLimit { memory: u64, max_memory: u64 },
    
    /// CPU limit invalid
    #[error("Invalid CPU limit: {cpu}% (max: {max_cpu}%)")]
    InvalidCpuLimit { cpu: u8, max_cpu: u8 },
    
    /// Environment variable validation failed
    #[error("Environment variable validation failed: {variable} = {value}")]
    InvalidEnvironmentVariable { variable: String, value: String },
    
    /// Working directory validation failed
    #[error("Working directory validation failed: {directory}")]
    InvalidWorkingDirectory { directory: String },
}

impl ValidationError {
    /// Create a syntax validation error
    #[inline]
    pub fn syntax_failed(message: impl Into<String>) -> Self {
        Self::SyntaxValidationFailed { message: message.into() }
    }

    /// Create a security validation error
    #[inline]
    pub fn security_failed(message: impl Into<String>) -> Self {
        Self::SecurityValidationFailed { message: message.into() }
    }

    /// Create a resource limit validation error
    #[inline]
    pub fn resource_limit_failed(message: impl Into<String>) -> Self {
        Self::ResourceLimitValidationFailed { message: message.into() }
    }

    /// Create a language not supported error
    #[inline]
    pub fn language_not_supported(language: impl Into<String>) -> Self {
        Self::LanguageNotSupported { language: language.into() }
    }

    /// Create a prohibited patterns error
    #[inline]
    pub fn prohibited_patterns(patterns: Vec<String>) -> Self {
        Self::ProhibitedPatterns { patterns }
    }

    /// Create a code size exceeded error
    #[inline]
    pub fn code_size_exceeded(size: usize, limit: usize) -> Self {
        Self::CodeSizeExceeded { size, limit }
    }
}

/// Security validation configuration for code execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Maximum code size in bytes
    pub max_code_size_bytes: usize,
    /// Prohibited patterns in code
    pub prohibited_patterns: Vec<String>,
    /// Allowed imports/modules
    pub allowed_imports: Vec<String>,
    /// Prohibited imports/modules  
    pub prohibited_imports: Vec<String>,
    /// Enable syntax validation
    pub syntax_validation: bool,
    /// Enable security scanning
    pub security_scanning: bool,
    /// Maximum nesting depth for code structures
    pub max_nesting_depth: u32,
    /// Maximum number of loops allowed
    pub max_loops: u32,
    /// Maximum function call depth
    pub max_call_depth: u32,
}

impl ValidationConfig {
    /// Create default validation configuration
    #[inline]
    pub fn default() -> Self {
        Self {
            max_code_size_bytes: 64 * 1024, // 64KB
            prohibited_patterns: vec![
                "eval(".to_string(),
                "exec(".to_string(),
                "__import__".to_string(),
                "subprocess".to_string(),
                "os.system".to_string(),
            ],
            allowed_imports: Vec::new(),
            prohibited_imports: vec![
                "subprocess".to_string(),
                "os".to_string(),
                "sys".to_string(),
                "socket".to_string(),
                "urllib".to_string(),
                "requests".to_string(),
            ],
            syntax_validation: true,
            security_scanning: true,
            max_nesting_depth: 10,
            max_loops: 100,
            max_call_depth: 50,
        }
    }

    /// Create secure Python validation configuration
    #[inline]
    pub fn python_secure() -> Self {
        Self {
            max_code_size_bytes: 128 * 1024, // 128KB
            prohibited_patterns: vec![
                "eval(".to_string(),
                "exec(".to_string(),
                "__import__".to_string(),
                "subprocess".to_string(),
                "os.system".to_string(),
                "open(".to_string(),
                "file(".to_string(),
                "input(".to_string(),
                "raw_input(".to_string(),
            ],
            allowed_imports: vec![
                "math".to_string(),
                "random".to_string(),
                "datetime".to_string(),
                "json".to_string(),
                "pandas".to_string(),
                "numpy".to_string(),
                "matplotlib".to_string(),
                "seaborn".to_string(),
            ],
            prohibited_imports: vec![
                "subprocess".to_string(),
                "os".to_string(),
                "sys".to_string(),
                "socket".to_string(),
                "urllib".to_string(),
                "requests".to_string(),
                "pickle".to_string(),
                "ctypes".to_string(),
            ],
            syntax_validation: true,
            security_scanning: true,
            max_nesting_depth: 8,
            max_loops: 50,
            max_call_depth: 30,
        }
    }

    /// Create secure JavaScript validation configuration
    #[inline]
    pub fn javascript_secure() -> Self {
        Self {
            max_code_size_bytes: 64 * 1024, // 64KB
            prohibited_patterns: vec![
                "eval(".to_string(),
                "Function(".to_string(),
                "require(".to_string(),
                "import(".to_string(),
                "fetch(".to_string(),
                "XMLHttpRequest".to_string(),
                "localStorage".to_string(),
                "sessionStorage".to_string(),
                "document".to_string(),
                "window".to_string(),
            ],
            allowed_imports: vec![
                "Math".to_string(),
                "Date".to_string(),
                "JSON".to_string(),
                "Array".to_string(),
                "Object".to_string(),
                "String".to_string(),
                "Number".to_string(),
            ],
            prohibited_imports: vec![
                "fs".to_string(),
                "http".to_string(),
                "https".to_string(),
                "net".to_string(),
                "child_process".to_string(),
                "cluster".to_string(),
                "crypto".to_string(),
            ],
            syntax_validation: true,
            security_scanning: true,
            max_nesting_depth: 6,
            max_loops: 30,
            max_call_depth: 20,
        }
    }

    /// Create secure Rust validation configuration
    #[inline]
    pub fn rust_secure() -> Self {
        Self {
            max_code_size_bytes: 256 * 1024, // 256KB
            prohibited_patterns: vec![
                "unsafe".to_string(),
                "std::process".to_string(),
                "std::fs".to_string(),
                "std::net".to_string(),
                "libc::".to_string(),
                "ptr::".to_string(),
                "mem::transmute".to_string(),
            ],
            allowed_imports: vec![
                "std::collections".to_string(),
                "std::iter".to_string(),
                "std::ops".to_string(),
                "std::fmt".to_string(),
                "std::str".to_string(),
                "std::vec".to_string(),
            ],
            prohibited_imports: vec![
                "std::process".to_string(),
                "std::fs".to_string(),
                "std::net".to_string(),
                "std::os".to_string(),
                "std::thread".to_string(),
                "std::sync".to_string(),
            ],
            syntax_validation: true,
            security_scanning: true,
            max_nesting_depth: 12,
            max_loops: 100,
            max_call_depth: 40,
        }
    }

    /// Create secure Go validation configuration
    #[inline]
    pub fn go_secure() -> Self {
        Self {
            max_code_size_bytes: 128 * 1024, // 128KB
            prohibited_patterns: vec![
                "os.Exec".to_string(),
                "os.Start".to_string(),
                "syscall.".to_string(),
                "unsafe.".to_string(),
                "reflect.".to_string(),
                "net/http".to_string(),
                "net/url".to_string(),
            ],
            allowed_imports: vec![
                "fmt".to_string(),
                "math".to_string(),
                "sort".to_string(),
                "strings".to_string(),
                "strconv".to_string(),
                "time".to_string(),
                "sync".to_string(),
            ],
            prohibited_imports: vec![
                "os".to_string(),
                "os/exec".to_string(),
                "net".to_string(),
                "net/http".to_string(),
                "syscall".to_string(),
                "unsafe".to_string(),
                "plugin".to_string(),
            ],
            syntax_validation: true,
            security_scanning: true,
            max_nesting_depth: 10,
            max_loops: 75,
            max_call_depth: 35,
        }
    }

    /// Create secure Bash validation configuration
    #[inline]
    pub fn bash_secure() -> Self {
        Self {
            max_code_size_bytes: 32 * 1024, // 32KB
            prohibited_patterns: vec![
                "rm -rf".to_string(),
                "chmod".to_string(),
                "chown".to_string(),
                "sudo".to_string(),
                "su ".to_string(),
                "passwd".to_string(),
                "curl".to_string(),
                "wget".to_string(),
                "nc ".to_string(),
                "netcat".to_string(),
                "> /dev".to_string(),
                "dd if=".to_string(),
                "mkfs".to_string(),
                "fdisk".to_string(),
            ],
            allowed_imports: Vec::new(),
            prohibited_imports: Vec::new(),
            syntax_validation: true,
            security_scanning: true,
            max_nesting_depth: 5,
            max_loops: 20,
            max_call_depth: 15,
        }
    }

    /// Validate code against this configuration
    #[inline]
    pub fn validate_code(&self, code: &str, language: &CodeLanguage) -> Result<(), ValidationError> {
        // Check code size
        if code.len() > self.max_code_size_bytes {
            return Err(ValidationError::code_size_exceeded(
                code.len(),
                self.max_code_size_bytes,
            ));
        }

        // Check prohibited patterns
        if self.security_scanning {
            let mut found_patterns = Vec::new();
            for pattern in &self.prohibited_patterns {
                if code.contains(pattern) {
                    found_patterns.push(pattern.clone());
                }
            }
            if !found_patterns.is_empty() {
                return Err(ValidationError::prohibited_patterns(found_patterns));
            }
        }

        // Language-specific validation
        match language {
            CodeLanguage::Python => self.validate_python_code(code),
            CodeLanguage::JavaScript => Self::validate_javascript_code(code),
            CodeLanguage::Rust => Self::validate_rust_code(code),
            CodeLanguage::Go => Self::validate_go_code(code),
            CodeLanguage::Bash => Self::validate_bash_code(code),
        }
    }

    /// Validate Python-specific patterns
    #[inline]
    fn validate_python_code(&self, code: &str) -> Result<(), ValidationError> {
        // Check for prohibited imports
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                for prohibited in &self.prohibited_imports {
                    if trimmed.contains(prohibited) {
                        return Err(ValidationError::security_failed(
                            format!("Prohibited import: {prohibited}")
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validate JavaScript-specific patterns
    #[inline]
    fn validate_javascript_code(code: &str) -> Result<(), ValidationError> {
        // Check for Node.js requires
        if code.contains("require(") {
            return Err(ValidationError::security_failed(
                "Node.js require() is not allowed"
            ));
        }
        Ok(())
    }

    /// Validate Rust-specific patterns
    #[inline]
    fn validate_rust_code(code: &str) -> Result<(), ValidationError> {
        // Check for unsafe blocks
        if code.contains("unsafe") {
            return Err(ValidationError::security_failed(
                "Unsafe Rust code is not allowed"
            ));
        }
        Ok(())
    }

    /// Validate Go-specific patterns
    #[inline]
    fn validate_go_code(code: &str) -> Result<(), ValidationError> {
        // Check for dangerous syscalls
        if code.contains("syscall.") {
            return Err(ValidationError::security_failed(
                "Direct syscalls are not allowed"
            ));
        }
        Ok(())
    }

    /// Validate Bash-specific patterns
    #[inline]
    fn validate_bash_code(code: &str) -> Result<(), ValidationError> {
        // Check for dangerous file operations
        if code.contains("rm -rf /") {
            return Err(ValidationError::security_failed(
                "Destructive file operations are not allowed"
            ));
        }
        Ok(())
    }
}

/// Resource limits for code execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum execution time in seconds
    pub max_execution_time_seconds: u64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
    /// Maximum CPU usage as percentage (0-100)
    pub max_cpu_percent: u8,
    /// Maximum number of file operations
    pub max_file_operations: u32,
    /// Maximum number of network requests
    pub max_network_requests: u32,
    /// Maximum output size in bytes
    pub max_output_size_bytes: u64,
    /// Maximum number of processes/threads
    pub max_processes: u32,
}

impl ResourceLimits {
    /// Create default resource limits
    #[inline]
    pub fn default() -> Self {
        Self {
            max_execution_time_seconds: 30,
            max_memory_bytes: 128 * 1024 * 1024, // 128MB
            max_cpu_percent: 80,
            max_file_operations: 100,
            max_network_requests: 0, // No network by default
            max_output_size_bytes: 1024 * 1024, // 1MB
            max_processes: 1,
        }
    }

    /// Create resource limits for analysis workloads
    #[inline]
    pub fn analysis_workload() -> Self {
        Self {
            max_execution_time_seconds: 60,
            max_memory_bytes: 512 * 1024 * 1024, // 512MB
            max_cpu_percent: 90,
            max_file_operations: 1000,
            max_network_requests: 0,
            max_output_size_bytes: 10 * 1024 * 1024, // 10MB
            max_processes: 1,
        }
    }

    /// Create resource limits for processing workloads
    #[inline]
    pub fn processing_workload() -> Self {
        Self {
            max_execution_time_seconds: 45,
            max_memory_bytes: 256 * 1024 * 1024, // 256MB
            max_cpu_percent: 85,
            max_file_operations: 500,
            max_network_requests: 10,
            max_output_size_bytes: 5 * 1024 * 1024, // 5MB
            max_processes: 2,
        }
    }

    /// Create resource limits for computation workloads
    #[inline]
    pub fn computation_workload() -> Self {
        Self {
            max_execution_time_seconds: 90,
            max_memory_bytes: 1024 * 1024 * 1024, // 1GB
            max_cpu_percent: 95,
            max_file_operations: 100,
            max_network_requests: 0,
            max_output_size_bytes: 2 * 1024 * 1024, // 2MB
            max_processes: 1,
        }
    }

    /// Create resource limits for automation workloads
    #[inline]
    pub fn automation_workload() -> Self {
        Self {
            max_execution_time_seconds: 120,
            max_memory_bytes: 64 * 1024 * 1024, // 64MB
            max_cpu_percent: 70,
            max_file_operations: 2000,
            max_network_requests: 0,
            max_output_size_bytes: 512 * 1024, // 512KB
            max_processes: 5,
        }
    }

    /// Validate execution request against these limits
    #[inline]
    pub fn validate_request(&self, request: &CodeExecutionRequest) -> Result<(), ValidationError> {
        if request.timeout_seconds > self.max_execution_time_seconds {
            return Err(ValidationError::InvalidTimeout {
                timeout: request.timeout_seconds,
                max_timeout: self.max_execution_time_seconds,
            });
        }

        if request.memory_limit_bytes > self.max_memory_bytes {
            return Err(ValidationError::InvalidMemoryLimit {
                memory: request.memory_limit_bytes,
                max_memory: self.max_memory_bytes,
            });
        }

        if request.cpu_limit_percent > self.max_cpu_percent {
            return Err(ValidationError::InvalidCpuLimit {
                cpu: request.cpu_limit_percent,
                max_cpu: self.max_cpu_percent,
            });
        }

        Ok(())
    }
}
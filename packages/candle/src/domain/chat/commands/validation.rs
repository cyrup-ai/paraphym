//! Command validation and sanitization
//!
//! Provides comprehensive input validation with zero-allocation patterns and blazing-fast
//! validation algorithms for production-ready security and error handling.

use std::collections::HashMap;
use std::sync::LazyLock;
use regex::Regex;

use super::types::ImmutableChatCommand;

/// Command validator with comprehensive validation rules
#[derive(Debug, Clone)]
pub struct CommandValidator {
    /// Maximum command length
    max_command_length: usize,
    /// Maximum parameter count
    max_parameter_count: usize,
    /// Maximum parameter value length
    max_parameter_value_length: usize,
    /// Allowed file extensions for path parameters
    allowed_extensions: Vec<String>,
    /// Blocked patterns for security
    blocked_patterns: Vec<Regex>,
}

impl Default for CommandValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandValidator {
    /// Create a new command validator with default settings
    pub fn new() -> Self {
        Self {
            max_command_length: 1024,
            max_parameter_count: 50,
            max_parameter_value_length: 512,
            allowed_extensions: vec![
                "txt".to_string(),
                "md".to_string(),
                "json".to_string(),
                "csv".to_string(),
                "html".to_string(),
                "pdf".to_string(),
            ],
            blocked_patterns: vec![
                // Prevent command injection
                Regex::new(r"[;&|`$()]").expect("Command injection regex should be valid"),
                // Prevent path traversal
                Regex::new(r"\.\.[\\/]").expect("Path traversal regex should be valid"),
                // Prevent script injection
                Regex::new(r"<script[^>]*>").expect("Script injection regex should be valid"),
            ],
        }
    }

    /// Validate a command with comprehensive checks
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if any command parameter fails validation checks
    /// including string length, integer ranges, file paths, or command-specific rules
    pub fn validate_command(&self, command: &ImmutableChatCommand) -> Result<(), ValidationError> {
        match command {
            ImmutableChatCommand::Help { command, .. } => {
                if let Some(cmd) = command {
                    self.validate_string_parameter("command", cmd, false)?;
                }
            }
            ImmutableChatCommand::Clear { keep_last, .. } => {
                if let Some(n) = keep_last {
                    Self::validate_integer_parameter("keep_last", i64::try_from(*n).unwrap_or(i64::MAX), Some(1), Some(1000))?;
                }
            }
            ImmutableChatCommand::Export { format, output, .. } => {
                Self::validate_enum_parameter(
                    "format",
                    format,
                    &["json", "markdown", "pdf", "html"],
                )?;
                if let Some(path) = output {
                    self.validate_path_parameter("output", path)?;
                }
            }
            ImmutableChatCommand::Config { key, value, .. } => {
                if let Some(k) = key {
                    self.validate_config_key(k)?;
                }
                if let Some(v) = value {
                    self.validate_config_value(v)?;
                }
            }
            ImmutableChatCommand::Search { query, limit, .. } => {
                self.validate_string_parameter("query", query, false)?;
                if let Some(n) = limit {
                    Self::validate_integer_parameter("limit", i64::try_from(*n).unwrap_or(i64::MAX), Some(1), Some(100))?;
                }
            }
            ImmutableChatCommand::Template {
                name,
                content,
                variables,
                ..
            } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
                if let Some(c) = content {
                    self.validate_content_parameter("content", c)?;
                }
                self.validate_variables(variables)?;
            }
            ImmutableChatCommand::Macro { name, commands, .. } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
                for (i, cmd) in commands.iter().enumerate() {
                    self.validate_string_parameter(&format!("command_{i}"), cmd, false)?;
                }
            }
            ImmutableChatCommand::Branch { name, source, .. } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
                if let Some(s) = source {
                    self.validate_name_parameter("source", s)?;
                }
            }
            ImmutableChatCommand::Session { name, .. } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
            }
            ImmutableChatCommand::Tool { name, args, .. } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
                self.validate_tool_args(args)?;
            }
            ImmutableChatCommand::Stats { period, .. } => {
                if let Some(p) = period {
                    Self::validate_enum_parameter("period", p, &["day", "week", "month", "year"])?;
                }
            }
            ImmutableChatCommand::Theme {
                name, properties, ..
            } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
                self.validate_theme_properties(properties)?;
            }
            ImmutableChatCommand::Debug { level, .. } => {
                if let Some(l) = level {
                    Self::validate_enum_parameter(
                        "level",
                        l,
                        &["trace", "debug", "info", "warn", "error"],
                    )?;
                }
            }
            ImmutableChatCommand::History { filter, .. } => {
                if let Some(f) = filter {
                    self.validate_string_parameter("filter", f, true)?;
                }
            }
            ImmutableChatCommand::Save { name, location, .. } => {
                if let Some(n) = name {
                    self.validate_name_parameter("name", n)?;
                }
                if let Some(l) = location {
                    self.validate_path_parameter("location", l)?;
                }
            }
            ImmutableChatCommand::Load { name, location, .. } => {
                self.validate_string_parameter("name", name, false)?;
                if let Some(l) = location {
                    self.validate_path_parameter("location", l)?;
                }
            }
            ImmutableChatCommand::Import { source, .. } => {
                self.validate_string_parameter("source", source, false)?;
                // Additional import source validation could be added here
            }
            ImmutableChatCommand::Settings { key, value, .. } => {
                if let Some(k) = key {
                    self.validate_string_parameter("key", k, false)?;
                }
                if let Some(v) = value {
                    self.validate_string_parameter("value", v, true)?;
                }
            }
            ImmutableChatCommand::Custom { name, args, .. } => {
                self.validate_name_parameter("name", name)?;
                for (k, v) in args {
                    self.validate_string_parameter(&format!("arg_{k}"), v, true)?;
                }
            }
            ImmutableChatCommand::Copy {
                message_id,
                content,
                format: _,
            } => {
                if let Some(msg_id) = message_id {
                    self.validate_string_parameter("message_id", msg_id, false)?;
                }
                if let Some(ctnt) = content {
                    self.validate_string_parameter("content", ctnt, false)?;
                }
                // Format is an enum, so no need to validate
            }
            ImmutableChatCommand::Retry {
                command, attempts, ..
            } => {
                if let Some(cmd) = command {
                    self.validate_string_parameter("command", cmd, false)?;
                }
                if let Some(attempts) = attempts
                    && (*attempts == 0 || *attempts > 100)
                {
                    return Err(ValidationError::InvalidParameterFormat {
                        parameter: "attempts".to_string(),
                        value: attempts.to_string(),
                        expected_format: "1-100".to_string(),
                    });
                }
            }
            ImmutableChatCommand::Undo { count, .. } => {
                if let Some(cnt) = count
                    && (*cnt == 0 || *cnt > 1000)
                {
                    return Err(ValidationError::InvalidParameterFormat {
                        parameter: "count".to_string(),
                        value: cnt.to_string(),
                        expected_format: "1-1000".to_string(),
                    });
                }
            }
            ImmutableChatCommand::Chat {
                message,
                context,
                priority,
            } => {
                self.validate_string_parameter("message", message, false)?;
                if let Some(ctx) = context {
                    self.validate_string_parameter("context", ctx, true)?;
                }
                if *priority > 10 {
                    return Err(ValidationError::InvalidParameterFormat {
                        parameter: "priority".to_string(),
                        value: priority.to_string(),
                        expected_format: "0-10".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validate string parameter
    fn validate_string_parameter(
        &self,
        name: &str,
        value: &str,
        allow_empty: bool,
    ) -> Result<(), ValidationError> {
        if !allow_empty && value.is_empty() {
            return Err(ValidationError::EmptyParameter {
                parameter: name.to_string(),
            });
        }

        if value.len() > self.max_parameter_value_length {
            return Err(ValidationError::ParameterTooLong {
                parameter: name.to_string(),
                max_length: self.max_parameter_value_length,
                actual_length: value.len(),
            });
        }

        // Check for blocked patterns
        for pattern in &self.blocked_patterns {
            if pattern.is_match(value) {
                return Err(ValidationError::SecurityViolation {
                    parameter: name.to_string(),
                    detail: "Contains blocked pattern".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate integer parameter
    fn validate_integer_parameter(
        name: &str,
        value: i64,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Result<(), ValidationError> {
        if let Some(min_val) = min
            && value < min_val
        {
            return Err(ValidationError::ParameterOutOfRange {
                parameter: name.to_string(),
                value: value.to_string(),
                min: Some(min_val.to_string()),
                max: max.map(|m| m.to_string()),
            });
        }

        if let Some(max_val) = max
            && value > max_val
        {
            return Err(ValidationError::ParameterOutOfRange {
                parameter: name.to_string(),
                value: value.to_string(),
                min: min.map(|m| m.to_string()),
                max: Some(max_val.to_string()),
            });
        }

        Ok(())
    }

    /// Validate enum parameter
    fn validate_enum_parameter(
        name: &str,
        value: &str,
        allowed: &[&str],
    ) -> Result<(), ValidationError> {
        if !allowed.contains(&value) {
            return Err(ValidationError::InvalidEnumValue {
                parameter: name.to_string(),
                value: value.to_string(),
                allowed_values: allowed.iter().map(ToString::to_string).collect(),
            });
        }
        Ok(())
    }

    /// Validate path parameter
    fn validate_path_parameter(&self, name: &str, path: &str) -> Result<(), ValidationError> {
        // Basic string validation
        self.validate_string_parameter(name, path, false)?;

        // Check for path traversal attempts
        if path.contains("..") {
            return Err(ValidationError::SecurityViolation {
                parameter: name.to_string(),
                detail: "Path traversal attempt detected".to_string(),
            });
        }

        // Validate file extension if present
        if let Some(ext_pos) = path.rfind('.') {
            let extension = &path[ext_pos + 1..];
            if !self
                .allowed_extensions
                .iter()
                .any(|ext| ext.as_ref() as &str == extension)
            {
                return Err(ValidationError::InvalidFileExtension {
                    parameter: name.to_string(),
                    extension: extension.to_string(),
                    allowed_extensions: self.allowed_extensions.clone(),
                });
            }
        }

        Ok(())
    }

    /// Validate configuration key
    fn validate_config_key(&self, key: &str) -> Result<(), ValidationError> {
        self.validate_string_parameter("key", key, false)?;

        // Config keys should be alphanumeric with dots and underscores
        let config_key_regex = Regex::new(r"^[a-zA-Z0-9._-]+$")
            .expect("Config key regex should be valid");
        if !config_key_regex.is_match(key) {
            return Err(ValidationError::InvalidParameterFormat {
                parameter: "key".to_string(),
                value: key.to_string(),
                expected_format: "alphanumeric with dots, underscores, and hyphens".to_string(),
            });
        }

        Ok(())
    }

    /// Validate configuration value
    fn validate_config_value(&self, value: &str) -> Result<(), ValidationError> {
        self.validate_string_parameter("value", value, true)?;
        Ok(())
    }

    /// Validate name parameter (for templates, macros, etc.)
    fn validate_name_parameter(&self, param_name: &str, name: &str) -> Result<(), ValidationError> {
        self.validate_string_parameter(param_name, name, false)?;

        // Names should be alphanumeric with underscores and hyphens
        let name_regex = Regex::new(r"^[a-zA-Z0-9_-]+$")
            .expect("Name validation regex should be valid");
        if !name_regex.is_match(name) {
            return Err(ValidationError::InvalidParameterFormat {
                parameter: "param_name".to_string(),
                value: name.to_string(),
                expected_format: "alphanumeric with underscores and hyphens".to_string(),
            });
        }

        Ok(())
    }

    /// Validate content parameter
    fn validate_content_parameter(&self, name: &str, content: &str) -> Result<(), ValidationError> {
        // Allow longer content but still validate
        if content.len() > self.max_parameter_value_length * 4 {
            return Err(ValidationError::ParameterTooLong {
                parameter: name.to_string(),
                max_length: self.max_parameter_value_length * 4,
                actual_length: content.len(),
            });
        }

        // Check for script injection attempts
        let script_regex = Regex::new(r"<script[^>]*>.*?</script>")
            .expect("Script injection detection regex should be valid");
        if script_regex.is_match(content) {
            return Err(ValidationError::SecurityViolation {
                parameter: name.to_string(),
                detail: "Script injection attempt detected".to_string(),
            });
        }

        Ok(())
    }

    /// Validate template/macro variables
    fn validate_variables(
        &self,
        variables: &HashMap<String, String>,
    ) -> Result<(), ValidationError> {
        if variables.len() > self.max_parameter_count {
            return Err(ValidationError::TooManyParameters {
                max_count: self.max_parameter_count,
                actual_count: variables.len(),
            });
        }

        for (key, value) in variables {
            self.validate_name_parameter("variable_key", key)?;
            self.validate_string_parameter("variable_value", value, true)?;
        }

        Ok(())
    }

    /// Validate tool arguments
    fn validate_tool_args(&self, args: &HashMap<String, String>) -> Result<(), ValidationError> {
        if args.len() > self.max_parameter_count {
            return Err(ValidationError::TooManyParameters {
                max_count: self.max_parameter_count,
                actual_count: args.len(),
            });
        }

        for (key, value) in args {
            self.validate_string_parameter("arg_key", key, false)?;
            self.validate_string_parameter("arg_value", value, true)?;
        }

        Ok(())
    }

    /// Validate theme properties
    fn validate_theme_properties(
        &self,
        properties: &HashMap<String, String>,
    ) -> Result<(), ValidationError> {
        if properties.len() > self.max_parameter_count {
            return Err(ValidationError::TooManyParameters {
                max_count: self.max_parameter_count,
                actual_count: properties.len(),
            });
        }

        for (key, value) in properties {
            self.validate_string_parameter("property_key", key, false)?;
            self.validate_string_parameter("property_value", value, true)?;
        }

        Ok(())
    }

    /// Sanitize input string
    pub fn sanitize_input(&self, input: &str) -> String {
        // Remove null bytes
        let sanitized = input.replace('\0', "");

        // Limit length
        if sanitized.len() > self.max_command_length {
            sanitized[..self.max_command_length].to_string()
        } else {
            sanitized
        }
    }

    /// Check if input is safe
    pub fn is_safe_input(&self, input: &str) -> bool {
        // Check length
        if input.len() > self.max_command_length {
            return false;
        }

        // Check for blocked patterns
        for pattern in &self.blocked_patterns {
            if pattern.is_match(input) {
                return false;
            }
        }

        true
    }
}

/// Validation error types
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    /// Parameter cannot be empty
    #[error("Parameter '{parameter}' cannot be empty")]
    EmptyParameter {
        /// Parameter name
        parameter: String,
    },

    /// Parameter exceeds maximum length
    #[error("Parameter '{parameter}' is too long: {actual_length} > {max_length}")]
    ParameterTooLong {
        /// Parameter name
        parameter: String,
        /// Maximum allowed length
        max_length: usize,
        /// Actual length provided
        actual_length: usize,
    },

    /// Parameter value is out of range
    #[error("Parameter '{parameter}' is out of range: {value} (min: {min:?}, max: {max:?})")]
    ParameterOutOfRange {
        /// Parameter name
        parameter: String,
        /// Parameter value provided
        value: String,
        /// Minimum allowed value
        min: Option<String>,
        /// Maximum allowed value
        max: Option<String>,
    },

    /// Parameter has invalid enum value
    #[error("Parameter '{parameter}' has invalid value '{value}', allowed: {allowed_values:?}")]
    InvalidEnumValue {
        /// Parameter name
        parameter: String,
        /// Invalid value provided
        value: String,
        /// List of allowed values
        allowed_values: Vec<String>,
    },

    /// Parameter has invalid format
    #[error("Parameter '{parameter}' has invalid format '{value}', expected: {expected_format}")]
    InvalidParameterFormat {
        /// Parameter name
        parameter: String,
        /// Invalid value provided
        value: String,
        /// Expected format description
        expected_format: String,
    },

    /// Parameter has invalid file extension
    #[error(
        "Parameter '{parameter}' has invalid file extension '{extension}', allowed: {allowed_extensions:?}"
    )]
    InvalidFileExtension {
        /// Parameter name with invalid extension
        parameter: String,
        /// The invalid extension that was provided
        extension: String,
        /// List of allowed file extensions
        allowed_extensions: Vec<String>,
    },

    /// Too many parameters provided
    #[error("Too many parameters: {actual_count} > {max_count}")]
    TooManyParameters {
        /// Maximum allowed parameter count
        max_count: usize,
        /// Actual parameter count provided
        actual_count: usize,
    },

    /// Security violation detected in parameter
    #[error("Security violation in parameter '{parameter}': {detail}")]
    SecurityViolation {
        /// Parameter name where violation occurred
        parameter: String,
        /// Details of the security violation
        detail: String,
    },
}

/// Global validator instance
static GLOBAL_VALIDATOR: LazyLock<CommandValidator> = LazyLock::new(CommandValidator::new);

/// Get global validator
pub fn get_global_validator() -> &'static CommandValidator {
    &GLOBAL_VALIDATOR
}

/// Validate command using global validator
///
/// # Errors
///
/// Returns `ValidationError` if command validation fails (see `CommandValidator::validate_command`)
pub fn validate_global_command(command: &ImmutableChatCommand) -> Result<(), ValidationError> {
    get_global_validator().validate_command(command)
}

/// Sanitize input using global validator
pub fn sanitize_global_input(input: &str) -> String {
    get_global_validator().sanitize_input(input)
}

/// Check if input is safe using global validator
pub fn is_global_safe_input(input: &str) -> bool {
    get_global_validator().is_safe_input(input)
}

//! Command-specific validation implementations
//!
//! Provides validation logic for each command type in the ImmutableChatCommand enum.
//! Each validator focuses on the specific requirements of its command.

use super::errors::ValidationError;
use super::parameter_validators::ValidationConfig;
use std::collections::HashMap;

impl ValidationConfig {
    /// Validate Help command - checks optional command parameter
    #[inline]
    pub fn validate_help_command(&self, command: Option<&String>) -> Result<(), ValidationError> {
        if let Some(cmd) = command {
            self.validate_string_parameter("command", cmd, false)?;
        }
        Ok(())
    }

    /// Validate Clear command - checks `keep_last` range
    #[inline]
    pub fn validate_clear_command(keep_last: Option<&usize>) -> Result<(), ValidationError> {
        if let Some(n) = keep_last {
            Self::validate_integer_parameter(
                "keep_last",
                i64::try_from(*n).unwrap_or(i64::MAX),
                Some(1),
                Some(1000),
            )?;
        }
        Ok(())
    }

    /// Validate Export command - checks format enum and output path
    #[inline]
    pub fn validate_export_command(
        &self,
        format: &str,
        output: Option<&String>,
    ) -> Result<(), ValidationError> {
        Self::validate_enum_parameter("format", format, &["json", "markdown", "pdf", "html"])?;
        if let Some(path) = output {
            self.validate_path_parameter("output", path)?;
        }
        Ok(())
    }

    /// Validate Config command - checks key/value format
    #[inline]
    pub fn validate_config_command(
        &self,
        key: Option<&str>,
        value: Option<&str>,
    ) -> Result<(), ValidationError> {
        if let Some(k) = key {
            self.validate_config_key(k)?;
        }
        if let Some(v) = value {
            self.validate_config_value(v)?;
        }
        Ok(())
    }

    /// Validate Search command - checks query and limit
    #[inline]
    pub fn validate_search_command(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<(), ValidationError> {
        self.validate_string_parameter("query", query, false)?;
        if let Some(n) = limit {
            Self::validate_integer_parameter(
                "limit",
                i64::try_from(n).unwrap_or(i64::MAX),
                Some(1),
                Some(100),
            )?;
        }
        Ok(())
    }

    /// Validate Template command - checks name, content, and variables
    #[inline]
    pub fn validate_template_command(
        &self,
        name: Option<&str>,
        content: Option<&str>,
        variables: &HashMap<String, String>,
    ) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        if let Some(c) = content {
            self.validate_content_parameter("content", c)?;
        }
        self.validate_variables(variables)?;
        Ok(())
    }

    /// Validate Macro command - checks name and commands list
    #[inline]
    pub fn validate_macro_command(
        &self,
        name: Option<&str>,
        commands: &[String],
    ) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        for (i, cmd) in commands.iter().enumerate() {
            self.validate_string_parameter(&format!("command_{i}"), cmd, false)?;
        }
        Ok(())
    }

    /// Validate Branch command - checks name and source
    #[inline]
    pub fn validate_branch_command(
        &self,
        name: Option<&str>,
        source: Option<&str>,
    ) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        if let Some(s) = source {
            self.validate_name_parameter("source", s)?;
        }
        Ok(())
    }

    /// Validate Session command - checks optional name
    #[inline]
    pub fn validate_session_command(&self, name: Option<&str>) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        Ok(())
    }

    /// Validate Tool command - checks name and args
    #[inline]
    pub fn validate_tool_command(
        &self,
        name: Option<&str>,
        args: &HashMap<String, String>,
    ) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        self.validate_tool_args(args)?;
        Ok(())
    }

    /// Validate Stats command - checks optional period
    #[inline]
    pub fn validate_stats_command(period: Option<&str>) -> Result<(), ValidationError> {
        if let Some(p) = period {
            Self::validate_enum_parameter("period", p, &["day", "week", "month", "year"])?;
        }
        Ok(())
    }

    /// Validate Theme command - checks name and properties
    #[inline]
    pub fn validate_theme_command(
        &self,
        name: Option<&str>,
        properties: &HashMap<String, String>,
    ) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        self.validate_theme_properties(properties)?;
        Ok(())
    }

    /// Validate Debug command - checks optional level
    #[inline]
    pub fn validate_debug_command(level: Option<&str>) -> Result<(), ValidationError> {
        if let Some(l) = level {
            Self::validate_enum_parameter("level", l, &["trace", "debug", "info", "warn", "error"])?;
        }
        Ok(())
    }

    /// Validate History command - checks optional filter
    #[inline]
    pub fn validate_history_command(&self, filter: Option<&str>) -> Result<(), ValidationError> {
        if let Some(f) = filter {
            self.validate_string_parameter("filter", f, false)?;
        }
        Ok(())
    }

    /// Validate Save command - checks name and location
    #[inline]
    pub fn validate_save_command(
        &self,
        name: Option<&str>,
        location: Option<&str>,
    ) -> Result<(), ValidationError> {
        if let Some(n) = name {
            self.validate_name_parameter("name", n)?;
        }
        if let Some(loc) = location {
            self.validate_path_parameter("location", loc)?;
        }
        Ok(())
    }

    /// Validate Load command - checks name and location
    #[inline]
    pub fn validate_load_command(
        &self,
        name: &str,
        location: Option<&str>,
    ) -> Result<(), ValidationError> {
        self.validate_name_parameter("name", name)?;
        if let Some(loc) = location {
            self.validate_path_parameter("location", loc)?;
        }
        Ok(())
    }

    /// Validate Import command - checks source path
    #[inline]
    pub fn validate_import_command(&self, source: &str) -> Result<(), ValidationError> {
        self.validate_path_parameter("source", source)?;
        Ok(())
    }

    /// Validate Settings command - checks key and value
    #[inline]
    pub fn validate_settings_command(
        &self,
        key: Option<&str>,
        value: Option<&str>,
    ) -> Result<(), ValidationError> {
        if let Some(k) = key {
            self.validate_config_key(k)?;
        }
        if let Some(v) = value {
            self.validate_config_value(v)?;
        }
        Ok(())
    }

    /// Validate Custom command - checks name and args
    #[inline]
    pub fn validate_custom_command(
        &self,
        name: &str,
        args: &HashMap<String, String>,
    ) -> Result<(), ValidationError> {
        self.validate_name_parameter("name", name)?;
        self.validate_tool_args(args)?;
        Ok(())
    }

    /// Validate Copy command - checks message_id and content
    #[inline]
    pub fn validate_copy_command(
        &self,
        message_id: Option<&str>,
        content: Option<&str>,
    ) -> Result<(), ValidationError> {
        if let Some(id) = message_id {
            self.validate_string_parameter("message_id", id, false)?;
        }
        if let Some(c) = content {
            self.validate_content_parameter("content", c)?;
        }
        Ok(())
    }

    /// Validate Retry command - checks command and attempts
    #[inline]
    pub fn validate_retry_command(
        &self,
        command: Option<&str>,
        attempts: Option<usize>,
    ) -> Result<(), ValidationError> {
        if let Some(cmd) = command {
            self.validate_string_parameter("command", cmd, false)?;
        }
        if let Some(n) = attempts {
            Self::validate_integer_parameter(
                "attempts",
                i64::try_from(n).unwrap_or(i64::MAX),
                Some(1),
                Some(10),
            )?;
        }
        Ok(())
    }

    /// Validate Undo command - checks count
    #[inline]
    pub fn validate_undo_command(count: Option<usize>) -> Result<(), ValidationError> {
        if let Some(n) = count {
            Self::validate_integer_parameter(
                "count",
                i64::try_from(n).unwrap_or(i64::MAX),
                Some(1),
                Some(100),
            )?;
        }
        Ok(())
    }

    /// Validate Chat command - checks message, context, and priority
    #[inline]
    pub fn validate_chat_command(
        &self,
        message: &str,
        context: Option<&str>,
        priority: Option<u8>,
    ) -> Result<(), ValidationError> {
        self.validate_string_parameter("message", message, false)?;
        if let Some(ctx) = context {
            self.validate_content_parameter("context", ctx)?;
        }
        if let Some(p) = priority {
            Self::validate_integer_parameter(
                "priority",
                i64::from(p),
                Some(0),
                Some(10),
            )?;
        }
        Ok(())
    }
}

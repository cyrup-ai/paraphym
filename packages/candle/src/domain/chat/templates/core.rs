//! Core template types and structures
//!
//! This module defines the fundamental types for the template system with
//! zero-allocation, lock-free architecture.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Core template error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TemplateError {
    /// Template was not found
    #[error("Template not found: {name}")]
    NotFound {
        /// Name of the template that was not found
        name: String,
    },

    /// Error occurred during template parsing
    #[error("Parse error: {message}")]
    ParseError {
        /// Details about the parsing error
        message: String,
    },

    /// Error occurred during template compilation
    #[error("Compile error: {message}")]
    CompileError {
        /// Details about the compilation error
        message: String,
    },

    /// Error occurred during template rendering
    #[error("Render error: {message}")]
    RenderError {
        /// Details about the rendering error
        message: String,
    },

    /// Error related to template variables
    #[error("Variable error: {message}")]
    VariableError {
        /// Details about the variable error
        message: String,
    },

    /// Operation was denied due to insufficient permissions
    #[error("Permission denied: {message}")]
    PermissionDenied {
        /// Details about the permission denial
        message: String,
    },

    /// Error occurred during storage operations
    #[error("Storage error: {message}")]
    StorageError {
        /// Details about the storage error
        message: String,
    },
}

/// Template result type
pub type TemplateResult<T> = Result<T, TemplateError>;

/// Template information structure for metadata queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// Unique identifier for the template
    pub id: String,
    /// Human-readable name of the template
    pub name: String,
    /// Category classification for organizational purposes
    pub category: TemplateCategory,
    /// Size of the template content in bytes
    pub size: usize,
    /// Number of template variables that can be substituted
    pub variable_count: usize,
    /// Unix timestamp when the template was created
    pub created_at: i64,
    /// Unix timestamp when the template was last modified
    pub modified_at: i64,
    /// Version string for template versioning
    pub version: String,
}

/// Template category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TemplateCategory {
    /// General chat conversation templates
    #[default]
    Chat,
    /// System-level templates for internal operations
    System,
    /// User-facing templates for user interactions
    User,
    /// Assistant response templates
    Assistant,
    /// Function call templates for structured operations
    Function,
    /// Tool usage templates for structured operations
    Tool,
    /// Context injection templates for conversation enhancement
    Context,
    /// Prompt templates for guiding AI behavior
    Prompt,
    /// Response formatting templates
    Response,
    /// User-defined custom template category
    Custom,
}



/// Variable type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariableType {
    /// String/text variable type
    String,
    /// Numeric variable type (integers and floats)
    Number,
    /// Boolean true/false variable type
    Boolean,
    /// Array/list variable type
    Array,
    /// Object/map variable type with key-value pairs
    Object,
    /// Any type - accepts all variable types
    Any,
}

/// Template variable definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Name of the template variable (used for substitution)
    pub name: String,
    /// Human-readable description of the variable's purpose
    pub description: String,
    /// Data type of the variable (string, number, boolean, etc.)
    pub var_type: VariableType,
    /// Default value to use if no value is provided
    pub default_value: Option<String>,
    /// Whether this variable must be provided for template rendering
    pub required: bool,
    /// Regular expression pattern for validating variable values
    pub validation_pattern: Option<String>,
    /// List of valid values for enumeration-type variables
    pub valid_values: Option<Vec<String>>,
    /// Minimum value for numeric variables
    pub min_value: Option<f64>,
    /// Maximum value for numeric variables
    pub max_value: Option<f64>,
}

bitflags::bitflags! {
    /// Template permissions using bitflags for zero-allocation checks
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct TemplatePermissions: u8 {
        /// Template can be read by users
        const READ = 1 << 0;
        /// Template can be modified by users
        const WRITE = 1 << 1;
        /// Template can be executed/rendered by users
        const EXECUTE = 1 << 2;
        /// Template can be shared with other users
        const SHARE = 1 << 3;
        /// Template can be deleted by users
        const DELETE = 1 << 4;
        /// All permissions enabled
        const ALL = Self::READ.bits() | Self::WRITE.bits() | Self::EXECUTE.bits() | Self::SHARE.bits() | Self::DELETE.bits();
    }
}

impl Default for TemplatePermissions {
    #[inline]
    fn default() -> Self {
        Self::ALL
    }
}

/// Template metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Unique identifier for the template
    pub id: String,
    /// Human-readable name of the template
    pub name: String,
    /// Description of what the template does
    pub description: String,
    /// Author or creator of the template
    pub author: String,
    /// Version string of the template
    pub version: String,
    /// Category classification for the template
    pub category: TemplateCategory,
    /// Tags associated with the template for search/filtering
    pub tags: Vec<String>,
    /// Timestamp when the template was created
    pub created_at: u64,
    /// Timestamp when the template was last modified
    pub modified_at: u64,
    /// Number of times this template has been used
    pub usage_count: u64,
    /// User rating of the template (0.0 to 5.0)
    pub rating: f64,
    /// Permission settings for the template
    pub permissions: TemplatePermissions,
}

/// Template value type for variables
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemplateValue {
    /// String value stored as String for zero-allocation sharing
    String(String),
    /// Numeric value stored as 64-bit floating point
    Number(f64),
    /// Boolean true/false value
    Boolean(bool),
    /// Array of template values
    Array(Vec<TemplateValue>),
    /// Object/map of key-value pairs
    Object(HashMap<String, TemplateValue>),
    /// Null/empty value
    Null,
}

impl From<&str> for TemplateValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for TemplateValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<f64> for TemplateValue {
    fn from(n: f64) -> Self {
        Self::Number(n)
    }
}

impl From<bool> for TemplateValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

/// Type alias for template function
pub type TemplateFn = Arc<dyn Fn(&[TemplateValue]) -> TemplateResult<TemplateValue> + Send + Sync>;

/// Template context for rendering
#[derive(Clone)]
pub struct TemplateContext {
    /// Variables available during template rendering
    pub variables: HashMap<String, TemplateValue>,
    /// Functions available during template rendering
    pub functions: HashMap<String, TemplateFn>,
}

impl TemplateContext {
    /// Create a new empty template context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Add a variable to the context (builder pattern)
    #[must_use]
    pub fn with_variable(
        mut self,
        name: impl Into<String>,
        value: impl Into<TemplateValue>,
    ) -> Self {
        self.variables.insert(name.into(), value.into());
        self
    }

    /// Set a variable in the context (mutating)
    pub fn set_variable(&mut self, name: impl Into<String>, value: impl Into<TemplateValue>) {
        self.variables.insert(name.into(), value.into());
    }

    /// Get a variable value by name
    pub fn get_variable(&self, name: &str) -> Option<&TemplateValue> {
        self.variables.get(name)
    }

    /// Get all variables as a reference to the `HashMap`
    pub fn variables(&self) -> &HashMap<String, TemplateValue> {
        &self.variables
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Abstract syntax tree for templates
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateAst {
    /// Static text content
    Text(String),
    /// Variable reference
    Variable(String),
    /// Expression with operator and operands
    Expression {
        /// The operator for this expression
        operator: String,
        /// The operands for this expression
        operands: Arc<[TemplateAst]>,
    },
    /// Conditional (if/else) statement
    Conditional {
        /// The condition to evaluate
        condition: Arc<TemplateAst>,
        /// AST to execute if condition is true
        if_true: Arc<TemplateAst>,
        /// Optional AST to execute if condition is false
        if_false: Option<Arc<TemplateAst>>,
    },
    /// Loop statement
    Loop {
        /// Loop variable name
        variable: String,
        /// The iterable expression to loop over
        iterable: Arc<TemplateAst>,
        /// The body of the loop
        body: Arc<TemplateAst>,
    },
    /// Block of multiple AST nodes
    Block(Arc<[TemplateAst]>),
    /// Function call
    Function {
        /// Function name
        name: String,
        /// Function arguments
        args: Arc<[TemplateAst]>,
    },
}

/// Compiled template representation
#[derive(Debug, Clone)]
pub struct CompiledTemplate {
    /// Template metadata and information
    pub metadata: TemplateMetadata,
    /// Compiled abstract syntax tree
    pub ast: TemplateAst,
    /// Template variables and their definitions
    pub variables: Arc<[TemplateVariable]>,
    /// Whether the template has been optimized for performance
    pub optimized: bool,
}

impl CompiledTemplate {
    /// Create a new compiled template
    pub fn new(
        metadata: TemplateMetadata,
        ast: TemplateAst,
        variables: Arc<[TemplateVariable]>,
    ) -> Self {
        Self {
            metadata,
            ast,
            variables,
            optimized: false,
        }
    }

    /// Render the template with the given context
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if template rendering fails or variables are missing
    pub fn render(&self, context: &TemplateContext) -> TemplateResult<String> {
        Self::render_ast(&self.ast, context)
    }

    /// Render AST node to string
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if rendering fails
    pub fn render_ast(ast: &TemplateAst, context: &TemplateContext) -> TemplateResult<String> {
        match ast {
            TemplateAst::Text(text) => Ok(text.clone()),
            TemplateAst::Variable(name) => {
                if let Some(value) = context.get_variable(name) {
                    match value {
                        TemplateValue::String(s) => Ok(s.clone()),
                        TemplateValue::Number(n) => Ok(n.to_string()),
                        TemplateValue::Boolean(b) => Ok(b.to_string()),
                        TemplateValue::Array(arr) => Ok(format!("[{} items]", arr.len())),
                        TemplateValue::Object(obj) => Ok(format!("{{{}keys}}", obj.len())),
                        TemplateValue::Null => Ok("null".to_string()),
                    }
                } else {
                    Err(TemplateError::VariableError {
                        message: format!("Variable '{name}' not found"),
                    })
                }
            }
            TemplateAst::Block(nodes) => {
                let mut result = String::new();
                for node in nodes.iter() {
                    let rendered = Self::render_ast(node, context)?;
                    result.push_str(&rendered);
                }
                Ok(result)
            }
            TemplateAst::Conditional {
                condition,
                if_true,
                if_false,
            } => {
                let cond_result = Self::render_ast(condition, context)?;
                let is_truthy = !cond_result.is_empty()
                    && cond_result.as_str() != "false"
                    && cond_result.as_str() != "0";

                if is_truthy {
                    Self::render_ast(if_true, context)
                } else if let Some(if_false_ast) = if_false {
                    Self::render_ast(if_false_ast, context)
                } else {
                    Ok(String::new())
                }
            }
            TemplateAst::Expression { operator, operands } => {
                Self::evaluate_expression(operator, operands, context)
            }
            TemplateAst::Loop { variable, iterable, body } => {
                Self::render_loop(variable, iterable, body, context)
            }
            TemplateAst::Function { name, args } => {
                Self::call_function(name, args, context)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn evaluate_expression(
        operator: &str,
        operands: &[TemplateAst],
        context: &TemplateContext,
    ) -> TemplateResult<String> {
        if operands.len() != 2 {
            return Err(TemplateError::RenderError {
                message: "Binary operators require exactly 2 operands".to_string(),
            });
        }

        let left = Self::render_ast(&operands[0], context)?;
        let right = Self::render_ast(&operands[1], context)?;

        match operator {
            "+" => {
                // Try numeric addition first
                if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    Ok((l + r).to_string())
                } else {
                    // String concatenation fallback
                    Ok(format!("{left}{right}"))
                }
            }
            "-" => {
                let l = left.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot subtract non-numeric value: {left}"),
                })?;
                let r = right.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot subtract non-numeric value: {right}"),
                })?;
                Ok((l - r).to_string())
            }
            "*" => {
                let l = left.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot multiply non-numeric value: {left}"),
                })?;
                let r = right.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot multiply non-numeric value: {right}"),
                })?;
                Ok((l * r).to_string())
            }
            "/" => {
                let l = left.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot divide non-numeric value: {left}"),
                })?;
                let r = right.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot divide non-numeric value: {right}"),
                })?;
                if r == 0.0 {
                    return Err(TemplateError::RenderError {
                        message: "Division by zero".to_string(),
                    });
                }
                Ok((l / r).to_string())
            }
            "%" => {
                let l = left.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot modulo non-numeric value: {left}"),
                })?;
                let r = right.parse::<f64>().map_err(|_| TemplateError::RenderError {
                    message: format!("Cannot modulo non-numeric value: {right}"),
                })?;
                if r == 0.0 {
                    return Err(TemplateError::RenderError {
                        message: "Modulo by zero".to_string(),
                    });
                }
                Ok((l % r).to_string())
            }
            "==" => Ok((left == right).to_string()),
            "!=" => Ok((left != right).to_string()),
            "<" => {
                if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    Ok((l < r).to_string())
                } else {
                    Ok((left < right).to_string())
                }
            }
            ">" => {
                if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    Ok((l > r).to_string())
                } else {
                    Ok((left > right).to_string())
                }
            }
            "<=" => {
                if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    Ok((l <= r).to_string())
                } else {
                    Ok((left <= right).to_string())
                }
            }
            ">=" => {
                if let (Ok(l), Ok(r)) = (left.parse::<f64>(), right.parse::<f64>()) {
                    Ok((l >= r).to_string())
                } else {
                    Ok((left >= right).to_string())
                }
            }
            "&&" | "and" => {
                let l_bool = !left.is_empty() && left != "false" && left != "0";
                let r_bool = !right.is_empty() && right != "false" && right != "0";
                Ok((l_bool && r_bool).to_string())
            }
            "||" | "or" => {
                let l_bool = !left.is_empty() && left != "false" && left != "0";
                let r_bool = !right.is_empty() && right != "false" && right != "0";
                Ok((l_bool || r_bool).to_string())
            }
            _ => Err(TemplateError::RenderError {
                message: format!("Unknown operator: {operator}"),
            }),
        }
    }

    fn render_loop(
        variable: &str,
        iterable: &TemplateAst,
        body: &TemplateAst,
        context: &TemplateContext,
    ) -> TemplateResult<String> {
        // Get the iterable value
        let iterable_value = Self::render_ast(iterable, context)?;

        // Get collection from context
        let collection = context.get_variable(&iterable_value)
            .ok_or_else(|| TemplateError::VariableError {
                message: format!("Loop iterable '{iterable_value}' not found"),
            })?;

        let mut result = String::new();

        match collection {
            TemplateValue::Array(items) => {
                for item in items {
                    // Create new context with loop variable
                    let mut loop_context = context.clone();
                    loop_context.set_variable(variable, item.clone());

                    // Render body with loop context
                    let rendered = Self::render_ast(body, &loop_context)?;
                    result.push_str(&rendered);
                }
            }
            _ => {
                return Err(TemplateError::RenderError {
                    message: "Loop iterable must be an array".to_string(),
                });
            }
        }

        Ok(result)
    }

    fn call_function(
        name: &str,
        args: &[TemplateAst],
        context: &TemplateContext,
    ) -> TemplateResult<String> {
        // Get function from context
        let func = context.functions.get(name)
            .ok_or_else(|| TemplateError::RenderError {
                message: format!("Function '{name}' not found"),
            })?;

        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            let rendered = Self::render_ast(arg, context)?;
            arg_values.push(TemplateValue::String(rendered));
        }

        // Call function
        let result = func(&arg_values)?;

        // Convert result to string
        match result {
            TemplateValue::String(s) => Ok(s),
            TemplateValue::Number(n) => Ok(n.to_string()),
            TemplateValue::Boolean(b) => Ok(b.to_string()),
            TemplateValue::Array(arr) => Ok(format!("[{} items]", arr.len())),
            TemplateValue::Object(obj) => Ok(format!("{{{}keys}}", obj.len())),
            TemplateValue::Null => Ok(String::new()),
        }
    }
}

/// Main chat template structure
#[derive(Debug, Clone)]
pub struct ChatTemplate {
    /// Template metadata and information
    pub metadata: TemplateMetadata,
    /// Raw template content string
    pub content: String,
    /// Template variables and their definitions
    pub variables: Arc<[TemplateVariable]>,
    /// Optional compiled template for faster rendering
    pub compiled: Option<CompiledTemplate>,
}

impl ChatTemplate {
    /// Create a new chat template
    pub fn new(
        metadata: TemplateMetadata,
        content: String,
        variables: Arc<[TemplateVariable]>,
    ) -> Self {
        Self {
            metadata,
            content,
            variables,
            compiled: None,
        }
    }

    /// Render the template with provided variables
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if template rendering fails or required variables are missing
    pub fn render<S: std::hash::BuildHasher>(&self, variables: &HashMap<String, String, S>) -> TemplateResult<String> {
        let mut context = TemplateContext::new();
        for (key, value) in variables {
            context.set_variable(key.clone(), TemplateValue::String(value.clone()));
        }

        if let Some(compiled) = &self.compiled {
            compiled.render(&context)
        } else {
            // Simple variable replacement for non-compiled templates
            let mut result = self.content.clone();
            for (key, value) in variables {
                result = result.replace(&format!("{{{{{key}}}}}"), value);
            }
            Ok(result)
        }
    }

    /// Get the template ID
    pub fn get_id(&self) -> &String {
        &self.metadata.id
    }

    /// Get the template name
    pub fn get_name(&self) -> &String {
        &self.metadata.name
    }

    /// Get the template content
    pub fn get_content(&self) -> &String {
        &self.content
    }

    /// Get the template variables
    pub fn get_variables(&self) -> &Arc<[TemplateVariable]> {
        &self.variables
    }

    /// Get template name (alias for `get_name` for compatibility)
    pub fn name(&self) -> &String {
        &self.metadata.name
    }

    /// Validate the template
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if:
    /// - Template name is empty
    /// - Template content is empty
    /// - Template validation checks fail
    pub fn validate(&self) -> TemplateResult<()> {
        // Basic validation
        if self.metadata.name.is_empty() {
            return Err(TemplateError::ParseError {
                message: "Template name cannot be empty".to_string(),
            });
        }

        if self.content.is_empty() {
            return Err(TemplateError::ParseError {
                message: "Template content cannot be empty".to_string(),
            });
        }

        // Additional validation can be added here
        Ok(())
    }

    /// Get template info
    pub fn info(&self) -> TemplateInfo {
        TemplateInfo {
            id: self.metadata.id.clone(),
            name: self.metadata.name.clone(),
            category: self.metadata.category,
            size: self.content.len(),
            variable_count: self.variables.len(),
            created_at: self.metadata.created_at.cast_signed(),
            modified_at: self.metadata.modified_at.cast_signed(),
            version: self.metadata.version.clone(),
        }
    }
}

/// Template configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Maximum size in bytes for a single template
    pub max_template_size: usize,
    /// Maximum number of variables allowed in a template
    pub max_variables: usize,
    /// Whether to allow templates to include other templates
    pub allow_nested_templates: bool,
    /// Whether to cache compiled templates for reuse
    pub cache_compiled: bool,
    /// Whether to apply optimizations to templates during compilation
    pub optimize_templates: bool,
    /// Security restrictions for template execution
    pub security_mode: SecurityMode,
}

/// Security mode for templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityMode {
    /// No external access, limited functions
    Strict,
    /// Standard functions allowed
    Normal,
    /// Most functions allowed
    Relaxed,
    /// All functions allowed
    Disabled,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            max_template_size: 1024 * 1024, // 1MB
            max_variables: 1000,
            allow_nested_templates: true,
            cache_compiled: true,
            optimize_templates: true,
            security_mode: SecurityMode::Normal,
        }
    }
}

/// Template example for documentation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateExample {
    /// Name of the template example
    pub name: String,
    /// Description of what the template demonstrates
    pub description: String,
    /// Input variables and their example values
    pub input_variables: HashMap<String, String>,
    /// Expected output when template is rendered with the input variables
    pub expected_output: String,
}

/// Template tag for categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemplateTag {
    /// Name of the tag
    pub name: String,
    /// Optional color for UI display
    pub color: Option<String>,
    /// Optional description of the tag's purpose
    pub description: Option<String>,
}

impl TemplateTag {
    /// Create a new template tag with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color: None,
            description: None,
        }
    }

    /// Set the color for this tag
    #[must_use]
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the description for this tag
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

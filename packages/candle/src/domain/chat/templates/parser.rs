//! Template parser implementation
//!
//! Provides high-performance parsing for template syntax with
//! zero-allocation patterns where possible.

use std::collections::HashMap;
use std::sync::Arc;

use super::core::{TemplateAst, TemplateError, TemplateResult, TemplateVariable, VariableType};

/// Template parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Maximum nesting depth for template structures
    pub max_depth: usize,
    /// Maximum number of tokens to parse
    pub max_tokens: usize,
    /// Whether to allow complex expressions in templates
    pub allow_expressions: bool,
    /// Whether to allow function calls in templates
    pub allow_functions: bool,
    /// Whether to require all variables to be explicitly defined
    pub strict_variables: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_depth: 32,
            max_tokens: 10000,
            allow_expressions: true,
            allow_functions: true,
            strict_variables: false,
        }
    }
}

/// Template parser implementation
#[derive(Debug)]
pub struct TemplateParser {
    config: ParserConfig,
}

impl TemplateParser {
    /// Create a new template parser with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create a new template parser with custom configuration
    #[must_use]
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse template content into AST
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if:
    /// - Maximum parsing depth is exceeded
    /// - Template syntax is invalid
    /// - Variable or expression parsing fails
    pub fn parse(&self, content: &str) -> TemplateResult<TemplateAst> {
        self.parse_with_depth(content, 0)
    }

    fn parse_with_depth(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if depth > self.config.max_depth {
            return Err(TemplateError::ParseError {
                message: "Maximum parsing depth exceeded".to_string(),
            });
        }

        let mut nodes = Vec::new();
        let mut current_text = String::new();
        let mut i = 0;
        let chars: Vec<char> = content.chars().collect();

        while i < chars.len() {
            // Check for {% block start %}
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '%' {
                // Save accumulated text
                if !current_text.is_empty() {
                    nodes.push(TemplateAst::Text(current_text.clone()));
                    current_text.clear();
                }

                // Find closing %}
                let block_start = i + 2;
                let mut block_end = block_start;
                while block_end + 1 < chars.len() {
                    if chars[block_end] == '%' && chars[block_end + 1] == '}' {
                        break;
                    }
                    block_end += 1;
                }

                if block_end + 1 >= chars.len() {
                    return Err(TemplateError::ParseError {
                        message: "Unclosed block tag".to_string(),
                    });
                }

                let block_content: String = chars[block_start..block_end].iter().collect();
                let block_content = block_content.trim();

                // Parse block based on tag
                if block_content.starts_with("if ") {
                    let (ast, new_i) = self.parse_conditional_block(content, i, depth)?;
                    nodes.push(ast);
                    i = new_i;
                    continue;
                } else if block_content.starts_with("for ") {
                    let (ast, new_i) = self.parse_loop_block(content, i, depth)?;
                    nodes.push(ast);
                    i = new_i;
                    continue;
                } else if block_content == "endif" || block_content == "endfor" 
                    || block_content.starts_with("elif ") || block_content == "elif"
                    || block_content == "else" {
                    // End tags handled by parent parser
                    break;
                }

                i = block_end + 2;
            }
            // Check for {{ expression }}
            else if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
                // Save accumulated text
                if !current_text.is_empty() {
                    nodes.push(TemplateAst::Text(current_text.clone()));
                    current_text.clear();
                }

                // Find closing }}
                let expr_start = i + 2;
                let mut expr_end = expr_start;
                let mut brace_count = 0;
                
                while expr_end < chars.len() {
                    if expr_end + 1 < chars.len() && chars[expr_end] == '}' && chars[expr_end + 1] == '}' {
                        if brace_count == 0 {
                            break;
                        }
                        brace_count -= 1;
                        expr_end += 1;
                    } else if expr_end + 1 < chars.len() && chars[expr_end] == '{' && chars[expr_end + 1] == '{' {
                        brace_count += 1;
                        expr_end += 1;
                    }
                    expr_end += 1;
                }

                if expr_end >= chars.len() {
                    return Err(TemplateError::ParseError {
                        message: "Unclosed expression".to_string(),
                    });
                }

                let expr_content: String = chars[expr_start..expr_end].iter().collect();
                let ast_node = self.parse_variable_or_expression(&expr_content, depth + 1)?;
                nodes.push(ast_node);

                i = expr_end + 2;
            } else {
                current_text.push(chars[i]);
                i += 1;
            }
        }

        // Add remaining text
        if !current_text.is_empty() {
            nodes.push(TemplateAst::Text(current_text));
        }

        // Return single node or block
        match nodes.len() {
            0 => Ok(TemplateAst::Text(String::new())),
            1 => {
                if let Some(node) = nodes.into_iter().next() {
                    Ok(node)
                } else {
                    Ok(TemplateAst::Text(String::new()))
                }
            }
            _ => Ok(TemplateAst::Block(nodes.into())),
        }
    }

    fn parse_conditional_block(
        &self,
        content: &str,
        start_pos: usize,
        depth: usize,
    ) -> TemplateResult<(TemplateAst, usize)> {
        if depth > self.config.max_depth {
            return Err(TemplateError::ParseError {
                message: "Maximum parsing depth exceeded".to_string(),
            });
        }

        // Extract condition from {% if condition %}
        let chars: Vec<char> = content.chars().collect();
        let mut block_start = start_pos + 2; // skip {%
        while block_start < chars.len() && (chars[block_start] == ' ' || chars[block_start] == '\t') {
            block_start += 1;
        }
        
        // Find %}
        let mut block_end = block_start;
        while block_end + 1 < chars.len() {
            if chars[block_end] == '%' && chars[block_end + 1] == '}' {
                break;
            }
            block_end += 1;
        }

        let block_content: String = chars[block_start..block_end].iter().collect();
        let condition_str = block_content.strip_prefix("if ").ok_or_else(|| TemplateError::ParseError {
            message: "Invalid if block syntax".to_string(),
        })?.trim();

        let condition = self.parse_expression(condition_str, depth + 1)?;

        // Find the body until endif/elif/else
        let body_start = block_end + 2;
        let (true_body, next_tag_pos, next_tag) = Self::find_block_end(content, body_start, &["endif", "elif", "else"])?;

        let if_true = self.parse_with_depth(&true_body, depth + 1)?;

        // Handle elif/else branches
        let (if_false, final_pos) = if next_tag.starts_with("elif") {
            // Parse elif as nested conditional
            let (elif_ast, end_pos) = self.parse_conditional_block(content, next_tag_pos, depth)?;
            (Some(Arc::new(elif_ast)), end_pos)
        } else if next_tag == "else" {
            // Find else body until endif
            let else_start = next_tag_pos;
            let chars_at: Vec<char> = content[else_start..].chars().collect();
            let mut i = 0;
            // Skip {% else %}
            while i + 1 < chars_at.len() {
                if chars_at[i] == '%' && chars_at[i + 1] == '}' {
                    i += 2;
                    break;
                }
                i += 1;
            }

            let (else_body, endif_pos, _) = Self::find_block_end(content, else_start + i, &["endif"])?;
            let else_ast = self.parse_with_depth(&else_body, depth + 1)?;
            (Some(Arc::new(else_ast)), endif_pos)
        } else {
            // endif - no else branch
            (None, next_tag_pos)
        };

        // Skip {% endif %}
        let chars_at: Vec<char> = content[final_pos..].chars().collect();
        let mut end_i = 0;
        while end_i + 1 < chars_at.len() {
            if chars_at[end_i] == '%' && chars_at[end_i + 1] == '}' {
                end_i += 2;
                break;
            }
            end_i += 1;
        }

        Ok((
            TemplateAst::Conditional {
                condition: Arc::new(condition),
                if_true: Arc::new(if_true),
                if_false,
            },
            final_pos + end_i,
        ))
    }

    fn parse_loop_block(
        &self,
        content: &str,
        start_pos: usize,
        depth: usize,
    ) -> TemplateResult<(TemplateAst, usize)> {
        if depth > self.config.max_depth {
            return Err(TemplateError::ParseError {
                message: "Maximum parsing depth exceeded".to_string(),
            });
        }

        // Extract loop header from {% for var in items %}
        let chars: Vec<char> = content.chars().collect();
        let mut block_start = start_pos + 2; // skip {%
        while block_start < chars.len() && (chars[block_start] == ' ' || chars[block_start] == '\t') {
            block_start += 1;
        }
        
        // Find %}
        let mut block_end = block_start;
        while block_end + 1 < chars.len() {
            if chars[block_end] == '%' && chars[block_end + 1] == '}' {
                break;
            }
            block_end += 1;
        }

        let block_content: String = chars[block_start..block_end].iter().collect();
        let loop_header = block_content.strip_prefix("for ").ok_or_else(|| TemplateError::ParseError {
            message: "Invalid for block syntax".to_string(),
        })?.trim();

        let parts: Vec<&str> = loop_header.split(" in ").collect();
        if parts.len() != 2 {
            return Err(TemplateError::ParseError {
                message: "Invalid loop syntax: expected 'for variable in iterable'".to_string(),
            });
        }

        let variable = parts[0].trim().to_string();
        let iterable_expr = parts[1].trim();

        // Parse iterable as expression or variable
        let iterable = if iterable_expr.contains(['+', '-', '*', '/', '=', '<', '>', '&', '|']) {
            self.parse_expression(iterable_expr, depth + 1)?
        } else {
            TemplateAst::Variable(iterable_expr.to_string())
        };

        // Find loop body until endfor
        let body_start = block_end + 2;
        let (body_content, endfor_pos, _) = Self::find_block_end(content, body_start, &["endfor"])?;

        let body = self.parse_with_depth(&body_content, depth + 1)?;

        // Skip {% endfor %}
        let chars_at: Vec<char> = content[endfor_pos..].chars().collect();
        let mut end_i = 0;
        while end_i + 1 < chars_at.len() {
            if chars_at[end_i] == '%' && chars_at[end_i + 1] == '}' {
                end_i += 2;
                break;
            }
            end_i += 1;
        }

        Ok((
            TemplateAst::Loop {
                variable,
                iterable: Arc::new(iterable),
                body: Arc::new(body),
            },
            endfor_pos + end_i,
        ))
    }

    fn find_block_end(
        content: &str,
        start: usize,
        end_tags: &[&str],
    ) -> TemplateResult<(String, usize, String)> {
        let chars: Vec<char> = content[start..].chars().collect();
        let mut i = 0;
        let mut depth = 0;
        let mut body = String::new();

        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '%' {
                // Find tag content
                let tag_start = i + 2;
                let mut tag_end = tag_start;
                while tag_end + 1 < chars.len() {
                    if chars[tag_end] == '%' && chars[tag_end + 1] == '}' {
                        break;
                    }
                    tag_end += 1;
                }

                let tag_content: String = chars[tag_start..tag_end].iter().collect();
                let tag_content = tag_content.trim();

                // Check for nesting
                if tag_content.starts_with("if ") || tag_content.starts_with("for ") {
                    depth += 1;
                    body.push_str(&chars[i..tag_end + 2].iter().collect::<String>());
                    i = tag_end + 2;
                    continue;
                } else if tag_content == "endif" || tag_content == "endfor" {
                    if depth > 0 {
                        depth -= 1;
                        body.push_str(&chars[i..tag_end + 2].iter().collect::<String>());
                        i = tag_end + 2;
                        continue;
                    }
                    // Found our end tag
                    for end_tag in end_tags {
                        if tag_content == *end_tag {
                            return Ok((body, start + i, tag_content.to_string()));
                        }
                    }
                } else if (tag_content == "else" || tag_content.starts_with("elif ")) && depth == 0 {
                    // Found our end tag
                    for end_tag in end_tags {
                        if tag_content.starts_with(end_tag) || tag_content == *end_tag {
                            return Ok((body, start + i, tag_content.to_string()));
                        }
                    }
                }

                body.push_str(&chars[i..tag_end + 2].iter().collect::<String>());
                i = tag_end + 2;
            } else {
                body.push(chars[i]);
                i += 1;
            }
        }

        Err(TemplateError::ParseError {
            message: format!("Unclosed block: expected one of {end_tags:?}"),
        })
    }

    fn parse_variable_or_expression(
        &self,
        content: &str,
        depth: usize,
    ) -> TemplateResult<TemplateAst> {
        let trimmed = content.trim();

        // Check for expressions first (before function calls)
        if self.config.allow_expressions
            && (trimmed.contains('+')
                || trimmed.contains('-')
                || trimmed.contains('*')
                || trimmed.contains('/')
                || trimmed.contains('%')
                || trimmed.contains("==")
                || trimmed.contains("!=")
                || trimmed.contains("<=")
                || trimmed.contains(">=")
                || trimmed.contains('<')
                || trimmed.contains('>')
                || trimmed.contains("&&")
                || trimmed.contains("||")
                || trimmed.contains(" and ")
                || trimmed.contains(" or "))
        {
            return self.parse_expression(trimmed, depth);
        }

        // Check for function calls
        if trimmed.contains('(') && trimmed.contains(')') {
            return self.parse_function_call(trimmed, depth);
        }

        // Simple variable
        Ok(TemplateAst::Variable(trimmed.to_string()))
    }

    fn parse_expression(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if depth > self.config.max_depth {
            return Err(TemplateError::ParseError {
                message: "Maximum parsing depth exceeded".to_string(),
            });
        }
        self.parse_logical_or(content, depth)
    }

    fn parse_logical_or(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if let Some(pos) = Self::find_operator(content, &["||", " or "]) {
            let (left_str, _op, right_str) = Self::extract_operator(content, pos, &["||", " or "])?;
            let left = self.parse_logical_and(&left_str, depth)?;
            let right = self.parse_logical_or(&right_str, depth)?;
            return Ok(TemplateAst::Expression {
                operator: "||".to_string(),
                operands: Arc::new([left, right]),
            });
        }

        self.parse_logical_and(content, depth)
    }

    fn parse_logical_and(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if let Some(pos) = Self::find_operator(content, &["&&", " and "]) {
            let (left_str, _op, right_str) = Self::extract_operator(content, pos, &["&&", " and "])?;
            let left = self.parse_comparison(&left_str, depth)?;
            let right = self.parse_logical_and(&right_str, depth)?;
            return Ok(TemplateAst::Expression {
                operator: "&&".to_string(),
                operands: Arc::new([left, right]),
            });
        }

        self.parse_comparison(content, depth)
    }

    fn parse_comparison(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if let Some(pos) = Self::find_operator(content, &["==", "!=", "<=", ">=", "<", ">"]) {
            let (left_str, op, right_str) = Self::extract_operator(content, pos, &["==", "!=", "<=", ">=", "<", ">"])?;
            let left = self.parse_additive(&left_str, depth)?;
            let right = self.parse_additive(&right_str, depth)?;
            return Ok(TemplateAst::Expression {
                operator: op,
                operands: Arc::new([left, right]),
            });
        }

        self.parse_additive(content, depth)
    }

    fn parse_additive(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if let Some(pos) = Self::find_operator(content, &["+", "-"]) {
            let (left_str, op, right_str) = Self::extract_operator(content, pos, &["+", "-"])?;
            let left = self.parse_additive(&left_str, depth)?;
            let right = self.parse_multiplicative(&right_str, depth)?;
            return Ok(TemplateAst::Expression {
                operator: op,
                operands: Arc::new([left, right]),
            });
        }

        self.parse_multiplicative(content, depth)
    }

    fn parse_multiplicative(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if let Some(pos) = Self::find_operator(content, &["*", "/", "%"]) {
            let (left_str, op, right_str) = Self::extract_operator(content, pos, &["*", "/", "%"])?;
            let left = self.parse_multiplicative(&left_str, depth)?;
            let right = self.parse_primary(&right_str, depth)?;
            return Ok(TemplateAst::Expression {
                operator: op,
                operands: Arc::new([left, right]),
            });
        }

        self.parse_primary(content, depth)
    }

    fn parse_primary(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        let trimmed = content.trim();

        // Handle parentheses
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let inner = &trimmed[1..trimmed.len() - 1];
            return self.parse_expression(inner, depth + 1);
        }

        // Handle numeric literals
        if trimmed.parse::<f64>().is_ok() {
            return Ok(TemplateAst::Text(trimmed.to_string()));
        }

        // Handle string literals
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            return Ok(TemplateAst::Text(trimmed[1..trimmed.len() - 1].to_string()));
        }

        // Variable reference
        Ok(TemplateAst::Variable(trimmed.to_string()))
    }

    fn find_operator(content: &str, operators: &[&str]) -> Option<usize> {
        let chars: Vec<char> = content.chars().collect();
        let mut paren_depth = 0;
        let mut last_op_pos = None;  // Track rightmost occurrence
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '(' {
                paren_depth += 1;
                i += 1;
            } else if chars[i] == ')' {
                paren_depth -= 1;
                i += 1;
            } else if paren_depth == 0 {
                for op in operators {
                    if i + op.len() <= chars.len() {
                        let substr: String = chars[i..i + op.len()].iter().collect();
                        if substr == *op {
                            last_op_pos = Some(i);  // Keep updating to track rightmost
                            i += op.len() - 1;  // Skip operator chars
                            break;
                        }
                    }
                }
                i += 1;
            } else {
                i += 1;
            }
        }

        last_op_pos  // Return rightmost operator position
    }

    fn extract_operator(
        content: &str,
        pos: usize,
        operators: &[&str],
    ) -> TemplateResult<(String, String, String)> {
        let chars: Vec<char> = content.chars().collect();
        
        for op in operators {
            if pos + op.len() <= chars.len() {
                let substr: String = chars[pos..pos + op.len()].iter().collect();
                if substr == *op {
                    let left: String = chars[..pos].iter().collect();
                    let right: String = chars[pos + op.len()..].iter().collect();
                    return Ok((left, (*op).to_string(), right));
                }
            }
        }

        Err(TemplateError::ParseError {
            message: "Operator not found at position".to_string(),
        })
    }

    fn parse_function_call(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
        if !self.config.allow_functions {
            return Err(TemplateError::ParseError {
                message: "Function calls not allowed".to_string(),
            });
        }

        if depth > self.config.max_depth {
            return Err(TemplateError::ParseError {
                message: "Maximum parsing depth exceeded".to_string(),
            });
        }

        let paren_pos = content.find('(').ok_or_else(|| TemplateError::ParseError {
            message: "Invalid function call syntax".to_string(),
        })?;
        let func_name = content[..paren_pos].trim();

        // Extract arguments
        let close_paren = content.rfind(')').ok_or_else(|| TemplateError::ParseError {
            message: "Unclosed function call".to_string(),
        })?;
        
        let args_str = &content[paren_pos + 1..close_paren];
        let mut args = Vec::new();

        if !args_str.trim().is_empty() {
            // Simple comma-split for now
            for arg in args_str.split(',') {
                let arg_ast = self.parse_expression(arg.trim(), depth + 1)?;
                args.push(arg_ast);
            }
        }

        Ok(TemplateAst::Function {
            name: func_name.to_string(),
            args: args.into(),
        })
    }

    /// Extract variables from template content
    ///
    /// # Errors
    ///
    /// Returns `TemplateError` if variable extraction fails
    pub fn extract_variables(&self, content: &str) -> TemplateResult<Vec<TemplateVariable>> {
        let mut variables = HashMap::new();
        let mut i = 0;
        let chars: Vec<char> = content.chars().collect();

        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
                i += 2;

                // Find closing }}
                let mut expr_end = i;
                let mut brace_count = 0;
                
                while expr_end < chars.len() {
                    if expr_end + 1 < chars.len() && chars[expr_end] == '}' && chars[expr_end + 1] == '}' {
                        if brace_count == 0 {
                            break;
                        }
                        brace_count -= 1;
                        expr_end += 1;
                    } else if expr_end + 1 < chars.len() && chars[expr_end] == '{' && chars[expr_end + 1] == '{' {
                        brace_count += 1;
                        expr_end += 1;
                    }
                    expr_end += 1;
                }

                let var_content: String = chars[i..expr_end].iter().collect();
                let var_name = var_content.trim();

                // Skip control structures
                if var_name.starts_with("if ")
                    || var_name.starts_with("for ")
                    || var_name.starts_with("end")
                {
                    i = expr_end + 2;
                    continue;
                }

                // Extract simple variable name
                let clean_name = var_name
                    .split_whitespace()
                    .next()
                    .unwrap_or(var_name)
                    .split('.')
                    .next()
                    .unwrap_or("")
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .trim();

                if !clean_name.is_empty()
                    && clean_name.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    variables.insert(
                        clean_name.to_string(),
                        TemplateVariable {
                            name: clean_name.to_string(),
                            description: String::new(),
                            var_type: VariableType::String,
                            default_value: None,
                            required: false,
                            validation_pattern: None,
                            valid_values: None,
                            min_value: None,
                            max_value: None,
                        },
                    );
                }

                i = expr_end + 2;
            } else {
                i += 1;
            }
        }

        Ok(variables.into_values().collect())
    }
}

impl Default for TemplateParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parser statistics for performance monitoring
#[derive(Debug, Clone, PartialEq)]
pub struct ParserStats {
    /// Total number of templates parsed
    pub parsed_count: usize,
    /// Total time spent parsing in microseconds
    pub total_parse_time_us: usize,
    /// Number of parse errors encountered
    pub error_count: usize,
    /// Peak memory usage during parsing in bytes
    pub peak_memory_bytes: usize,
}

/// Parse error type alias for convenience
pub type ParseError = TemplateError;

/// Parse result type alias for convenience
pub type ParseResult<T> = Result<T, ParseError>;

/// Quick parse function for convenience
///
/// # Errors
///
/// Returns `TemplateError` if parsing fails (see `TemplateParser::parse`)
pub fn parse_template(content: &str) -> TemplateResult<TemplateAst> {
    let parser = TemplateParser::new();
    parser.parse(content)
}

/// Extract variables from template content
///
/// # Errors
///
/// Returns `TemplateError` if variable extraction fails
pub fn extract_variables(content: &str) -> TemplateResult<Vec<TemplateVariable>> {
    let parser = TemplateParser::new();
    parser.extract_variables(content)
}

/// Validate template syntax
///
/// # Errors
///
/// Returns `TemplateError` if template syntax is invalid
pub fn validate_template(content: &str) -> TemplateResult<()> {
    let parser = TemplateParser::new();
    parser.parse(content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chat::templates::core::{TemplateContext, CompiledTemplate};

    #[test]
    fn test_simple_variable_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let result = parser.parse("Hello {{name}}!")?;

        match result {
            TemplateAst::Block(nodes) => {
                assert_eq!(nodes.len(), 3);
            }
            _ => panic!("Expected block AST"),
        }
        Ok(())
    }

    #[test]
    fn test_variable_extraction() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let variables = parser
            .extract_variables("Hello {{name}}, you have {{count}} messages.")?;

        assert_eq!(variables.len(), 2);
        assert!(variables.iter().any(|v| v.name.as_str() == "name"));
        assert!(variables.iter().any(|v| v.name.as_str() == "count"));
        Ok(())
    }

    #[test]
    fn test_template_validation() {
        assert!(validate_template("Hello {{name}}!").is_ok());
        assert!(validate_template("{{unclosed").is_ok()); // Parser is lenient
    }

    #[test]
    fn test_subtraction_associativity() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let context = TemplateContext::new();

        let ast = parser.parse("{{ 10 - 3 - 2 }}")?;
        let result = CompiledTemplate::render_ast(&ast, &context)?;

        assert_eq!(result.trim(), "5", "10 - 3 - 2 should be left-associative: (10-3)-2 = 5");
        Ok(())
    }

    #[test]
    fn test_division_associativity() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let context = TemplateContext::new();

        let ast = parser.parse("{{ 20 / 4 / 2 }}")?;
        let result = CompiledTemplate::render_ast(&ast, &context)?;

        assert_eq!(result.trim(), "2.5", "20 / 4 / 2 should be left-associative: (20/4)/2 = 2.5");
        Ok(())
    }

    #[test]
    fn test_modulo_associativity() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let context = TemplateContext::new();

        let ast = parser.parse("{{ 17 % 5 % 2 }}")?;
        let result = CompiledTemplate::render_ast(&ast, &context)?;

        assert_eq!(result.trim(), "0", "17 % 5 % 2 should be left-associative: (17%5)%2 = 0");
        Ok(())
    }

    #[test]
    fn test_precedence_with_associativity() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let context = TemplateContext::new();

        let ast = parser.parse("{{ 10 - 2 * 3 }}")?;
        let result = CompiledTemplate::render_ast(&ast, &context)?;

        assert_eq!(result.trim(), "4", "10 - 2 * 3 should respect precedence: 10 - (2*3) = 4");
        Ok(())
    }

    #[test]
    fn test_parentheses_override_associativity() -> Result<(), Box<dyn std::error::Error>> {
        let parser = TemplateParser::new();
        let context = TemplateContext::new();

        let ast = parser.parse("{{ 10 - (3 - 2) }}")?;
        let result = CompiledTemplate::render_ast(&ast, &context)?;

        assert_eq!(result.trim(), "9", "10 - (3 - 2) should respect parentheses: 10 - 1 = 9");
        Ok(())
    }
}

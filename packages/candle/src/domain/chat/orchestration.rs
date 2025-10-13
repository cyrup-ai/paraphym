//! Chat loop orchestration utilities for multi-stage tool calling
//!
//! This module provides helper functions for the 4-stage tool calling process:
//! - Stage 1: Tool Selection
//! - Stage 2: Function Calling  
//! - Stage 3: Tool Execution (handled by `SweetMcpRouter`)
//! - Stage 4: Result Interpretation

use std::collections::HashMap;
use serde_json::json;
use sweet_mcp_type::ToolInfo;
use anyhow::{Result, Context};

use super::templates;
use super::types::responses::{ToolSelectionResponse, OpenAIFunctionCallResponse, FinalResponse};

/// Format tools as simple text list for Stage 1 (Tool Selection)
#[must_use]
pub fn format_tools_for_selection(tools: &[ToolInfo]) -> String {
    tools
        .iter()
        .map(|tool| {
            let desc = tool.description.as_deref().unwrap_or("No description");
            format!("- {}: {}", tool.name, desc)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format tools in `OpenAI` tools format for Stage 2 (Function Calling)
///
/// # Errors
///
/// Returns error if serialization to JSON fails
pub fn format_tools_openai(tools: &[ToolInfo]) -> Result<String> {    let openai_tools: Vec<serde_json::Value> = tools
        .iter()
        .map(|tool| {
            json!({
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description.as_ref().unwrap_or(&String::new()),
                    "parameters": tool.input_schema
                }
            })
        })
        .collect();
    
    serde_json::to_string_pretty(&openai_tools)
        .context("Failed to serialize tools to OpenAI format")
}

/// Format tool execution results for Stage 4 (Result Interpretation)
#[must_use]
pub fn format_tool_results(
    tool_calls: &[super::types::responses::ToolCall],
    results: &[(String, Result<serde_json::Value, String>)]
) -> String {
    results
        .iter()
        .enumerate()
        .map(|(idx, (call_id, result))| {
            let tool_name = tool_calls
                .get(idx)
                .map_or("unknown", |tc| tc.function.name.as_str());            
            match result {
                Ok(value) => format!(
                    "Tool: {}\nCall ID: {}\nStatus: Success\nResult: {}",
                    tool_name,
                    call_id,
                    serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
                ),
                Err(error) => format!(
                    "Tool: {tool_name}\nCall ID: {call_id}\nStatus: Error\nError: {error}"
                ),
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Render Stage 1 prompt (Tool Selection)
///
/// # Errors
///
/// Returns error if template rendering fails
pub fn render_stage1_prompt(user_input: &str, available_tools: &[ToolInfo]) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("user_input".to_string(), user_input.to_string());
    variables.insert("available_tools".to_string(), format_tools_for_selection(available_tools));

    templates::render_template("tool_selection", &variables)
        .context("Failed to render tool_selection template")
}

/// Render Stage 2 prompt (Function Calling)
///
/// # Errors
///
/// Returns error if template rendering or tool formatting fails
pub fn render_stage2_prompt(user_input: &str, selected_tools: &[ToolInfo]) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("user_input".to_string(), user_input.to_string());
    variables.insert("tools_json".to_string(), format_tools_openai(selected_tools)?);

    templates::render_template("function_calling", &variables)
        .context("Failed to render function_calling template")
}

/// Render Stage 4 prompt (Result Interpretation)
///
/// # Errors
///
/// Returns error if template rendering or JSON serialization fails
pub fn render_stage4_prompt(
    user_message: &str,
    tool_calls: &[super::types::responses::ToolCall],
    results: &[(String, Result<serde_json::Value, String>)]
) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("user_message".to_string(), user_message.to_string());
    
    // Serialize tool calls to JSON
    let tool_calls_json = serde_json::to_string_pretty(tool_calls)
        .context("Failed to serialize tool calls")?;
    variables.insert("tool_calls_json".to_string(), tool_calls_json);
    
    // Format tool results
    variables.insert("tool_results".to_string(), format_tool_results(tool_calls, results));

    templates::render_template("result_interpretation", &variables)
        .context("Failed to render result_interpretation template")
}

/// Parse Stage 1 response (Tool Selection)
///
/// # Errors
///
/// Returns error if JSON parsing fails
pub fn parse_tool_selection_response(json_str: &str) -> Result<ToolSelectionResponse> {
    serde_json::from_str(json_str)
        .context("Failed to parse tool selection response")
}

/// Parse Stage 2 response (Function Calling)
///
/// # Errors
///
/// Returns error if JSON parsing fails
pub fn parse_function_call_response(json_str: &str) -> Result<OpenAIFunctionCallResponse> {
    serde_json::from_str(json_str)
        .context("Failed to parse function call response")
}

/// Parse Stage 4 response (Final Response)
///
/// # Errors
///
/// Returns error if JSON parsing fails
pub fn parse_final_response(json_str: &str) -> Result<FinalResponse> {
    serde_json::from_str(json_str)
        .context("Failed to parse final response")
}

#[cfg(test)]
#[must_use]
pub fn get_selected_tool_schemas(
    selected_names: &[String],
    available_tools: &[ToolInfo]
) -> Vec<ToolInfo> {
    available_tools
        .iter()
        .filter(|tool| selected_names.contains(&tool.name))
        .cloned()
        .collect()
}

/// Helper to collect `AsyncStream` into String
#[must_use]
pub fn collect_stream_to_string(
    stream: &ystream::AsyncStream<crate::domain::context::chunk::CandleStringChunk>
) -> String {
    let mut result = String::new();
    while let Some(chunk) = stream.try_next() {
        result.push_str(&chunk.0);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_tools_for_selection() -> Result<(), Box<dyn std::error::Error>> {
        // Convert serde_json to simd_json by serializing and deserializing
        let schema_json = serde_json::to_string(&json!({"type": "object"}))?;
        let mut schema_bytes = schema_json.as_bytes().to_vec();
        let input_schema = simd_json::to_owned_value(&mut schema_bytes)?;

        let tools = vec![
            ToolInfo {
                name: "calculator".to_string(),
                description: Some("Perform calculations".to_string()),
                input_schema: input_schema.clone(),
            },
            ToolInfo {
                name: "search".to_string(),
                description: Some("Search the web".to_string()),
                input_schema,
            },
        ];

        let formatted = format_tools_for_selection(&tools);
        assert!(formatted.contains("calculator: Perform calculations"));
        assert!(formatted.contains("search: Search the web"));
        Ok(())
    }
    
    #[test]
    fn test_get_selected_tool_schemas() -> Result<(), Box<dyn std::error::Error>> {
        // Convert serde_json to simd_json
        let schema_json = serde_json::to_string(&json!({}))?;
        let mut schema_bytes = schema_json.as_bytes().to_vec();
        let input_schema = simd_json::to_owned_value(&mut schema_bytes)?;

        let tools = vec![
            ToolInfo {
                name: "tool1".to_string(),
                description: Some("desc1".to_string()),
                input_schema: input_schema.clone(),
            },
            ToolInfo {
                name: "tool2".to_string(),
                description: Some("desc2".to_string()),
                input_schema,
            },
        ];

        let selected = get_selected_tool_schemas(&vec!["tool1".to_string()], &tools);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].name, "tool1");
        Ok(())
    }
}
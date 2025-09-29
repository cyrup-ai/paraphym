// Test file to verify ZeroOneOrMany tools syntax works correctly
use cyrup_sugars::ZeroOneOrMany;
use sweet_mcp_type::{ToolInfo, JsonValue};

fn test_tools_ergonomic_syntax() {
    // Create sample ToolInfo objects
    let tool1 = ToolInfo {
        name: "execute_python".to_string(),
        description: Some("Execute Python code".to_string()),
        input_schema: JsonValue::Object(std::collections::HashMap::new()),
    };

    let tool2 = ToolInfo {
        name: "search_web".to_string(),
        description: Some("Search the web".to_string()),
        input_schema: JsonValue::Object(std::collections::HashMap::new()),
    };

    // Test ZeroOneOrMany ergonomic conversion
    let _single_tool: ZeroOneOrMany<ToolInfo> = tool1.clone().into();
    let _multiple_tools: ZeroOneOrMany<ToolInfo> = vec![tool1, tool2].into();

    println!("ZeroOneOrMany ergonomic syntax works!");
}

fn main() {
    test_tools_ergonomic_syntax();
}
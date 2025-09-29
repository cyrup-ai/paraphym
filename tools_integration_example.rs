/// Example showing the expected tools integration usage
/// This demonstrates how the ergonomic ZeroOneOrMany syntax should work

use cyrup_sugars::ZeroOneOrMany;
use sweet_mcp_type::{ToolInfo, JsonValue};
use std::collections::HashMap;

// Mock builder for demonstration
struct CandleFluentAi;
struct MockProvider;
struct MockBuilder {
    tools: ZeroOneOrMany<ToolInfo>,
}

impl CandleFluentAi {
    fn agent_role(name: &str) -> MockBuilder {
        MockBuilder {
            tools: ZeroOneOrMany::none(),
        }
    }
}

impl MockBuilder {
    fn completion_provider(self, _provider: MockProvider) -> Self {
        self
    }

    // The key method - should accept ergonomic ZeroOneOrMany syntax
    fn tools<T>(mut self, tools: T) -> Self
    where
        T: Into<ZeroOneOrMany<ToolInfo>>
    {
        self.tools = tools.into();
        self
    }
}

fn main() {
    // Create sample ToolInfo objects
    let python_tool = ToolInfo {
        name: "execute_python".to_string(),
        description: Some("Execute Python code securely".to_string()),
        input_schema: JsonValue::Object(HashMap::from([
            ("type".to_string(), JsonValue::String("object".to_string())),
            ("properties".to_string(), JsonValue::Object(HashMap::from([
                ("code".to_string(), JsonValue::Object(HashMap::from([
                    ("type".to_string(), JsonValue::String("string".to_string())),
                ])))
            ])))
        ])),
    };

    let search_tool = ToolInfo {
        name: "search_web".to_string(),
        description: Some("Search the web for information".to_string()),
        input_schema: JsonValue::Object(HashMap::from([
            ("type".to_string(), JsonValue::String("object".to_string())),
            ("properties".to_string(), JsonValue::Object(HashMap::from([
                ("query".to_string(), JsonValue::Object(HashMap::from([
                    ("type".to_string(), JsonValue::String("string".to_string())),
                ])))
            ])))
        ])),
    };

    // Test ergonomic syntax - single tool
    let _builder1 = CandleFluentAi::agent_role("assistant")
        .completion_provider(MockProvider)
        .tools(python_tool.clone()); // Single ToolInfo -> ZeroOneOrMany

    // Test ergonomic syntax - multiple tools
    let _builder2 = CandleFluentAi::agent_role("assistant")
        .completion_provider(MockProvider)
        .tools(vec![python_tool.clone(), search_tool.clone()]); // Vec<ToolInfo> -> ZeroOneOrMany

    // Test ergonomic syntax - direct ZeroOneOrMany
    let tools_collection = ZeroOneOrMany::many(vec![python_tool, search_tool]);
    let _builder3 = CandleFluentAi::agent_role("assistant")
        .completion_provider(MockProvider)
        .tools(tools_collection); // ZeroOneOrMany<ToolInfo> -> ZeroOneOrMany

    println!("All ergonomic syntax patterns work correctly!");
    println!("✅ Single ToolInfo conversion");
    println!("✅ Vec<ToolInfo> conversion");
    println!("✅ Direct ZeroOneOrMany usage");
}
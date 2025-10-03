use sweetmcp_plugin_builder::prelude::*;
use extism_pdk::*;
use serde_json::Value;

struct TestTool;

impl McpTool for TestTool {
    const NAME: &'static str = "test";

    fn description(builder: DescriptionBuilder) -> DescriptionBuilder {
        builder
            .does("Test tool functionality")
            .when("Running tests")
            .perfect_for("testing")
    }

    fn schema(builder: SchemaBuilder) -> Value {
        builder.required_string("input", "Test input").build()
    }

    fn execute(_args: Value) -> Result<sweetmcp_plugin_builder::CallToolResult, Error> {
        Ok(ContentBuilder::text("Test result"))
    }
}

#[test]
fn test_fluent_plugin_builder() {
    let plugin = mcp_plugin("test-plugin")
        .description("A test plugin")
        .tool::<TestTool>()
        .serve();

    let tools = plugin.describe().unwrap();
    assert_eq!(tools.tools.len(), 1);
    assert_eq!(tools.tools[0].name, "test");
}

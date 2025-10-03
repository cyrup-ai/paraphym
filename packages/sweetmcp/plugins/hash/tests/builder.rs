use sweet_mcp_hash::{schema, builder::*};
use serde_json::json;

#[test]
fn test_schema_macro() {
    let schema = schema! {
        properties: {
            data: {
                type: "string",
                description: "Input data"
            },
            algorithm: {
                type: "string", 
                description: "Hash algorithm",
                enum: ["sha256", "md5"]
            }
        },
        required: [data, algorithm]
    };

    assert!(schema.is_object());
    assert!(schema["properties"]["data"]["type"] == "string");
}

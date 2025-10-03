use sweetmcp_json_client::JsonClient;
use sweet_mcp_type::{Request, RequestId, JsonValue};
use std::collections::HashMap;

#[test]
fn test_client_creation() {
    let client = JsonClient::new("https://localhost:8443").unwrap();
    assert_eq!(client.protocol_name(), "JSON-RPC 2.0");
    assert_eq!(client.server_url(), "https://localhost:8443");
    assert!(client.is_connected());
}

#[test]
fn test_invalid_url() {
    let result = JsonClient::new("not-a-url");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_serialization() {
    let client = JsonClient::new("https://localhost:8443").unwrap();
    
    let request = Request {
        id: RequestId::Str("test-123".to_string()),
        method: "tools/call".to_string(),
        params: JsonValue::from([("name", "time"), ("arguments", "{}")]
            .iter()
            .map(|(k, v)| (k.to_string(), JsonValue::from(*v)))
            .collect::<HashMap<String, JsonValue>>()),
        meta: None,
    };

    let serialized = client.serialize_request(&request).unwrap();
    assert!(!serialized.is_empty());
}

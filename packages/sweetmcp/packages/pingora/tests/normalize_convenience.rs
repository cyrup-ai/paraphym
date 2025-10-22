use sweetmcp::normalize::{
    normalize_to_jsonrpc,
    denormalize_from_jsonrpc,
    quick_detect_protocol,
    test_context,
    Proto,
};

#[test]
fn test_normalize_and_denormalize_jsonrpc() {
    // Simple JSON-RPC request body
    let body = br#"{"jsonrpc":"2.0","method":"echo","id":1,"params":{"msg":"hi"}}"#;

    // Normalize with no headers
    let (ctx, jsonrpc) = normalize_to_jsonrpc("user", body, None)
        .expect("normalize_to_jsonrpc should succeed for JSON-RPC body");

    // Ensure protocol detection recognized JSON-RPC
    assert_eq!(quick_detect_protocol(body, None).unwrap(), Proto::JsonRpc);

    // Denormalize back using the same context
    let bytes = denormalize_from_jsonrpc(&ctx, &jsonrpc)
        .expect("denormalize_from_jsonrpc should succeed for JSON-RPC ctx");

    // Result should be valid JSON
    let v: serde_json::Value = serde_json::from_slice(&bytes).expect("valid JSON");
    assert_eq!(v.get("jsonrpc").and_then(|s| s.as_str()), Some("2.0"));
}

#[test]
fn test_test_context_helper() {
    // Ensure helper constructs a protocol context for GraphQL
    let ctx = test_context(Proto::GraphQL);
    // Sanity: protocol recorded as GraphQL
    assert_eq!(ctx.protocol, Proto::GraphQL);
}

# Protocol Normalization: GraphQL/Cap'n Proto Disconnection

## Status
**DISCONNECTED** - Full conversion pipeline exists but bypassed

## Problem
GraphQL and Cap'n Proto conversion to MCP format is fully implemented but never called. Protocol detection, conversion, and validation all exist but requests take a different path.

## Disconnected Components (32 items)

### 1. Cap'n Proto Conversion
**File**: `normalize/parsers.rs`
- `dynamic_value_to_json()` never called (line 439)
- `convert_struct_to_json()` never called (line 520)
- `convert_list_to_json()` never called (line 554)

### 2. GraphQL Conversion
**File**: `normalize/parsers.rs`
- `graphql_value_to_json()` never called (line 1333)
- `graphql_value_to_json_with_variables()` never called
- `json_to_graphql_value()` never called
- `parse_graphql_variables()` never called

### 3. GraphQL Schema Introspection (17 fields dead)
**File**: `normalize/schema_introspection.rs`

**SchemaData fields never read** (lines 121-127):
- `query_type: Option<TypeRef>`
- `mutation_type: Option<TypeRef>`
- `subscription_type: Option<TypeRef>`

**TypeData fields never read**:
- `description`
- `enum_values`
- `input_fields`

**FieldData fields never read**:
- `description`
- `field_type`
- `args: Vec<InputValueData>`
- `is_deprecated`
- `deprecation_reason`

**TypeRef fields never read**:
- `kind`
- `of_type`

**EnumValueData fields never read**:
- `name`
- `description`
- `is_deprecated`
- `deprecation_reason`

**InputValueData fields never read**:
- `name`
- `description`
- `input_type`
- `default_value`

### 4. JSON-RPC Conversion
**File**: `normalize/parsers.rs`
- `normalize_to_jsonrpc()` never called
- `denormalize_from_jsonrpc()` never called
- `to_json_rpc()` never called
- `create_method_name()` never called

### 5. Protocol Detection
**File**: `normalize/parsers.rs` or detection module
- `quick_detect_protocol()` never called

### 6. GraphQL Operation Extraction
**File**: `normalize/parsers.rs`
- `extract_operation_name()` never called
- `extract_operation_type()` never called
- `extract_field_arguments()` never called

### 7. Error Response Creation
**File**: `normalize/parsers.rs`
- `create_graphql_error()` never called
- `create_error_response()` never called

### 8. Protocol Context Fields (10 unused)
**File**: `normalize/mod.rs` (ProtocolContext struct)
- `content_type` never read
- `user_agent` never read
- `custom_headers` never read
- `conversion_start` never read
- `error_message` never read
- `preserve_field_names` never read
- `validate_jsonrpc` never read
- `max_depth` never read
- `timeout_ms` never read

### 9. Validation Functions
- `validate_graphql_query()` never called
- `validate_json_rpc()` never called
- `validate_capnp_format()` never called

## Root Cause Analysis

### Current Request Flow
Requests likely bypass normalization entirely:
1. Request arrives
2. Auth check
3. Direct proxy to upstream
4. No protocol conversion

### Expected Flow (Disconnected)
1. Request arrives
2. `quick_detect_protocol()` → GraphQL/JSON-RPC/Cap'n Proto
3. Protocol-specific validation
4. Conversion to MCP format
5. Forward to upstream
6. Convert MCP response back to original protocol

## Investigation Required

### 1. Find Request Entry Point
```bash
grep -r "ProxyHttp::request_filter\|upstream_peer" src/edge/
grep -r "async fn handle_request\|process_request" src/
```

### 2. Check if Normalization Called
```bash
grep -r "normalize::.*convert\|ProtocolContext::new" src/
grep -r "detect_protocol\|protocol_type" src/
```

### 3. Find Schema Introspection Usage
```bash
grep -r "SchemaData\|TypeData\|FieldData" src/ --include="*.rs" | grep -v "^.*\.rs:.*struct\|^.*\.rs:.*pub "
```

## Reconnection Steps

### 1. Enable Protocol Detection
**File**: Find request handler (likely `edge/core/proxy_impl.rs`)
```rust
async fn request_filter(...) {
    let protocol = quick_detect_protocol(&request_body, &headers)?;

    let normalized = match protocol {
        Protocol::GraphQL => convert_graphql_to_mcp(request_body)?,
        Protocol::JsonRpc => convert_jsonrpc_to_mcp(request_body)?,
        Protocol::Capnp => convert_capnp_to_mcp(request_body)?,
        Protocol::Mcp => request_body, // Already MCP
    };

    // Store protocol in context for response conversion
    ctx.protocol = Some(protocol);
}
```

### 2. Enable Response Conversion
**File**: Response handler
```rust
async fn response_filter(...) {
    if let Some(protocol) = ctx.protocol {
        let response_body = match protocol {
            Protocol::GraphQL => denormalize_from_mcp_to_graphql(mcp_response)?,
            Protocol::JsonRpc => denormalize_from_mcp_to_jsonrpc(mcp_response)?,
            // ...
        };
    }
}
```

### 3. Use Schema Introspection
**File**: GraphQL conversion
```rust
fn convert_graphql_to_mcp(body: &str) -> Result<McpRequest> {
    let graphql_query = parse_graphql(body)?;

    // Use introspection to understand schema
    let schema_data = fetch_schema_introspection()?;

    // Read query_type, mutation_type, subscription_type
    let operation_type = determine_operation(&graphql_query, &schema_data);

    // Read field metadata (args, types, deprecation)
    let mcp_tools = convert_fields_to_tools(&schema_data.query_type?.fields);
}
```

### 4. Populate ProtocolContext
**File**: Protocol conversion
```rust
let mut ctx = ProtocolContext {
    content_type: headers.get("content-type"),
    user_agent: headers.get("user-agent"),
    custom_headers: extract_custom_headers(headers),
    conversion_start: Instant::now(),
    preserve_field_names: config.preserve_field_names,
    validate_jsonrpc: true,
    max_depth: 10,
    timeout_ms: 5000,
    error_message: None,
    // ... other fields
};
```

### 5. Enable Validation
```rust
match protocol {
    Protocol::GraphQL => validate_graphql_query(&body)?,
    Protocol::JsonRpc => validate_json_rpc(&body)?,
    Protocol::Capnp => validate_capnp_format(&body)?,
}
```

## Files to Modify
- `edge/core/proxy_impl.rs` - Add protocol detection to request_filter
- `normalize/mod.rs` - Create public conversion functions
- `normalize/parsers.rs` - Export conversion functions
- Response filter - Add denormalization

## Testing
1. Send GraphQL query → verify conversion to MCP → verify response converted back
2. Send JSON-RPC 2.0 → verify normalization
3. Send Cap'n Proto binary → verify parsing
4. Verify schema introspection fields populated and used
5. Test error responses in each protocol format

## Expected Behavior After Reconnection
1. ✅ GraphQL queries converted to MCP tools/resources
2. ✅ JSON-RPC methods mapped to MCP
3. ✅ Cap'n Proto structs deserialized
4. ✅ Schema introspection enables smart conversion
5. ✅ Errors returned in original protocol format
6. ✅ All protocol context fields populated and used

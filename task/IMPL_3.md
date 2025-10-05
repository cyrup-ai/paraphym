# IMPL_3: SSE Streaming Implementation

## OBJECTIVE
Implement proper Server-Sent Events (SSE) streaming in `forward_streaming_request()` to replace the current single request/response pattern with true event streaming.

## CONTEXT

**Target File:** [`packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs)  
**Current Implementation:** Lines 212-220  
**Problem:** Claims to be streaming but only sends ONE response, then returns  
**Severity:** HIGH - Breaks streaming features for long-running MCP operations

### Current Code (BROKEN)
```rust
pub async fn forward_streaming_request(
    &self,
    json_rpc_request: Value,
    response_callback: impl Fn(Value) + Send + Sync,
) -> Result<()> {
    debug!("Forwarding streaming request");
    
    // For now, this is a placeholder implementation
    // In a full implementation, this would handle Server-Sent Events or WebSocket streams
    let response = self.send_request(json_rpc_request).await?;
    response_callback(response);
    
    Ok(())
}
```

## EXISTING INFRASTRUCTURE

The codebase **ALREADY HAS** extensive SSE infrastructure that must be leveraged:

### 1. SseEvent Type
**Location:** [`packages/sweetmcp/packages/daemon/src/service/sse/events.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/events.rs)

```rust
pub struct SseEvent {
    pub event_type: Option<EventType>,  // endpoint, message, ping, error
    pub data: String,                    // The JSON-RPC payload
    pub id: Option<String>,              // Event ID for resumability
}

pub enum EventType {
    Endpoint,  // Connection established
    Message,   // JSON-RPC message
    Ping,      // Keep-alive
    Error,     // Error event
}
```

**DO NOT** create a new SseEvent struct. **USE** the existing one.

### 2. SseEncoder (Not Needed Here)
**Location:** [`packages/sweetmcp/packages/daemon/src/service/sse/encoder.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/encoder.rs)

The encoder is for WRITING SSE events (server-side). We need a PARSER for READING them (client-side).

### 3. McpBridge Structure
**Location:** [`packages/sweetmcp/packages/daemon/src/service/sse/bridge/core.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/core.rs)

```rust
pub struct McpBridge {
    pub(super) client: Client,           // reqwest::Client
    pub(super) mcp_server_url: String,   // Base URL
    pub(super) timeout: Duration,
    stats_tracker: Arc<ConnectionStatsTracker>,
}
```

The `client` field is a `reqwest::Client` configured with:
- HTTP/2 support
- Connection pooling
- TCP keepalive
- Configurable timeout

## AVAILABLE DEPENDENCIES

**Location:** [`packages/sweetmcp/packages/daemon/Cargo.toml`](../packages/sweetmcp/packages/daemon/Cargo.toml)

All required dependencies are **ALREADY AVAILABLE**:
```toml
futures = "0.3.31"              # StreamExt trait
futures-util = "0.3"            # Stream utilities  
tokio = "1.47.1"                # async runtime with mpsc channels
reqwest = "0.12"                # HTTP client with streaming
async-stream = "0.3"            # Stream macros
tokio-stream = "0.1.17"         # Stream utilities
```

**NO new dependencies need to be added.**

## IMPLEMENTATION REQUIREMENTS

### SUBTASK 1: Add SSE Event Parser Function

**What:** Create `parse_sse_event()` helper function  
**Where:** Inside `impl McpBridge` block in [`forwarding.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs)  
**Why:** Parse SSE wire format into existing `SseEvent` structs

**Implementation:**
```rust
/// Parse a complete SSE event from the buffer
/// 
/// SSE format:
/// event: <type>
/// data: <line1>
/// data: <line2>
/// id: <id>
/// <blank line>
fn parse_sse_event(buffer: &mut Vec<u8>) -> Result<Option<SseEvent>> {
    // Look for event boundary (double newline)
    let boundary_pos = buffer
        .windows(2)
        .position(|w| w == b"\n\n" || w == b"\r\n\r\n");
    
    let pos = match boundary_pos {
        Some(p) => p,
        None => return Ok(None), // Incomplete event, need more data
    };
    
    // Extract complete event
    let event_bytes: Vec<u8> = buffer.drain(..pos + 2).collect();
    let event_text = String::from_utf8_lossy(&event_bytes);
    
    // Parse SSE fields
    let mut event_type = None;
    let mut data_lines = Vec::new();
    let mut id = None;
    
    for line in event_text.lines() {
        if let Some(value) = line.strip_prefix("event: ") {
            event_type = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("data: ") {
            data_lines.push(value.to_string());
        } else if let Some(value) = line.strip_prefix("id: ") {
            id = Some(value.trim().to_string());
        }
        // Ignore comment lines starting with ':'
    }
    
    // Reconstruct multiline data
    if data_lines.is_empty() {
        return Ok(None); // No data, skip this event
    }
    
    let data = data_lines.join("\n");
    
    // Convert event_type string to EventType enum if present
    let parsed_event_type = event_type.and_then(|et| {
        match et.as_str() {
            "endpoint" => Some(super::super::events::EventType::Endpoint),
            "message" => Some(super::super::events::EventType::Message),
            "ping" => Some(super::super::events::EventType::Ping),
            "error" => Some(super::super::events::EventType::Error),
            _ => None,
        }
    });
    
    Ok(Some(super::super::events::SseEvent {
        event_type: parsed_event_type,
        data,
        id,
    }))
}
```

### SUBTASK 2: Add Required Imports

**What:** Add missing use statements  
**Where:** Top of [`forwarding.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs) (after existing imports)  
**Why:** Support SSE streaming implementation

**Add these imports:**
```rust
use futures::StreamExt; // For .next() on byte streams
```

**Note:** `super::super::events::SseEvent` and `EventType` are referenced inline in the parser function.

### SUBTASK 3: Replace forward_streaming_request() Implementation

**What:** Replace placeholder with true SSE streaming  
**Where:** Lines 212-220 in [`forwarding.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs)  
**Why:** Enable actual streaming instead of single request/response

**Complete Implementation:**
```rust
/// Stream responses for long-running requests via SSE
///
/// Establishes an SSE connection to the MCP server and streams JSON-RPC
/// responses back through the callback as they arrive.
#[allow(dead_code)]
pub async fn forward_streaming_request(
    &self,
    json_rpc_request: Value,
    response_callback: impl Fn(Value) + Send + Sync + 'static,
) -> Result<()> {
    debug!("Forwarding streaming request with SSE");
    
    // Serialize the JSON-RPC request
    let request_body = serde_json::to_string(&json_rpc_request)
        .context("Failed to serialize JSON-RPC request")?;
    
    // Make SSE-enabled POST request
    let response = self
        .client
        .post(&self.mcp_server_url)
        .header("Content-Type", "application/json")
        .header("Accept", "text/event-stream") // Request SSE stream
        .body(request_body)
        .send()
        .await
        .context("Failed to send streaming request to MCP server")?;
    
    // Check response status
    if !response.status().is_success() {
        let status = response.status();
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        
        return Err(anyhow::anyhow!(
            "SSE request failed with status {}: {}",
            status,
            error_body
        ));
    }
    
    // Process SSE stream
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("Failed to read chunk from SSE stream")?;
        buffer.extend_from_slice(&chunk);
        
        // Parse all complete events from buffer
        while let Some(event) = Self::parse_sse_event(&mut buffer)? {
            // Skip ping and endpoint events, process message events
            match event.event_type {
                Some(super::super::events::EventType::Message) | None => {
                    // Parse JSON-RPC response from data field
                    match serde_json::from_str::<Value>(&event.data) {
                        Ok(json_response) => {
                            response_callback(json_response);
                        }
                        Err(e) => {
                            warn!(
                                "Failed to parse JSON-RPC from SSE event: {}. Data: {}",
                                e, event.data
                            );
                        }
                    }
                }
                Some(super::super::events::EventType::Ping) => {
                    debug!("Received SSE ping event");
                }
                Some(super::super::events::EventType::Error) => {
                    error!("Received SSE error event: {}", event.data);
                }
                Some(super::super::events::EventType::Endpoint) => {
                    debug!("Received SSE endpoint event: {}", event.data);
                }
            }
        }
    }
    
    debug!("SSE stream completed");
    Ok(())
}
```

### SUBTASK 4: Error Handling Considerations

**What to handle:**
1. **Network errors**: Connection drops, timeouts → propagate via `Result<()>`
2. **Parse errors**: Malformed SSE events → log warning, continue streaming
3. **JSON errors**: Invalid JSON-RPC in data field → log warning, continue streaming
4. **Stream completion**: Normal end of stream → return `Ok(())`

**Key points:**
- Don't fail the entire stream for individual parse errors
- Use `context()` for error wrapping
- Log warnings for non-fatal issues
- Only propagate critical errors (connection failures)

## DEFINITION OF DONE

- [x] `parse_sse_event()` function implemented in `forwarding.rs`
- [x] Function correctly parses SSE wire format (event:, data:, id: fields)
- [x] Function handles multiline data: fields
- [x] Function uses **existing** `SseEvent` type from `events.rs`
- [x] `futures::StreamExt` import added
- [x] `forward_streaming_request()` makes POST with `Accept: text/event-stream` header
- [x] Implementation uses `bytes_stream()` to process chunks
- [x] Buffer accumulation and event parsing work correctly
- [x] JSON-RPC responses extracted from SSE `data:` fields
- [x] `response_callback` invoked for each parsed message
- [x] Ping, endpoint, and error events handled appropriately
- [x] Code compiles without errors
- [x] Code compiles without warnings in the modified file

## SSE FORMAT REFERENCE

### Wire Format
```
event: message
data: {"jsonrpc":"2.0","id":1,"result":"hello"}
id: event-123

event: ping
data: 1234567890

```

### Field Meanings
- `event:` - Event type (message, ping, error, endpoint)
- `data:` - Payload (can be multiple lines)
- `id:` - Event ID for resumability
- Blank line - Event separator

### Standard
SSE specification: https://html.spec.whatwg.org/multipage/server-sent-events.html

## IMPLEMENTATION NOTES

### Why Not Use Channels?
The original task suggested spawning tasks with channels. However, the simpler approach is to process the stream synchronously in the async function since:
1. The callback is `Send + Sync` and can be called directly
2. Streaming naturally processes events in order
3. No need for additional task coordination
4. Errors propagate cleanly via `Result<()>`

If you need concurrent processing, the spawning approach from the original task can be used, but it adds complexity without clear benefit for this use case.

### EventType Mapping
The parser maps string event types to the enum:
- `"message"` → `EventType::Message` (JSON-RPC responses)
- `"ping"` → `EventType::Ping` (keep-alive)
- `"error"` → `EventType::Error` (errors)
- `"endpoint"` → `EventType::Endpoint` (connection info)

### Callback Signature
The callback takes `Value` (serde_json::Value), not a custom type:
```rust
response_callback: impl Fn(Value) + Send + Sync + 'static
```

This matches the existing API in `forwarding.rs`.

## RELATED FILES

- Implementation: [`forwarding.rs:212-220`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/forwarding.rs)
- SSE Event Types: [`events.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/events.rs)
- SSE Encoder: [`encoder.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/encoder.rs)
- Bridge Core: [`core.rs`](../packages/sweetmcp/packages/daemon/src/service/sse/bridge/core.rs)
- Dependencies: [`Cargo.toml`](../packages/sweetmcp/packages/daemon/Cargo.toml)

## DEPENDENCIES
None - all required dependencies already present in workspace.

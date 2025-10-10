# `packages/sweetmcp/packages/axum/src/notifications.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: axum
- **File Hash**: 1915cb4a  
- **Timestamp**: 2025-10-10T02:15:59.631160+00:00  
- **Lines of Code**: 176

---## Panic-Prone Code


### Line 119: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        self.send_json_rpc_notification(
            "$/progress",
            serde_json::to_value(&notification).unwrap(),
        )
        .await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 154: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        self.send_json_rpc_notification(
            "$/context/changed",
            serde_json::to_value(&notification).unwrap(),
        )
        .await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 204: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
        };

        self.send_json_rpc_notification("$/cancelled", serde_json::to_value(notification).unwrap())
            .await;
    }
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `handle_cancelled_notification()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/notifications.rs` (line 251)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Handler for cancelled notification
pub fn handle_cancelled_notification(params: CancelledNotification) {
    log::info!(
        "Received cancelled notification: request_id={}, reason={:?}",
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `format_notification()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/notifications.rs` (line 221)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Format a notification as a JSON-RPC notification message
pub fn format_notification(method: &str, params: Value) -> String {
    let notification = JsonRpcNotification {
        jsonrpc: crate::JSONRPC_VERSION.to_string(),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `handle_initialized_notification()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/notifications.rs` (line 246)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Handler for initialized notification
pub fn handle_initialized_notification() {
    log::info!("Received initialized notification");
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `init_notification_system()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/notifications.rs` (line 239)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Initialize the notification system with a JSON-RPC sender
pub async fn init_notification_system(json_rpc_sender: mpsc::Sender<NotificationPayload>) {
    NOTIFICATION_REGISTRY
        .set_json_rpc_sender(json_rpc_sender)
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
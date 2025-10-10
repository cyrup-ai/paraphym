# `forks/surrealdb/tests/common/socket.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: surrealdb
- **File Hash**: edc3d2c9  
- **Timestamp**: 2025-10-10T02:16:01.059320+00:00  
- **Lines of Code**: 413

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 413 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Panic-Prone Code


### Line 87: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	pub async fn connect(addr: &str, format: Option<Format>, msg_format: Format) -> Result<Self> {
		let url = format!("ws://{addr}/rpc");
		let mut req = url.into_client_request().unwrap();
		if let Some(v) = format.map(|v| v.to_string()) {
			req.headers_mut().insert("Sec-WebSocket-Protocol", v.parse().unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 89: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		let mut req = url.into_client_request().unwrap();
		if let Some(v) = format.map(|v| v.to_string()) {
			req.headers_mut().insert("Sec-WebSocket-Protocol", v.parse().unwrap());
		}
		let (stream, _) = connect_async(req).await?;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 116: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
	) -> Result<Self> {
		let url = format!("ws://{addr}/rpc");
		let mut req = url.into_client_request().unwrap();
		if let Some(v) = format.map(|v| v.to_string()) {
			req.headers_mut().insert("Sec-WebSocket-Protocol", v.parse().unwrap());
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 118: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
		let mut req = url.into_client_request().unwrap();
		if let Some(v) = format.map(|v| v.to_string()) {
			req.headers_mut().insert("Sec-WebSocket-Protocol", v.parse().unwrap());
		}
		for (key, value) in headers.into_iter() {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 183: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Should use .expect() with descriptive message in tests

```rust
						let msg = surrealdb_core::rpc::format::cbor::decode(msg.as_slice())?;
						// Then we convert the SurrealQL to JSON.
						let msg = msg.into_json_value().unwrap();
						// Then output the response.
						debug!("Received message: {msg:?}");
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
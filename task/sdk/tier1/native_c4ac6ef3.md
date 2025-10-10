# `forks/surrealdb/crates/sdk/src/api/engine/remote/ws/native.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: sdk
- **File Hash**: c4ac6ef3  
- **Timestamp**: 2025-10-10T02:16:00.932614+00:00  
- **Lines of Code**: 707

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 707 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 791
  - TODO
  - 

```rust
		}

		// TODO: Figure out what this test was supposed to track.
		//
		//	Note this test changed with the value inversion PR, below is the previous
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 236: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust

		// Unwrap because a router request cannot fail to serialize.
		let payload = crate::core::rpc::format::revision::encode(&request).unwrap();

		Message::Binary(payload)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 292: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
														.response_channel
														.send(Ok(DbResponse::Other(
															array.into_iter().next().unwrap(),
														)))
														.await;
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 344: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
											}
											.into_router_request(None)
											.unwrap();

											let value = crate::core::rpc::format::revision::encode(
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 349: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
												&request,
											)
											.unwrap();
											Message::Binary(value)
										};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 425: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
						.expect("replay commands should always convert to route requests");

					let message = crate::core::rpc::format::revision::encode(&request).unwrap();

					if let Err(error) = state.sink.send(Message::Binary(message)).await {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 439: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
					}
					.into_router_request(None)
					.unwrap();
					trace!("Request {:?}", request);
					let payload = crate::core::rpc::format::revision::encode(&request).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 441: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
					.unwrap();
					trace!("Request {:?}", request);
					let payload = crate::core::rpc::format::revision::encode(&request).unwrap();

					if let Err(error) = state.sink.send(Message::Binary(payload)).await {
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 469: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
) {
	let ping = {
		let request = Command::Health.into_router_request(None).unwrap();
		let value = crate::core::rpc::format::revision::encode(&request).unwrap();
		Message::Binary(value)
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 470: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
	let ping = {
		let request = Command::Health.into_router_request(None).unwrap();
		let value = crate::core::rpc::format::revision::encode(&request).unwrap();
		Message::Binary(value)
	};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 647: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			let start = SystemTime::now();
			let r = func();
			(start.elapsed().unwrap(), r)
		};
		//
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 652: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
		let compress = |v: &Vec<u8>| {
			let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
			encoder.write_all(v).unwrap();
			encoder.finish().unwrap()
		};
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 653: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
			encoder.write_all(v).unwrap();
			encoder.finish().unwrap()
		};
		// Generate a random vector
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 681: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
					.with_fixint_encoding()
					.serialize_into(&mut payload, &vector)
					.unwrap();
				payload
			});
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 705: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
					.with_varint_encoding()
					.serialize_into(&mut payload, &vector)
					.unwrap();
				payload
			});
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 725: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			// Unversioned
			let (duration, payload) =
				timed(&|| crate::core::rpc::format::bincode::encode(&vector).unwrap());
			results.push((
				payload.len(),
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 749: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			// Versioned
			let (duration, payload) =
				timed(&|| crate::core::rpc::format::revision::encode(&vector).unwrap());
			results.push((payload.len(), VERSIONED, duration, payload.len() as f32 / ref_payload));

```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 768: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
			// CBor
			let (duration, payload) = timed(&|| {
				let cbor = rpc::format::cbor::encode(vector.clone()).unwrap();
				let mut res = Vec::new();
				ciborium::into_writer(&cbor, &mut res).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 770: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
				let cbor = rpc::format::cbor::encode(vector.clone()).unwrap();
				let mut res = Vec::new();
				ciborium::into_writer(&cbor, &mut res).unwrap();
				res
			});
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "


### Line 423: `.expect()`

- **Pattern**: .expect()
- **Issue**: Can panic in production code

```rust
						.clone()
						.into_router_request(None)
						.expect("replay commands should always convert to route requests");

					let message = crate::core::rpc::format::revision::encode(&request).unwrap();
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Tests in Source Directory


### Line 630: `#[cfg]`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/remote/ws/native.rs` (line 630)
- **Issue**: Tests must be in `./tests` directory, not in `./src`

```rust

#[cfg(test)]
mod tests {
	use std::io::Write;
	use std::time::SystemTime;
```

### Action Required

- Extract tests into `./tests` directory
  - `tests/` should mirror the file structure of the `src/` with file names prepended with `test_`
  - Update this section with specific remediation instructions
  

## Orphaned Methods


### `router_handle_response()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/remote/ws/native.rs` (line 260)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

async fn router_handle_response(response: Message, state: &mut RouterState) -> HandleResult {
	match Response::try_from(&response) {
		Ok(option) => {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `router_reconnect()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/remote/ws/native.rs` (line 406)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

async fn router_reconnect(
	maybe_connector: &Option<Connector>,
	config: &WebSocketConfig,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `large_vector_serialisation_bench()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/remote/ws/native.rs` (line 642)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

	#[test_log::test]
	fn large_vector_serialisation_bench() {
		//
		let timed = |func: &dyn Fn() -> Vec<u8>| {
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `router_handle_route()`

- **Location**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk/src/api/engine/remote/ws/native.rs` (line 140)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
}

async fn router_handle_route(
	Route {
		request,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
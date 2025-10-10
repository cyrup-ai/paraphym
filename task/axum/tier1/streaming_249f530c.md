# `packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: axum
- **File Hash**: 249f530c  
- **Timestamp**: 2025-10-10T02:15:59.626472+00:00  
- **Lines of Code**: 473

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 473 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Tier 1 Infractions 


- Line 209
  - FIX
  - 

```rust
    let (_query_id, receiver) = manager.subscribe_to_table(&table).await?;
    
    // CRITICAL FIX: Store the receiver to keep it alive
    SUBSCRIPTION_REGISTRY.insert(uri.clone(), (table.clone(), receiver));
    
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Tier 3 Evaluations


- Line 351
  - Fallback
  - 

```rust
        format!("cms://node/{}", slug)
    } else {
        // Fallback to a generated ID based on title
        let normalized_title = row.title
            .chars()
```

- is this actually a non-production indicator or a false positive? If false positive, remove it from the task file.
- If IT IS a non-production fake, fabrication, incomplete, dangeours or lacking implementation: add detailed notes explaining the issue and plan out the necessary replacement work in sequential steps. 
- Update this section of the task file with the notes and plan.

## Panic-Prone Code


### Line 86: `.unwrap()`

- **Pattern**: .unwrap()
- **Issue**: Can panic in production code

```rust
                                        log::error!("Fallback URI creation failed for {}: {}", notification.query_id, e);
                                        // Use a guaranteed-valid static URL
                                        Url::parse("cms://node/error").unwrap()
                                    })
                            });
```

### Action Required

- unwrap() should never be used in `./src/**/*.rs` or `./tests/**/*.rs` (period). The code should be updated with proper error handling and all match arms addressed.
- unwrap_or_else() is a-ok. 
- expect() should never be used in `./src/**/*.rs` but should ALWAYS BE USED in `./tests/**/*.rs` (rather than unwrap)
- panic can be approved with my written consent for situations that should in practice never happen  
  - ASK FOR WRITTEN PERMISSION
  - If granted, annotate the code with a comment "APPROVED PANIC "

## Orphaned Methods


### `stream_resources_by_parent()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 393)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources by parent
pub fn stream_resources_by_parent(parent: String) -> ResourceStream {
    let request = ListResourcesRequest {
        parent: Some(parent),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_advanced()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 431)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Advanced resource streaming with multiple filters
pub fn stream_resources_advanced(request: ListResourcesRequest) -> ResourceStream {
    resources_list_stream(Some(request))
}
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_with_retry()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 603)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources with error recovery
pub fn stream_resources_with_retry(
    request: Option<ListResourcesRequest>,
    max_retries: usize,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_paginated_resources()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 411)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream paginated resources
pub fn stream_paginated_resources(limit: u32, offset: u32) -> ResourceStream {
    let request = ListResourcesRequest {
        limit: Some(limit),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_custom_query()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 436)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources with custom query
pub fn stream_resources_custom_query(query: String) -> ResourceStream {
    let (tx, rx) = mpsc::channel(16);

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_sorted_resources()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 421)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream sorted resources
pub fn stream_sorted_resources(sort_field: String, sort_direction: String) -> ResourceStream {
    let request = ListResourcesRequest {
        sort_field: Some(sort_field),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_with_search()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 402)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources with search query
pub fn stream_resources_with_search(query: String) -> ResourceStream {
    let request = ListResourcesRequest {
        search_query: Some(query),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_realtime()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 473)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources with real-time updates
pub fn stream_resources_realtime(request: Option<ListResourcesRequest>) -> ResourceStream {
    let (tx, rx) = mpsc::channel(32); // Larger buffer for real-time updates

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_by_tags()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 384)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources by tags
pub fn stream_resources_by_tags(tags: Vec<String>) -> ResourceStream {
    let request = ListResourcesRequest {
        tags: Some(tags),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_by_type()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 375)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Stream resources by type
pub fn stream_resources_by_type(resource_type: String) -> ResourceStream {
    let request = ListResourcesRequest {
        resource_types: Some(vec![resource_type]),
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `stream_resources_batched()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs` (line 542)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust

/// Batch stream resources in chunks
pub fn stream_resources_batched(
    request: Option<ListResourcesRequest>,
    batch_size: usize,
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
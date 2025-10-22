# Performance Issue: Unnecessary String Cloning in Hot Path

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Lines 375-379 (embed_text) and 459-463 (batch_embed_text)

## Severity
**MEDIUM** - Performance degradation on every request

## Issue Description

Every request allocates and clones strings unnecessarily:

```rust
// Line 375-379
worker
    .embed_tx
    .send(EmbedRequest {
        text: text.to_string(),  // <-- Unnecessary allocation
        task,
        response: response_tx,
    })
```

The function signature is:
```rust
pub async fn embed_text(
    &self,
    registry_key: &str,
    text: &str,  // <-- Already a borrowed string
    task: Option<String>,
) -> Result<Vec<f32>, PoolError>
```

## Impact

1. **Memory Allocation**: Every request allocates a new String
2. **CPU Overhead**: String cloning on every embed operation
3. **Cache Pressure**: More allocations = worse cache performance
4. **Throughput**: Reduced requests/second in high-load scenarios

For a 1KB text input:
- Current: Allocates 1KB + String metadata per request
- If 1000 req/sec: 1MB/sec of unnecessary allocations

## Root Cause

The `EmbedRequest` struct owns the text:

```rust
pub struct EmbedRequest {
    pub text: String,  // <-- Owns the string
    pub task: Option<String>,
    pub response: oneshot::Sender<Result<Vec<f32>, PoolError>>,
}
```

## Fix Options

### Option 1: Use Arc<str> (Recommended)

```rust
pub struct EmbedRequest {
    pub text: Arc<str>,  // Cheap clone, shared ownership
    pub task: Option<String>,
    pub response: oneshot::Sender<Result<Vec<f32>, PoolError>>,
}

// Usage
worker.embed_tx.send(EmbedRequest {
    text: Arc::from(text),  // Single allocation, shared
    task,
    response: response_tx,
})
```

### Option 2: Use Cow<'static, str>

Only works if text lifetime can be extended.

### Option 3: Accept the clone

If text is typically small (<100 bytes), the overhead may be acceptable.

## Batch Embed Impact

For `batch_embed_text`, the issue is worse:

```rust
// Line 459-463
worker.batch_embed_tx.send(BatchEmbedRequest {
    texts: texts.to_vec(),  // <-- Clones entire Vec<String>
    task,
    response: response_tx,
})
```

If `texts` contains 100 strings of 1KB each:
- Allocates 100KB per batch request
- Clones 100 String objects

## Recommendation

1. Change `EmbedRequest.text` to `Arc<str>`
2. Change `BatchEmbedRequest.texts` to `Arc<[String]>` or `Vec<Arc<str>>`
3. Benchmark before/after to quantify improvement

## Measurement

Add metrics to track:
- Allocation rate before/after
- Throughput (requests/sec) before/after
- Memory usage under load

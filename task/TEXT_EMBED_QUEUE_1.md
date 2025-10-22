# Queue Depth Reporting Always Zero

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

Line 162

## Severity
**LOW** - Missing observability

## Issue Description

Queue depth is hardcoded to 0:

```rust
let pong = HealthPong {
    worker_id,
    timestamp: now,
    queue_depth: 0, // Note: tokio mpsc doesn't expose len()
};
```

## Impact

- No visibility into queue backlog
- Can't detect overloaded workers
- Metrics are incomplete

## Fix

The comment is incorrect for bounded channels. If using bounded channels:

```rust
let embed_depth = embed_rx.len();
let batch_depth = batch_embed_rx.len();
let total_depth = embed_depth + batch_depth;

let pong = HealthPong {
    worker_id,
    timestamp: now,
    queue_depth: total_depth,
};
```

## Recommendation

Fix after implementing bounded channels (TEXT_EMBED_PERF_2).

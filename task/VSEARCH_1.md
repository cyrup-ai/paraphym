# VSEARCH_1: Complete RequestInfo Callback Support

## OBJECTIVE
Add RequestInfo callback support to enable user interaction for ambiguous search results. This completes the multi-stage cognitive filtering system.

## LOCATION
`packages/candle/src/memory/vector/vector_search.rs`

## OUTSTANDING WORK

### Add request_info_callback field to SearchOptions struct

**File**: `packages/candle/src/memory/vector/vector_search.rs:115`

Add this field to the `SearchOptions` struct:

```rust
pub struct SearchOptions {
    // ... existing fields ...
    
    /// Optional callback for RequestInfo outcomes requiring user interaction
    /// Callback receives: (result_id, similarity, confidence) -> bool (accept/reject)
    pub request_info_callback: Option<Arc<dyn Fn(&str, f32, f32) -> bool + Send + Sync>>,
}
```

### Update RequestInfo case to use callback

**File**: `packages/candle/src/memory/vector/vector_search.rs:443-451`

Replace the current RequestInfo implementation:

```rust
// CURRENT (incomplete):
DecisionOutcome::RequestInfo => {
    tracing::debug!(
        "CognitiveProcessor REQUEST_INFO: similarity={:.4}",
        similarity
    );
    // Treat as deferred for future callback support
    state.deferred_results.push((
        id,
        vector,
        similarity,
        metadata,
        decision.confidence,
    ));
}
```

With the complete callback implementation:

```rust
// REQUIRED:
DecisionOutcome::RequestInfo => {
    tracing::debug!(
        "CognitiveProcessor REQUEST_INFO: similarity={:.4}",
        similarity
    );
    if let Some(ref callback) = options.request_info_callback {
        let should_accept = callback(&id, similarity, decision.confidence);
        if should_accept {
            tracing::debug!(
                "RequestInfo callback accepted: id={}, similarity={:.4}",
                id,
                similarity
            );
            state.final_results.push((id, vector, similarity, metadata));
        } else {
            tracing::debug!(
                "RequestInfo callback rejected: id={}, similarity={:.4}",
                id,
                similarity
            );
            // Rejected by callback - exclude from results
        }
    } else {
        // Fallback: treat as deferred
        tracing::trace!(
            "No RequestInfo callback provided, treating as deferred: id={}",
            id
        );
        state.deferred_results.push((
            id,
            vector,
            similarity,
            metadata,
            decision.confidence,
        ));
    }
}
```

## DEFINITION OF DONE

- [ ] request_info_callback field added to SearchOptions struct
- [ ] RequestInfo case checks for callback presence
- [ ] Callback invoked with (result_id, similarity, confidence) parameters
- [ ] Callback return value determines accept/reject decision
- [ ] Accepted results added to final_results with debug logging
- [ ] Rejected results excluded (not added to any queue) with debug logging
- [ ] Fallback to deferred queue when no callback provided with trace logging
- [ ] Code compiles without warnings (`cargo check -p paraphym_candle`)

## COMPLETED WORK (DO NOT MODIFY)

✅ CognitiveSearchState struct created with deferred_results and final_results fields
✅ DecisionOutcome::Defer case queues results with confidence scores  
✅ process_deferred_results() function implements secondary threshold evaluation
✅ Two-stage filtering operational: Stage 1 (classify) → Stage 2 (process deferred)
✅ All "for now" comments removed
✅ Comprehensive tracing added for Accept, Defer, Reject paths
# Fix Incorrect block_on Usage in llava.rs Lines 255 & 270

## Location
`packages/candle/src/capability/vision/llava.rs:255` and `270`

## Problem
Using `block_on` in a dedicated model thread for processing requests.

```rust
// Line 255
let result = rt.block_on(async {
    Self::process_ask(
        &model,
        &tokenizer,
        // ...
    ).await
});

// Line 270
let result = rt.block_on(async {
    Self::process_ask_url(
        &model,
        &tokenizer,
        // ...
    ).await
});
```

## Context
LLaVA model runs in a dedicated thread due to Candle's non-Send trait objects. The thread uses `block_on` to call async processing methods.

## Why It's Complex
This case is more nuanced because:
- Candle's LLaVA model contains non-Send trait objects
- Cannot be moved across async task boundaries
- Requires thread-local execution

## Potential Solutions

### Option 1: Keep thread but improve pattern
If the model truly must stay thread-local, ensure the thread is properly async-aware and uses `block_on` only at the thread boundary, not for every request.

### Option 2: Refactor to async-compatible model
Investigate if newer Candle versions support Send or if wrapper types can make it Send-safe, then convert to proper tokio tasks.

### Option 3: Dedicated async runtime per thread
Each model thread could have its own `tokio::runtime::Runtime` instead of using shared_runtime().block_on().

## Recommended Approach
This needs deeper investigation into Candle's Send constraints. Document why the model is non-Send and evaluate if the thread-based approach is truly necessary.

## Impact
- Medium priority (architectural constraint)
- May require upstream Candle changes
- Consider for future refactoring

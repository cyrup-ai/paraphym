# Task 009: Verify paraphym-simd thread safety

## Problem
Before moving SIMD operations to spawn_blocking, need to verify they're thread-safe.

## Operations to Check
From `src/core/generation/generator.rs:258-330`:
- `scale_temperature()` - paraphym-simd
- `topk_filtering_simd()` - paraphym-simd
- `prepare_nucleus_sampling_simd()` - paraphym-simd
- `softmax()` - paraphym-simd
- `argmax()` - paraphym-simd

## Investigation Steps
1. Check paraphym-simd source code
2. Verify no global state
3. Verify no thread-local storage
4. Check for any mutex/locks
5. Test with concurrent spawn_blocking calls

## Location
- `packages/simd/src/` - Need to audit all SIMD functions

## Impact
**BLOCKING** - Must verify before Task 004 can proceed

## Expected Result
All SIMD operations should be pure functions with no shared state, making them thread-safe.

## Estimated Effort
2 hours

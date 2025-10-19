# Master Task Summary: Fix paraphym_candle Performance

## Problem Statement

Phi-4 model in paraphym_candle shows **zero output** and takes **>120 seconds** before timeout.

Candle baseline (same model, same hardware) works perfectly:
- ✅ Loads in **0.36 seconds**
- ✅ Generates at **93.87 tokens/s**
- ✅ No hangs or issues

**Conclusion**: The problem is in paraphym_candle wrapper code, NOT Candle or Metal.

## Root Causes Identified

1. **Excessive spawn_blocking in generator.rs**
   - Wrapping fast CPU/Metal operations unnecessarily
   - Adding 10-100x overhead
   - Causing deadlocks and race conditions

2. **Disabled Pre-warming**
   - Feature removed without authorization
   - System now spawns only 1 worker
   - Original design spawned 2 for better throughput

3. **Complex Async Wrapping**
   - Over-engineered model loading
   - Unnecessary tokio task spawning
   - Candle examples use simple synchronous code

## Task Execution Order

Execute tasks in this order:

### Phase 1: Baseline & New Model (Proof of Concept)
1. ✅ **001-baseline-candle-qwen3-benchmark.md** - COMPLETED
   - Baseline: 0.36s load, 93.87 tokens/s
   - Proves Candle+Metal works perfectly

2. **002-convert-qwen3coder-to-quantized.md** - NEXT
   - Convert Qwen3Coder to use quantized model
   - Use Candle's simple pattern (from examples)
   - No spawn_blocking
   - Keep same struct names - minimal changes

3. **003-fix-generator-remove-spawn-blocking.md**
   - Remove ALL spawn_blocking from hot path
   - Match Candle's synchronous approach
   - Fix tokenizer, tensor, logits operations

### Phase 2: Integration & Testing
4. **005-make-qwen3-default-model.md**
   - Set Qwen3 as default
   - Update examples
   - Test end-to-end

5. **006-benchmark-qwen3-in-paraphym.md**
   - Compare to baseline
   - Verify performance matches
   - Document any overhead

### Phase 3: Restore Features
6. **004-restore-registry-pre-warming.md**
   - Restore 2-worker cold start
   - Keep wait-for-ready fix
   - Ensure no deadlocks

7. **008-restore-adaptive-worker-scaling.md**
   - Restore full adaptive scaling
   - Test scale-up under load
   - Verify idle eviction works

### Phase 4: Fix Original Model
8. **007-fix-phi4-reasoning-model.md**
   - Apply same fixes to Phi4
   - Use proven Qwen3 pattern
   - Verify performance

### Phase 5: Cleanup
9. **009-cleanup-and-final-testing.md**
   - Remove debug logging
   - Code quality checks
   - Comprehensive testing
   - Performance documentation

## Success Criteria

### Minimum Acceptable Performance
- **Qwen3 1.7B**: Load < 1s, Generate > 70 tokens/s
- **Phi4 7.8B**: Load < 5s, Generate > 40 tokens/s
- **No Hangs**: All models start generating immediately
- **Concurrent**: 3+ simultaneous requests work

### Features Working
- ✅ Pre-warming (2 workers on cold start)
- ✅ Adaptive scaling (up to max_workers)
- ✅ Idle eviction (5 minute timeout)
- ✅ Memory governance
- ✅ Circuit breakers
- ✅ Health checks

## Timeline Estimate

- Phase 1: 3-4 hours (critical path)
- Phase 2: 1-2 hours (testing)
- Phase 3: 1-2 hours (restore features)
- Phase 4: 1 hour (apply to Phi4)
- Phase 5: 1-2 hours (cleanup)

**Total**: 7-11 hours of focused work

## Notes

- Each task is independent and testable
- Can parallelize some tasks after Phase 1
- Benchmark after each major change
- Don't skip testing steps
- Document all performance numbers

## Key Learning

**The Candle examples are the source of truth.**

When in doubt, check how Candle's examples do it:
- Simple synchronous code
- Direct tensor operations
- No excessive async wrapping
- Fast and reliable

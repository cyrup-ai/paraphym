# Task: Cleanup and Final End-to-End Testing

## Objective
Remove debug logging, clean up code, and perform comprehensive testing of all models and features.

## Cleanup Tasks

### 1. Remove Debug Logging

Files with excessive `>>>` and emoji logging to clean:
- `/Volumes/samsung_t9/paraphym/packages/candle/src/core/generation/generator.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/phi4_reasoning.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/capabilities/text_to_text.rs`
- `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/core/spawn.rs`

Keep only meaningful logs at INFO level:
- Model loading completion
- Worker state transitions
- Performance metrics
- Error conditions

Remove debugging logs like:
- `>>> Worker X received prompt request`
- `>>> GENERATE STARTED`
- `🎬 Starting generation`
- `📦 Chunk #X`

### 2. Code Quality

Run checks:
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Format code
cargo fmt

# Check for warnings
cargo clippy --all-targets --all-features

# Run tests
cargo test
```

## Final Testing Matrix

### Test 1: Qwen3 (Default Model)
```bash
cargo run --example fluent_builder --release -- \
  --query "What is 2+2?" --max-tokens 50
```
**Expected**: 
- Load: < 2s
- Generation: 80+ tokens/s
- Output: Correct response

### Test 2: Phi4 Reasoning
```bash
cargo run --example fluent_builder --release -- \
  --model phi-4-reasoning \
  --query "Solve: If x+5=10, what is x?" \
  --max-tokens 100
```
**Expected**:
- Load: < 5s (larger model)
- Generation: 40+ tokens/s
- Output: Step-by-step reasoning

### Test 3: Concurrent Requests
```bash
# Run 3 instances simultaneously
for i in {1..3}; do
  cargo run --example fluent_builder --release -- \
    --query "Count to $i" --max-tokens 20 &
done
wait
```
**Expected**:
- All complete successfully
- Workers scale up to 3
- No deadlocks or errors

### Test 4: Chat History
```bash
cargo run --example fluent_builder --release
# Use interactive mode
# Send multiple messages
# Verify history is maintained
```

### Test 5: Long Generation
```bash
cargo run --example fluent_builder --release -- \
  --query "Write a detailed explanation of photosynthesis" \
  --max-tokens 500
```
**Expected**:
- Consistent token generation speed
- No timeouts
- Complete response

## Performance Documentation

Create `/Volumes/samsung_t9/paraphym/benchmarks/final-performance-report.md`:

```markdown
# Final Performance Report

## Qwen3 1.7B (Default)
- Model Load: X.XXs
- First Token Latency: X.XXms
- Token Generation: X.XX tokens/s
- Comparison to Candle: X% overhead

## Phi4 Reasoning 7.8GB
- Model Load: X.XXs  
- First Token Latency: X.XXms
- Token Generation: X.XX tokens/s

## System Metrics
- Cold start: 2 workers spawned
- Concurrent capacity: 4 workers max
- Idle eviction: 5 minutes
- Memory governance: Working
- Circuit breakers: Working
- Health checks: Working

## Issues Resolved
- ✅ Excessive spawn_blocking removed
- ✅ Race conditions fixed
- ✅ Model loading optimized
- ✅ Pre-warming restored
- ✅ Adaptive scaling working

## Known Limitations
- [List any remaining issues]

## Next Steps
- [Future improvements]
```

## Success Criteria
- All tests pass
- Performance within 20% of Candle baseline
- No warnings or errors
- Code is clean and maintainable
- Documentation is complete

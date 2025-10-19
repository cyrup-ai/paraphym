# Task: Benchmark Qwen3 in paraphym_candle

## Objective
After implementing Qwen3, benchmark it against the Candle baseline to verify performance parity.

## Commands

```bash
cd /Volumes/samsung_t9/paraphym

# Build with metal features
cargo build --example fluent_builder --release --features metal

# Run benchmark
cargo run --example fluent_builder --release --features metal -- \
  --query "What is 2+2?" \
  --max-tokens 50
```

## Expected Results

Should match Candle baseline:
- **Model Load Time**: < 2 seconds (Candle: 0.36s)
- **Generation Speed**: 80+ tokens/s (Candle: 93.87 tokens/s)
- **No Hangs**: Immediate output
- **Consistent**: Multiple runs produce similar results

## Measurements to Record

Create `/Volumes/samsung_t9/paraphym/benchmarks/paraphym-qwen3-performance.md`:

```markdown
# paraphym_candle Qwen3 1.7B Performance

**Date**: [TIMESTAMP]
**Model**: unsloth/Qwen3-1.7B-GGUF (Q4_K_M)
**Features**: `--features metal`
**Architecture**: paraphym_candle with pool

## Comparison to Candle Baseline

| Metric | Candle Baseline | paraphym_candle | Delta |
|--------|----------------|-----------------|-------|
| Model Load | 0.36s | X.XXs | +X.XXs |
| Prompt Processing | 3.14 t/s | X.XX t/s | X.XX t/s |
| Token Generation | 93.87 t/s | X.XX t/s | X.XX t/s |

## Test Results

### Test 1: Simple Math
**Prompt**: "What is 2+2?"
**Max Tokens**: 50
- Model Load: X.XX seconds
- First Token Latency: X.XX seconds
- Generation Speed: X.XX tokens/s
- Total Time: X.XX seconds

### Test 2: Longer Generation
**Prompt**: "Explain quantum physics"
**Max Tokens**: 200
- Generation Speed: X.XX tokens/s
- Consistency: [PASS/FAIL]

## Analysis

### If Matching Baseline (EXPECTED)
- ✅ Architecture is sound
- ✅ Generator.rs fixes worked
- ✅ Pool overhead is minimal
- Ready to restore Phi4

### If Slower Than Baseline
- Identify bottlenecks with profiling
- Check for remaining spawn_blocking calls
- Verify Metal usage
- Review pool overhead
```

## Success Criteria
- Performance within 20% of Candle baseline
- Model loads in < 2 seconds
- Token generation > 70 tokens/s
- No hangs or timeouts

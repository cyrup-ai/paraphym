# INPROD_25: Sampling Parameters Configuration

## SEVERITY: LOW

## OBJECTIVE
Load sampling parameters (top_k, top_p) from configuration instead of using hardcoded defaults.

## LOCATION
- `packages/candle/src/providers/kimi_k2.rs`
- `packages/candle/src/providers/qwen3_coder.rs`

## CURRENT STATE
- kimi_k2.rs:393: `.with_top_k(50) // Default for now`
- kimi_k2.rs:394: `.with_top_p(0.9) // Default for now`
- qwen3_coder.rs:481: `.with_top_k(50) // Default for now`
- qwen3_coder.rs:482: `.with_top_p(0.9) // Default for now`
- Hardcoded values used instead of configuration or parameters

## SUBTASK 1: Load from Configuration or Parameters
- Check if params includes top_k and top_p values
- If present, use provided values
- Otherwise, load defaults from provider configuration
- Support per-provider default values

## SUBTASK 2: Update KimiK2Provider
- Locate kimi_k2.rs:393-394
- Replace hardcoded 50 and 0.9 with configured values
- Use params or provider config for defaults
- Remove "Default for now" comments

## SUBTASK 3: Update Qwen3CoderProvider
- Locate qwen3_coder.rs:481-482
- Replace hardcoded 50 and 0.9 with configured values
- Use params or provider config for defaults
- Ensure consistency with KimiK2Provider

## DEFINITION OF DONE
- [ ] Sampling parameters loaded from config or params
- [ ] No hardcoded 50 and 0.9 values remain
- [ ] Both providers updated consistently
- [ ] Provider-specific defaults supported
- [ ] Stub comments removed

## RESEARCH NOTES
- Review params structure and available fields
- Check provider configuration structures
- Examine SamplingConfig and parameter handling
- Look for configuration loading patterns

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

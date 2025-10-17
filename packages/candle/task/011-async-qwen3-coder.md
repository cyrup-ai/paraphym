# Task 011: Apply same fixes to CandleQwen3CoderModel

## Problem
Same issues as Phi4: LoadedQwen3CoderModel probably reloads model on every request

## Files to Audit
- `src/capability/text_to_text/qwen3_coder.rs`

## Changes Needed
1. Check if LoadedQwen3CoderModel caches the actual model or just file path
2. If broken, apply same fix as Task 007
3. Ensure tokenizer loading uses spawn_blocking
4. Ensure model loading uses spawn_blocking

## Dependencies
- Task 007 (to use as template)

## Status
**NOT STARTED** - Need to audit code first

## Estimated Effort
3 hours

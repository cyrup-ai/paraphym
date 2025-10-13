# Test Extraction Plan for packages/candle

## Overview
Extract all tests from `./src/**/*.rs` to `./tests/` directory with mirrored structure.
Tests must be prefixed with `test_` and match the filename of the source they test.

## Tools
- **nextest**: Already installed (v0.9.105) ✓
- **Command**: `cargo nextest run`

## Current State

### Existing Test Files in ./tests/ (10 files)
```
tests/domain_context_extraction_error.rs
tests/domain_voice_audio.rs
tests/domain_voice_mod.rs
tests/domain_voice_transcription.rs
tests/integration_generation_pipeline.rs
tests/memory/cognitive_integration_tests.rs
tests/memory/cognitive/state_tests.rs
tests/memory/mcp_memory_tests.rs
tests/memory/quantum_system_test.rs
tests/memory/test_quantum_mcts.rs
```

### Files with Tests in ./src/ (43 files)
```
src/cli/args.rs
src/cli/completion.rs
src/cli/config.rs
src/cli/handler.rs
src/cli/prompt.rs
src/context/extraction/error.rs
src/context/extraction/mod.rs
src/core/generation/config.rs
src/core/generation/stats.rs
src/core/generation/tokens.rs
src/core/generation/types.rs
src/core/model_config.rs
src/core/simd_adapters.rs
src/core/tokenizer/core.rs
src/domain/chat/formatting.rs
src/domain/chat/loop.rs
src/domain/chat/message/message_processing.rs
src/domain/chat/message/mod.rs
src/domain/chat/orchestration.rs
src/domain/chat/templates/parser.rs
src/domain/completion/prompt_formatter.rs
src/domain/context/extraction/mod.rs
src/domain/context/provider.rs
src/domain/model/error.rs
src/domain/util/json_util.rs
src/domain/util/notnan.rs
src/lib.rs
src/memory/core/mod.rs
src/memory/migration/converter.rs
src/memory/monitoring/metrics.rs
src/memory/monitoring/metrics_test.rs
src/memory/monitoring/mod.rs
src/memory/monitoring/tests/metrics_tests.rs
src/memory/schema/relationship_schema.rs
src/memory/transaction/mod.rs
src/memory/transaction/tests/transaction_manager_tests.rs
src/memory/vector/vector_index.rs
src/memory/vector/vector_repository.rs
src/util/input_resolver.rs
src/util/json_util.rs
src/voice/audio.rs
src/voice/mod.rs
src/voice/transcription.rs
src/workflow/parallel.rs
```

## Naming Convention

### Source to Test Mapping
For a source file at: `./src/module/submodule/file.rs`
The test file goes to: `./tests/module/submodule/test_file.rs`

Examples:
- `src/cli/args.rs` → `tests/cli/test_args.rs`
- `src/domain/chat/formatting.rs` → `tests/domain/chat/test_formatting.rs`
- `src/memory/vector/vector_index.rs` → `tests/memory/vector/test_vector_index.rs`

### Special Cases
- `src/lib.rs` → `tests/test_lib.rs`
- `src/domain/chat/message/mod.rs` → `tests/domain/chat/message/test_mod.rs`

## Extraction Process

### Phase 1: Preparation (CURRENT)
- [x] Verify nextest installation
- [x] Map all source files with tests
- [x] Document existing test structure
- [ ] Create directory structure in ./tests/
- [ ] Run baseline: `cargo nextest run` to see current state

### Phase 2: Extraction (File-by-file, manual)
For each file with tests:
1. Read the source file carefully
2. Identify the `#[cfg(test)]` module(s)
3. Extract test code with all imports and helper functions
4. Create corresponding `test_*.rs` file in ./tests/
5. Remove `#[cfg(test)]` wrapper (tests/ files don't need it)
6. Update module imports to reference the source code
7. Verify the file compiles: `cargo check`
8. Run tests: `cargo nextest run --test test_filename`
9. Remove test module from source file only after tests pass
10. Document completion

### Phase 3: Verification
- Run full test suite: `cargo nextest run`
- Verify no tests remain in ./src/
- Verify all tests in ./tests/ are discovered by nextest
- Check coverage report

## Directory Structure to Create

```
tests/
├── cli/
│   ├── test_args.rs
│   ├── test_completion.rs
│   ├── test_config.rs
│   ├── test_handler.rs
│   └── test_prompt.rs
├── context/
│   └── extraction/
│       ├── test_error.rs
│       └── test_mod.rs
├── core/
│   ├── generation/
│   │   ├── test_config.rs
│   │   ├── test_stats.rs
│   │   ├── test_tokens.rs
│   │   └── test_types.rs
│   ├── test_model_config.rs
│   ├── test_simd_adapters.rs
│   └── tokenizer/
│       └── test_core.rs
├── domain/
│   ├── chat/
│   │   ├── message/
│   │   │   ├── test_message_processing.rs
│   │   │   └── test_mod.rs
│   │   ├── templates/
│   │   │   └── test_parser.rs
│   │   ├── test_formatting.rs
│   │   ├── test_loop.rs
│   │   └── test_orchestration.rs
│   ├── completion/
│   │   └── test_prompt_formatter.rs
│   ├── context/
│   │   ├── extraction/
│   │   │   └── test_mod.rs
│   │   └── test_provider.rs
│   ├── model/
│   │   └── test_error.rs
│   └── util/
│       ├── test_json_util.rs
│       └── test_notnan.rs
├── memory/
│   ├── core/
│   │   └── test_mod.rs
│   ├── migration/
│   │   └── test_converter.rs
│   ├── monitoring/
│   │   └── test_metrics.rs
│   ├── schema/
│   │   └── test_relationship_schema.rs
│   ├── transaction/
│   │   └── test_mod.rs
│   └── vector/
│       ├── test_vector_index.rs
│       └── test_vector_repository.rs
├── util/
│   ├── test_input_resolver.rs
│   └── test_json_util.rs
├── voice/
│   ├── test_audio.rs
│   ├── test_mod.rs
│   └── test_transcription.rs
├── workflow/
│   └── test_parallel.rs
└── test_lib.rs
```

## Notes & Warnings

### Critical Points
1. **DO NOT AUTOMATE** - Each file requires manual review
2. **DO NOT RUSH** - This will take multiple sessions
3. **VERIFY EACH FILE** - Run tests after each extraction
4. **PRESERVE CONTEXT** - Keep all helper functions and test utilities
5. **CHECK IMPORTS** - Update paths from `super::*` to explicit crate paths

### Common Patterns to Watch For
- Tests using `super::*` imports → Change to `use paraphym_candle::module::*`
- Helper functions within test modules → Keep them in extracted tests
- Test-only public items with `#[cfg(test)]` → May need to be `pub(crate)` instead
- Shared test utilities → Consider `tests/common/mod.rs` for shared code

### Files That May Need Special Attention
- `src/lib.rs` - Integration tests showing API examples
- `src/memory/monitoring/tests/metrics_tests.rs` - Already in tests-like structure
- `src/memory/transaction/tests/transaction_manager_tests.rs` - Same
- Files with both unit tests and integration tests

## Progress Tracking

Total files to extract: 43

Extracted: 0
Remaining: 43
Percentage: 0%

## Next Steps

1. Create all required directories in ./tests/
2. Run baseline `cargo nextest run` to capture current state
3. Start extraction with simplest files first (e.g., util, cli)
4. Work module by module
5. Re-run tests frequently

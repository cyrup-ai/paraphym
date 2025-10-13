# Test Extraction Status

## Baseline Metrics (Before Extraction)
- **Date**: 2025-10-13
- **Total Tests**: 157
- **Passing**: 156
- **Failing**: 1 (util::json_util::tests::stringified_roundtrip)
- **Nextest Version**: 0.9.105

## Current Status (After First Extraction)
- **Total Tests**: 157
- **Passing**: 156
- **Failing**: 1 (same pre-existing failure)
- **Files Extracted**: 1 / 43

## IMPORTANT: Naming Convention Discovery

**Correct Pattern**: Tests use flat structure with underscores, NOT directory mirroring!

Example:
- ✅ CORRECT: `src/domain/util/notnan.rs` → `tests/domain_util_notnan.rs`
- ❌ WRONG: `src/domain/util/notnan.rs` → `tests/domain/util/test_notnan.rs`

This follows the existing pattern seen in files like:
- `tests/domain_voice_audio.rs` (for src/voice/audio.rs)
- `tests/domain_context_extraction_error.rs` (for src/context/extraction/error.rs)

---

## Extraction Progress

### Files to Extract: 43 total

#### Status Legend
- ⬜ Not Started
- 🟡 In Progress  
- ✅ Completed & Verified
- ❌ Failed (needs retry)

---

### CLI Module (5 files)
- ⬜ src/cli/args.rs → tests/cli_args.rs
- ⬜ src/cli/completion.rs → tests/cli_completion.rs
- ⬜ src/cli/config.rs → tests/cli_config.rs
- ⬜ src/cli/handler.rs → tests/cli_handler.rs
- ⬜ src/cli/prompt.rs → tests/cli_prompt.rs

### Context Module (2 files)
- ⬜ src/context/extraction/error.rs → tests/context_extraction_error.rs
- ⬜ src/context/extraction/mod.rs → tests/context_extraction_mod.rs

### Core Module (8 files)
- ⬜ src/core/generation/config.rs → tests/core_generation_config.rs
- ⬜ src/core/generation/stats.rs → tests/core_generation_stats.rs
- ⬜ src/core/generation/tokens.rs → tests/core_generation_tokens.rs
- ⬜ src/core/generation/types.rs → tests/core_generation_types.rs
- ⬜ src/core/model_config.rs → tests/core_model_config.rs
- ⬜ src/core/simd_adapters.rs → tests/core_simd_adapters.rs
- ⬜ src/core/tokenizer/core.rs → tests/core_tokenizer_core.rs

### Domain Module (13 files)
- ⬜ src/domain/chat/formatting.rs → tests/domain_chat_formatting.rs
- ⬜ src/domain/chat/loop.rs → tests/domain_chat_loop.rs
- ⬜ src/domain/chat/message/message_processing.rs → tests/domain_chat_message_message_processing.rs
- ⬜ src/domain/chat/message/mod.rs → tests/domain_chat_message_mod.rs
- ⬜ src/domain/chat/orchestration.rs → tests/domain_chat_orchestration.rs
- ⬜ src/domain/chat/templates/parser.rs → tests/domain_chat_templates_parser.rs
- ⬜ src/domain/completion/prompt_formatter.rs → tests/domain_completion_prompt_formatter.rs
- ⬜ src/domain/context/extraction/mod.rs → tests/domain_context_extraction_mod.rs
- ⬜ src/domain/context/provider.rs → tests/domain_context_provider.rs
- ⬜ src/domain/model/error.rs → tests/domain_model_error.rs
- ⬜ src/domain/util/json_util.rs → tests/domain_util_json_util.rs
- ✅ src/domain/util/notnan.rs → tests/domain_util_notnan.rs (COMPLETED)

### Memory Module (9 files)
- ⬜ src/memory/core/mod.rs → tests/memory_core_mod.rs
- ⬜ src/memory/migration/converter.rs → tests/memory_migration_converter.rs
- ⬜ src/memory/monitoring/metrics.rs → tests/memory_monitoring_metrics.rs
- ⬜ src/memory/monitoring/metrics_test.rs → (special case - already test file)
- ⬜ src/memory/monitoring/mod.rs → tests/memory_monitoring_mod.rs
- ⬜ src/memory/monitoring/tests/metrics_tests.rs → (special case - already test file)
- ⬜ src/memory/schema/relationship_schema.rs → tests/memory_schema_relationship_schema.rs
- ⬜ src/memory/transaction/mod.rs → tests/memory_transaction_mod.rs
- ⬜ src/memory/transaction/tests/transaction_manager_tests.rs → (special case - already test file)
- ⬜ src/memory/vector/vector_index.rs → tests/memory_vector_vector_index.rs
- ⬜ src/memory/vector/vector_repository.rs → tests/memory_vector_vector_repository.rs

### Util Module (2 files)
- ⬜ src/util/input_resolver.rs → tests/util_input_resolver.rs
- ⬜ src/util/json_util.rs → tests/util_json_util.rs

### Voice Module (3 files)
- ⬜ src/voice/audio.rs → tests/voice_audio.rs
- ⬜ src/voice/mod.rs → tests/voice_mod.rs
- ⬜ src/voice/transcription.rs → tests/voice_transcription.rs

### Workflow Module (1 file)
- ⬜ src/workflow/parallel.rs → tests/workflow_parallel.rs

### Root (1 file)
- ⬜ src/lib.rs → tests/lib.rs

---

## Progress Summary
- Extracted: 1 / 43
- Percentage: 2.3%
- Current Status: IN PROGRESS

## Completed Extractions

### 1. domain_util_notnan.rs ✅
- **Source**: src/domain/util/notnan.rs
- **Target**: tests/domain_util_notnan.rs
- **Tests**: 3 (test_notnan_creation, test_notnan_ordering, test_notnan_into_inner)
- **Status**: All tests passing
- **Date**: 2025-10-13

## Next Files to Extract

Recommended order (simple to complex):
1. src/cli/args.rs - CLI argument parsing tests
2. src/cli/config.rs - CLI configuration tests
3. src/cli/prompt.rs - CLI prompt tests
4. src/domain/chat/loop.rs - Simple enum display test
5. src/core/generation/types.rs - Constants validation

## Notes
- Naming convention corrected: use underscores, not directories
- All test files go in tests/ root level
- Remove #[cfg(test)] wrapper when extracting
- Change `use super::*` to explicit crate imports
- Verify each file before marking complete

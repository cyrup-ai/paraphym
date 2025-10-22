# STELLA_1: Eliminate Massive Code Duplication — Outstanding Items

## Objective

Finish the refactor by restoring `base.rs` functionality while keeping all shared logic in `utils.rs` and preserving behavior.

## Outstanding Work

- **Restore `TextEmbeddingCapable` for `StellaEmbeddingModel`** in `packages/candle/src/capability/text_embedding/stella/base.rs`:
  - **Implement** `embed()` and `batch_embed()` using the shared utilities:
    - `detect_device_and_dtype()`
    - `configure_stella_tokenizer(...)`
    - `create_stella_config(...)`
    - `load_stella_weights(...)`
  - **Use model info** from `self.info()` to derive `max_length`, `embedding_dimension`, and detect variant via `detect_variant(self.info().registry_key)`.
  - **Load files** via `self.huggingface_file(...)` for:
    - `model.safetensors`
    - `2_Dense_{dimension}/model.safetensors`
    - `tokenizer.json`
  - **Preserve inference behavior** exactly as before refactor:
    - Format inputs with `format_with_instruction(...)`.
    - Tokenize, create tensors, call `EmbeddingModel::forward_norm(...)`.
    - Extract the first embedding as `Vec<f32>` (single) or `Vec<Vec<f32>>` (batch).
  - **Error handling**: propagate errors; do not use `unwrap()` or `expect()`.

## Acceptance Criteria

- **`base.rs`** compiles with `embed()` and `batch_embed()` implemented and calling `utils.rs` functions exclusively for device/dtype detection, tokenizer config, config creation, and weight loading.
- **No functional behavior change** vs. pre-refactor logic; this is strictly a duplication removal refactor.
- **No `unwrap()`/`expect()`** in `base.rs`.
- **No duplicated tokenizer or weight-loading logic** remains in `base.rs` or `loaded.rs`.
- **Build passes**: `cargo check --package cyrup_candle`.

Optional (quality): remove unused imports in `loaded.rs` once the above is complete.

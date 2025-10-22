# Task: Text Embedding Worker — Build Verification Only

## Objective
- Verify that the simplified state ownership changes in the text embedding worker compile successfully with no functional regressions.

## Instructions
- From the repository root, run:
  - `cargo check -p candle`

## Definition of Done
- The command `cargo check -p candle` completes successfully (exit code 0) for the workspace, confirming the changes in:
  - `./packages/candle/src/capability/registry/pool/capabilities/text_embedding.rs`

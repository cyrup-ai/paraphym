# `packages/simd/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: simd
- **File Hash**: 0c5e0d9c  
- **Timestamp**: 2025-10-10T02:15:58.228903+00:00  
- **Lines of Code**: 127

---## Unused Dependencies
### `ahash`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
ahash = "0.8"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies]
- ahash = "0.8"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove ahash --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `arrayvec`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
arrayvec = "0.7"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies]
- arrayvec = "0.7"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove arrayvec --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `bindgen_cuda`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.bindgen_cuda]
optional = true
version = "0.1.5"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.bindgen_cuda]
- optional = true
- version = "0.1.5"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove bindgen_cuda --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-core`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.candle-core]
branch = "main"
default-features = false
git = "https://github.com/huggingface/candle"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.candle-core]
- branch = "main"
- default-features = false
- git = "https://github.com/huggingface/candle"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-core --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-kernels`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.candle-kernels]
branch = "main"
git = "https://github.com/huggingface/candle"
optional = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.candle-kernels]
- branch = "main"
- git = "https://github.com/huggingface/candle"
- optional = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-kernels --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-metal-kernels`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.candle-metal-kernels]
branch = "main"
git = "https://github.com/huggingface/candle"
optional = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.candle-metal-kernels]
- branch = "main"
- git = "https://github.com/huggingface/candle"
- optional = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-metal-kernels --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-nn`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.candle-nn]
branch = "main"
default-features = false
git = "https://github.com/huggingface/candle"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.candle-nn]
- branch = "main"
- default-features = false
- git = "https://github.com/huggingface/candle"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-nn --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-transformers`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.candle-transformers]
branch = "main"
default-features = false
git = "https://github.com/huggingface/candle"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.candle-transformers]
- branch = "main"
- default-features = false
- git = "https://github.com/huggingface/candle"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-transformers --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `crossbeam-utils`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
crossbeam-utils = "0.8"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies]
- crossbeam-utils = "0.8"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove crossbeam-utils --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `cudarc`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.cudarc]
default-features = false
features = ["cublas", "cublaslt", "cudnn", "curand", "dynamic-linking", "f16", "std"]
optional = true
version = "0.17.3"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.cudarc]
- default-features = false
- features = ["cublas", "cublaslt", "cudnn", "curand", "dynamic-linking", "f16", "std"]
- optional = true
- version = "0.17.3"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove cudarc --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `dashmap`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
dashmap = "6.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies]
- dashmap = "6.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove dashmap --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `half`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.half]
optional = true
version = "2.6.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.half]
- optional = true
- version = "2.6.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove half --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `libm`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
libm = "0.2"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies]
- libm = "0.2"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove libm --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `metal`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.metal]
optional = true
version = "0.32.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.metal]
- optional = true
- version = "0.32.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove metal --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `parking_lot`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
parking_lot = "0.12"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies]
- parking_lot = "0.12"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove parking_lot --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rayon`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.rayon]
optional = true
version = "1.11.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dependencies.rayon]
- optional = true
- version = "1.11.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rayon --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `criterion`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
criterion = "0.7"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dev-dependencies]
- criterion = "0.7"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove criterion --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.env_logger]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
- [dev-dependencies.env_logger]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/packages/simd/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
# `packages/candle/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: candle
- **File Hash**: a8e3f7ca  
- **Timestamp**: 2025-10-10T02:15:58.177071+00:00  
- **Lines of Code**: 259

---## Unused Dependencies
### `async-channel`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.async-channel]
version = "2.5.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.async-channel]
- version = "2.5.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove async-channel --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `async-stream`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
async-stream = "0.3.6"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- async-stream = "0.3.6"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove async-stream --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `bindgen_cuda`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.bindgen_cuda]
optional = true
version = "0.1.5"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.bindgen_cuda]
- optional = true
- version = "0.1.5"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove bindgen_cuda --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `cache-padded`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
cache-padded = "1.3.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- cache-padded = "1.3.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove cache-padded --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-kernels`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
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
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.candle-kernels]
- branch = "main"
- git = "https://github.com/huggingface/candle"
- optional = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-kernels --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `candle-metal-kernels`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
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
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.candle-metal-kernels]
- branch = "main"
- git = "https://github.com/huggingface/candle"
- optional = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove candle-metal-kernels --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `clap`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.clap]
features = ["derive"]
version = "4.5.48"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.clap]
- features = ["derive"]
- version = "4.5.48"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove clap --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `console_error_panic_hook`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.console_error_panic_hook]
optional = true
version = "0.1.7"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.console_error_panic_hook]
- optional = true
- version = "0.1.7"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove console_error_panic_hook --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `crossbeam-deque`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
crossbeam-deque = "0.8.6"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- crossbeam-deque = "0.8.6"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove crossbeam-deque --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `cudarc`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
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
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.cudarc]
- default-features = false
- features = ["cublas", "cublaslt", "cudnn", "curand", "dynamic-linking", "f16", "std"]
- optional = true
- version = "0.17.3"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove cudarc --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `faiss`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.faiss]
optional = true
version = "0.12.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.faiss]
- optional = true
- version = "0.12.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove faiss --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `fastrand`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
fastrand = "2.3.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- fastrand = "2.3.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove fastrand --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `flume`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
flume = "0.11.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- flume = "0.11.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove flume --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `futures`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.futures]
version = "0.3.31"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.futures]
- version = "0.3.31"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove futures --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `gix`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
gix = "0.73.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- gix = "0.73.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove gix --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `globset`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.globset]
optional = true
version = "0.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.globset]
- optional = true
- version = "0.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove globset --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `half`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.half]
optional = true
version = "2.6.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.half]
- optional = true
- version = "2.6.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove half --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `hound`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
hound = "3.5.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- hound = "3.5.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove hound --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `ignore`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
ignore = "0.4.23"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- ignore = "0.4.23"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove ignore --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `js-sys`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.js-sys]
optional = true
version = "0.3.81"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.js-sys]
- optional = true
- version = "0.3.81"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove js-sys --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `jwalk`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
jwalk = "0.8.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- jwalk = "0.8.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove jwalk --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `lazy_static`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
lazy_static = "1.5.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- lazy_static = "1.5.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove lazy_static --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `md5`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
md5 = "0.8.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- md5 = "0.8.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove md5 --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `memchr`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.memchr]
version = "2.7.6"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.memchr]
- version = "2.7.6"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove memchr --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `memmap2`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
memmap2 = "0.9.8"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- memmap2 = "0.9.8"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove memmap2 --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `metal`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.metal]
optional = true
version = "0.32.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.metal]
- optional = true
- version = "0.32.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove metal --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `nalgebra`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.nalgebra]
features = ["serde-serialize"]
version = "0.34"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.nalgebra]
- features = ["serde-serialize"]
- version = "0.34"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove nalgebra --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `ndarray`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.ndarray]
version = "0.16.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.ndarray]
- version = "0.16.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove ndarray --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `num-complex`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.num-complex]
version = "0.4.6"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.num-complex]
- version = "0.4.6"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove num-complex --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `ordered-float`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.ordered-float]
version = "5.1.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.ordered-float]
- version = "5.1.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove ordered-float --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `petgraph`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.petgraph]
version = "0.8.3"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.petgraph]
- version = "0.8.3"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove petgraph --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rand`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
rand = "0.9.2"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- rand = "0.9.2"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rand --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rand_pcg`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
rand_pcg = "0.9.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- rand_pcg = "0.9.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rand_pcg --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rayon`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.rayon]
optional = true
version = "1.11.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.rayon]
- optional = true
- version = "1.11.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rayon --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `ropey`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
ropey = "2.0.0-beta.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- ropey = "2.0.0-beta.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove ropey --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `serde_cbor`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
serde_cbor = "0.11.2"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- serde_cbor = "0.11.2"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove serde_cbor --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `shell-words`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
shell-words = "1.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- shell-words = "1.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove shell-words --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `strsim`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
strsim = "0.11.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- strsim = "0.11.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove strsim --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `sweetmcp-json-client`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.sweetmcp-json-client]
path = "../sweetmcp/packages/json-client"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.sweetmcp-json-client]
- path = "../sweetmcp/packages/json-client"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove sweetmcp-json-client --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `sweetmcp-stdio-client`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.sweetmcp-stdio-client]
path = "../sweetmcp/packages/stdio-client"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.sweetmcp-stdio-client]
- path = "../sweetmcp/packages/stdio-client"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove sweetmcp-stdio-client --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `symphonia`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.symphonia]
features = ["all"]
optional = true
version = "0.5.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.symphonia]
- features = ["all"]
- optional = true
- version = "0.5.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove symphonia --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tower`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.tower]
optional = true
version = "0.5.2"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.tower]
- optional = true
- version = "0.5.2"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tower --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `value-trait`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
value-trait = "0.11.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- value-trait = "0.11.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove value-trait --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `walkdir`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.walkdir]
version = "2.5.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.walkdir]
- version = "2.5.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove walkdir --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `wasm-bindgen`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.wasm-bindgen]
optional = true
version = "0.2.104"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.wasm-bindgen]
- optional = true
- version = "0.2.104"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove wasm-bindgen --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `web-sys`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.web-sys]
optional = true
version = "0.3.81"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies.web-sys]
- optional = true
- version = "0.3.81"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove web-sys --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `wide`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
wide = "0.7.33"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dependencies]
- wide = "0.7.33"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove wide --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `assert_approx_eq`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.assert_approx_eq]
version = "1.1.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies.assert_approx_eq]
- version = "1.1.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove assert_approx_eq --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `criterion`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.criterion]
version = "0.7.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies.criterion]
- version = "0.7.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove criterion --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.env_logger]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies.env_logger]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `fnv`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
fnv = "1.0.7"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies]
- fnv = "1.0.7"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove fnv --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `hdrhistogram`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
hdrhistogram = "7.5.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies]
- hdrhistogram = "7.5.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove hdrhistogram --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `jemalloc-sys`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
jemalloc-sys = "0.5.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies]
- jemalloc-sys = "0.5.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove jemalloc-sys --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `mockall`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.mockall]
version = "0.13.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies.mockall]
- version = "0.13.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove mockall --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `mockito`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.mockito]
version = "1.7.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies.mockito]
- version = "1.7.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove mockito --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `quanta`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
quanta = "0.12.6"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies]
- quanta = "0.12.6"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove quanta --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tempfile`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.tempfile]
version = "3.23.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies.tempfile]
- version = "3.23.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tempfile --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `weak-table`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
weak-table = "0.3.2"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
- [dev-dependencies]
- weak-table = "0.3.2"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove weak-table --manifest-path /Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
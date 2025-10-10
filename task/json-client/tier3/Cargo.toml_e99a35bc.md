# `packages/sweetmcp/packages/json-client/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: json-client
- **File Hash**: e99a35bc  
- **Timestamp**: 2025-10-10T02:15:59.703253+00:00  
- **Lines of Code**: 34

---## Unused Dependencies
### `simd-json`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.simd-json]
default-features = false
features = ["known-key", "runtime-detection", "swar-number-parsing", "value-no-dup-keys"]
version = "0.16.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
- [dependencies.simd-json]
- default-features = false
- features = ["known-key", "runtime-detection", "swar-number-parsing", "value-no-dup-keys"]
- version = "0.16.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove simd-json --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `thiserror`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
thiserror = "2.0.17"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
- [dependencies]
- thiserror = "2.0.17"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove thiserror --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tokio`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.tokio]
features = ["full"]
version = "1.47.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
- [dependencies.tokio]
- features = ["full"]
- version = "1.47.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tokio --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `pretty_assertions`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
pretty_assertions = "1.4.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
- [dev-dependencies]
- pretty_assertions = "1.4.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove pretty_assertions --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tokio-test`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
tokio-test = "0.4.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
- [dev-dependencies]
- tokio-test = "0.4.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tokio-test --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/json-client/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
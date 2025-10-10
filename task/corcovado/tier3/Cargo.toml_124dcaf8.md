# `packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: corcovado
- **File Hash**: 124dcaf8  
- **Timestamp**: 2025-10-10T02:15:58.464383+00:00  
- **Lines of Code**: 53

---## Unused Dependencies
### `cfg-if`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
cfg-if = "0.1.9"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
- [dependencies]
- cfg-if = "0.1.9"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove cfg-if --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tracing`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.tracing]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
- [dependencies.tracing]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tracing --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `bytes`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
bytes = "0.3.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
- [dev-dependencies]
- bytes = "0.3.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove bytes --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `criterion`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.criterion]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
- [dev-dependencies.criterion]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove criterion --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.env_logger]
default-features = false
version = "0.4.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
- [dev-dependencies.env_logger]
- default-features = false
- version = "0.4.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tempdir`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
tempdir = "0.3.7"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
- [dev-dependencies]
- tempdir = "0.3.7"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tempdir --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/vendor/rio/corcovado/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
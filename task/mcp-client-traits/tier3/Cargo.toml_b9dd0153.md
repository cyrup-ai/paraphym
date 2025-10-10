# `packages/sweetmcp/packages/mcp-client-traits/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: mcp-client-traits
- **File Hash**: b9dd0153  
- **Timestamp**: 2025-10-10T02:15:58.343664+00:00  
- **Lines of Code**: 33

---## Unused Dependencies
### `log`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.log]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
- [dependencies.log]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove log --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `thiserror`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
thiserror = "2.0.17"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
- [dependencies]
- thiserror = "2.0.17"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove thiserror --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `uuid`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.uuid]
features = ["v4"]
version = "1.18.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
- [dependencies.uuid]
- features = ["v4"]
- version = "1.18.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove uuid --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `pretty_assertions`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
pretty_assertions = "1.4.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
- [dev-dependencies]
- pretty_assertions = "1.4.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove pretty_assertions --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tokio-test`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
tokio-test = "0.4.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
- [dev-dependencies]
- tokio-test = "0.4.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tokio-test --manifest-path /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/mcp-client-traits/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
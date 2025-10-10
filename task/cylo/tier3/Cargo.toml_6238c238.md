# `packages/cylo/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: cylo
- **File Hash**: 6238c238  
- **Timestamp**: 2025-10-10T02:15:57.757653+00:00  
- **Lines of Code**: 60

---## Unused Dependencies
### `cyrup_sugars`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.cyrup_sugars]
branch = "main"
features = ["all"]
git = "https://github.com/cyrup-ai/cyrup-sugars"
package = "cyrup_sugars"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies.cyrup_sugars]
- branch = "main"
- features = ["all"]
- git = "https://github.com/cyrup-ai/cyrup-sugars"
- package = "cyrup_sugars"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove cyrup_sugars --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.env_logger]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies.env_logger]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `hyper-util`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.hyper-util]
features = ["client-legacy", "tokio"]
version = "0.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies.hyper-util]
- features = ["client-legacy", "tokio"]
- version = "0.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove hyper-util --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `statig`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
statig = "0.4.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies]
- statig = "0.4.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove statig --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `termcolor`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.termcolor]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies.termcolor]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove termcolor --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `uuid`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.uuid]
features = ["v4", "serde"]
version = "1.18.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies.uuid]
- features = ["v4", "serde"]
- version = "1.18.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove uuid --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `watchexec-signals`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
watchexec-signals = "5.0.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dependencies]
- watchexec-signals = "5.0.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove watchexec-signals --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `assert_cmd`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
assert_cmd = "2.0.17"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dev-dependencies]
- assert_cmd = "2.0.17"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove assert_cmd --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `assert_fs`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
assert_fs = "1.1.3"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dev-dependencies]
- assert_fs = "1.1.3"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove assert_fs --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.env_logger]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dev-dependencies.env_logger]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `predicates`

- **Declared in**: `/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
predicates = "3.1.3"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
- [dev-dependencies]
- predicates = "3.1.3"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove predicates --manifest-path /Volumes/samsung_t9/paraphym/packages/cylo/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
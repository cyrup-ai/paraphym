# `forks/surrealdb/crates/core/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: core
- **File Hash**: fefbd7ca  
- **Timestamp**: 2025-10-10T02:16:00.744189+00:00  
- **Lines of Code**: 232

---## Unused Dependencies
### `async-executor`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.async-executor]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dependencies.async-executor]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove async-executor --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `md-5`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.md-5]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dependencies.md-5]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove md-5 --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `criterion`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.criterion]
features = ["async_tokio"]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.criterion]
- features = ["async_tokio"]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove criterion --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.env_logger]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.env_logger]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `flate2`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.flate2]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.flate2]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove flate2 --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `pprof`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.pprof]
features = ["flamegraph", "criterion"]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.pprof]
- features = ["flamegraph", "criterion"]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove pprof --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rstest`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.rstest]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.rstest]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rstest --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `serial_test`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.serial_test]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.serial_test]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove serial_test --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `temp-dir`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.temp-dir]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.temp-dir]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove temp-dir --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `test-log`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.test-log]
features = ["trace"]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.test-log]
- features = ["trace"]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove test-log --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `time`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.time]
features = ["serde"]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.time]
- features = ["serde"]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove time --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tokio`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.tokio]
features = ["macros", "sync", "rt-multi-thread"]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.tokio]
- features = ["macros", "sync", "rt-multi-thread"]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tokio --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tracing-subscriber`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.tracing-subscriber]
features = ["env-filter"]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.tracing-subscriber]
- features = ["env-filter"]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tracing-subscriber --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `wiremock`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.wiremock]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
- [dev-dependencies.wiremock]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove wiremock --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
# `forks/surrealdb/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: surrealdb
- **File Hash**: 23846b8c  
- **Timestamp**: 2025-10-10T02:16:01.070962+00:00  
- **Lines of Code**: 431

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 431 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Unused Dependencies
### `argon2`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.argon2]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.argon2]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove argon2 --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `base64`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.base64]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.base64]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove base64 --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `chrono`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.chrono]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.chrono]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove chrono --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `geo`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.geo]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.geo]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove geo --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `geo-types`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.geo-types]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.geo-types]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove geo-types --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `http-body`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
http-body = "1.0.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies]
- http-body = "1.0.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove http-body --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `http-body-util`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies]
http-body-util = "0.1.1"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies]
- http-body-util = "0.1.1"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove http-body-util --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `num_cpus`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.num_cpus]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.num_cpus]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove num_cpus --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rand`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.rand]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.rand]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rand --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rust_decimal`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.rust_decimal]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.rust_decimal]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rust_decimal --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tokio-tungstenite`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.tokio-tungstenite]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dependencies.tokio-tungstenite]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tokio-tungstenite --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `env_logger`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.env_logger]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dev-dependencies.env_logger]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove env_logger --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `opentelemetry-proto`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.opentelemetry-proto]
features = ["gen-tonic", "metrics", "logs"]
version = "0.7.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dev-dependencies.opentelemetry-proto]
- features = ["gen-tonic", "metrics", "logs"]
- version = "0.7.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove opentelemetry-proto --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `rstest`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.rstest]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dev-dependencies.rstest]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove rstest --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `serial_test`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.serial_test]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dev-dependencies.serial_test]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove serial_test --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tonic`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
tonic = "0.12.3"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dev-dependencies]
- tonic = "0.12.3"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tonic --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `wiremock`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.wiremock]
workspace = true
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
- [dev-dependencies.wiremock]
- workspace = true
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove wiremock --manifest-path /Volumes/samsung_t9/paraphym/forks/surrealdb/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
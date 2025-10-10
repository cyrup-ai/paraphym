# `forks/pingora/pingora-proxy/Cargo.toml`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: pingora-proxy
- **File Hash**: a46322ee  
- **Timestamp**: 2025-10-10T02:16:01.369854+00:00  
- **Lines of Code**: 64

---## Unused Dependencies
### `clap`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dependencies.clap]
features = ["derive"]
version = "3.2.25"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dependencies.clap]
- features = ["derive"]
- version = "3.2.25"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove clap --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `pingora-limits`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.pingora-limits]
path = "../pingora-limits"
version = "0.6.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies.pingora-limits]
- path = "../pingora-limits"
- version = "0.6.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove pingora-limits --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `pingora-load-balancing`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.pingora-load-balancing]
default-features = false
path = "../pingora-load-balancing"
version = "0.6.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies.pingora-load-balancing]
- default-features = false
- path = "../pingora-load-balancing"
- version = "0.6.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove pingora-load-balancing --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `prometheus`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
prometheus = "0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies]
- prometheus = "0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove prometheus --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `serde`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies.serde]
features = ["derive"]
version = "1.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies.serde]
- features = ["derive"]
- version = "1.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove serde --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `serde_json`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
serde_json = "1.0"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies]
- serde_json = "1.0"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove serde_json --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `serde_yaml`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
serde_yaml = "0.8"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies]
- serde_yaml = "0.8"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove serde_yaml --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions
### `tokio-test`

- **Declared in**: `/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml` ([dev-dependencies])
- **Issue**: Dependency is declared but no imports found in any `.rs` files

- **Cargo.toml snippet**:
```toml
[dev-dependencies]
tokio-test = "0.4"
```

- **Suggested removal (unified diff)**:
```diff
--- a/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
+++ b/Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
- [dev-dependencies]
- tokio-test = "0.4"
```

- **Optional: cargo-edit command**:
```bash
# If you have cargo-edit installed: cargo install cargo-edit
cargo remove tokio-test --manifest-path /Volumes/samsung_t9/paraphym/forks/pingora/pingora-proxy/Cargo.toml
```

**Action Required**:
- Unused dependencies should be removed from the `Cargo.toml` file
- Update this section with the specific remediation instructions---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
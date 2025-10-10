# `packages/sweetmcp/packages/daemon/src/installer/uninstall.rs`

- **Path**: /Volumes/samsung_t9/paraphym
- **Project**: daemon
- **File Hash**: cc16d4b4  
- **Timestamp**: 2025-10-10T02:15:59.689436+00:00  
- **Lines of Code**: 346

---## ⚠️ PRIORITY: Decompose into Submodules

**File**: ``  
**Lines of Code**: 346 (threshold: 300)

This file MUST be decomposed before addressing any other violations.

- Decompose the code into logical separation of concerns with no single module >= 300 lines of code. 
- Ensure all the sum of parts exactly equals the original with ONLY production quality source code
- Ensure the original "shadowing" module is deleted so the new decomposed submodule is ACTUALLY USED
- Ensure there are absolutely no backup files left in the codebase polluting up the code with the original monolithic file.

---## Orphaned Methods


### `backup_configuration()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/installer/uninstall.rs` (line 355)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Backup configuration before uninstall (API function for future CLI use)
#[allow(dead_code)]
pub fn backup_configuration() -> Result<PathBuf> {
    let config_dir = get_config_directory();
    let backup_dir = get_backup_directory();
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `import_wildcard_certificate_linux()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/installer/uninstall.rs` (line 91)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Import wildcard certificate on Linux
#[cfg(target_os = "linux")]
fn import_wildcard_certificate_linux(cert_path: &str) -> Result<()> {
    info!("Importing SweetMCP wildcard certificate to Linux system trust store");

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `remove_wildcard_certificate_linux()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/installer/uninstall.rs` (line 178)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Remove wildcard certificate from Linux system trust store
#[cfg(target_os = "linux")]
async fn remove_wildcard_certificate_linux() -> Result<()> {
    info!("Removing SweetMCP wildcard certificate from Linux system trust store");

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `remove_wildcard_certificate_macos()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/installer/uninstall.rs` (line 148)
- **Visibility**: private
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Remove wildcard certificate from macOS keychain
#[cfg(target_os = "macos")]
async fn remove_wildcard_certificate_macos() -> Result<()> {
    info!("Removing SweetMCP wildcard certificate from macOS System keychain");

```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.


### `restore_configuration()`

- **Location**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/daemon/src/installer/uninstall.rs` (line 439)
- **Visibility**: pub
- **Issue**: Function is defined but never called anywhere in the codebase

```rust
/// Restore configuration from backup (API function for future CLI use)
#[allow(dead_code)]
pub fn restore_configuration(backup_path: &Path) -> Result<()> {
    if !backup_path.exists() {
        return Err(anyhow::anyhow!("Backup file not found: {:?}", backup_path));
```

### Action Required:

- Evaluate the intended purpose of the orphaned method, assuming it is intended to be used by default.
- If it should be used, update this section with instructions on how to incorporate it into the codebase.
- If it is deprecated, ask for permission to remove it.
- Update this section with your findings and instructions on how to proceed.

---

*Generated by kargo-turd 0.1.0*

/Volumes/samsung_t9/paraphym
# ASYNC_FS: Convert std::fs to tokio::fs

## OBJECTIVE
Replace all synchronous std::fs operations with tokio::fs async operations.
This completes the 100% tokio async migration for file I/O.

## CONVERSION PATTERNS

### Pattern 1: read_to_string
```rust
// BEFORE:
use std::fs;
let content = std::fs::read_to_string(path)?;

// AFTER:
use tokio::fs;
let content = tokio::fs::read_to_string(path).await?;
```

### Pattern 2: read (binary)
```rust
// BEFORE:
let bytes = std::fs::read(path)?;

// AFTER:
let bytes = tokio::fs::read(path).await?;
```

### Pattern 3: File::open
```rust
// BEFORE:
use std::fs::File;
let mut file = std::fs::File::open(path)?;

// AFTER:
use tokio::fs::File;
let mut file = tokio::fs::File::open(path).await?;
```

### Pattern 4: metadata
```rust
// BEFORE:
let metadata = std::fs::metadata(path)?;

// AFTER:
let metadata = tokio::fs::metadata(path).await?;
```

### Pattern 5: read_dir
```rust
// BEFORE:
for entry in std::fs::read_dir(path)? {
    let entry = entry?;
    // process entry
}

// AFTER:
let mut entries = tokio::fs::read_dir(path).await?;
while let Some(entry) = entries.next_entry().await? {
    // process entry
}
```

### Pattern 6: create_dir_all
```rust
// BEFORE:
std::fs::create_dir_all(path)?;

// AFTER:
tokio::fs::create_dir_all(path).await?;
```

### Pattern 7: write
```rust
// BEFORE:
std::fs::write(path, data)?;

// AFTER:
tokio::fs::write(path, data).await?;
```

## FILES TO CONVERT (38 instances across 20+ files)

### High Priority Files (Most Usage):

1. **src/domain/context/provider.rs** (8 instances)
   - Lines: 665, 763, 773, 777, 1028, 1034, 1193, 1207, 1405
   - Heavy file operations for context loading

2. **src/capability/text_embedding/gte_qwen.rs** (6 instances)
   - Lines: 253, 261, 381, 389, 537, 545
   - Config and index file loading

3. **src/capability/text_embedding/bert.rs** (3 instances)
   - Lines: 293, 395, 545
   - Model configuration loading

4. **src/core/generation/models.rs** (3 instances)
   - Lines: 230, 326, 428
   - Model file operations

5. **src/capability/text_embedding/nvembed.rs** (2 instances)
   - Lines: 289, 543
   - Index file operations

6. **src/domain/chat/config.rs** (2 instances)
   - Lines: 1175, 1224
   - Config read/write operations

### Medium Priority Files:

7. **src/builders/document.rs** (uses std::fs)
   - Document building operations

8. **src/cli/config.rs** (uses std::fs)
   - CLI configuration

9. **src/cli/handler.rs** (uses std::fs)
   - CLI file handling

10. **src/memory/migration/importer.rs** (uses std::fs::File)
    - Data import operations

11. **src/memory/migration/exporter.rs** (uses std::fs::File, metadata)
    - Data export operations

12. **src/capability/text_to_image/flux_schnell.rs** (Line 427)
    - Config loading

13. **src/capability/vision/llava.rs** (Line 181)
    - Vision model config

14. **src/capability/text_to_image/stable_diffusion_35_turbo/mod.rs** (Line 652)
    - Model config

15. **src/domain/context/traits.rs** (Line 347)
    - Context loading

16. **src/domain/init/globals.rs** (Line 269)
    - Global config loading

17. **src/capability/text_to_text/qwen3_coder.rs** (Line 171)
    - GGUF file operations

18. **src/memory/api/middleware.rs** (Line 131)
    - API middleware file ops

19. **src/builders/agent_role.rs** (Line 1336)
    - Directory creation

20. **src/pool/core/memory_governor.rs** (Line 537)
    - System file reading

## FUNCTION SIGNATURE UPDATES

When converting to tokio::fs, functions must become async:

```rust
// BEFORE:
fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    // parse content
}

// AFTER:
async fn load_config(path: &Path) -> Result<Config> {
    let content = tokio::fs::read_to_string(path).await?;
    // parse content
}
```

## IMPORT UPDATES

```rust
// REMOVE:
use std::fs;
use std::fs::File;

// ADD:
use tokio::fs;
use tokio::fs::File;
```

## VERIFICATION

```bash
# Should return 0 (except test files)
cd packages/candle && rg "std::fs::" --type rust src/ | wc -l

# Should have many tokio::fs usages
cd packages/candle && rg "tokio::fs::" --type rust src/ | wc -l

# Should compile
cargo check --package paraphym_candle
```

## DEFINITION OF DONE
- All std::fs operations converted to tokio::fs
- All affected functions are async
- Proper .await calls on all file operations
- Code compiles successfully
- 100% async file I/O

# YSTREAM_U: Remove ystream Dependency

## OBJECTIVE
Remove ystream from Cargo.toml after ALL files are converted.
Remove crossbeam, crossbeam_queue, crossbeam_utils from Cargo.toml.

## VERIFICATION BEFORE REMOVAL

### Step 1: Verify Zero Usage
```bash
cd /Volumes/samsung_t9/paraphym
grep -r "use ystream" packages/candle/src --include="*.rs"
# Expected: No results

grep -r "AsyncStream::" packages/candle/src --include="*.rs" | grep -v "async_stream::"
# Expected: No results
```

### Step 2: Remove Dependency
In `packages/candle/Cargo.toml`:
```toml
# Remove this line:
ystream = { git = "https://github.com/cyrup-ai/ystream", branch = "main", package = "ystream" }
```

### Step 3: Verify Compilation
```bash
cargo check --package paraphym_candle
```

## DEFINITION OF DONE
- Zero ystream imports in codebase
- ystream removed from Cargo.toml
- Compiles with 0 errors
- All streaming uses tokio_stream

# STUB_1: Remove Stub Comments in Globals Module

## OBJECTIVE
Remove misleading "stub" comments and replace with proper documentation explaining the implementation patterns in the globals initialization module.

## PRIORITY
🔴 CRITICAL - Code quality and clarity

## BACKGROUND
The `init/globals.rs` file contains comments labeled "stub" on lines 19 and 53, suggesting the implementation is temporary or incomplete. This creates confusion about whether the code is production-ready. The implementation appears correct but needs better documentation.

## SUBTASK 1: Document Module Restructuring Pattern
**File:** `packages/candle/src/init/globals.rs`  
**Line:** 19

**Current code:**
```rust
stub
use crate::memory::manager::{MemoryConfig, SurrealDBMemoryManager};
```

**Required replacement:**
```rust
/// Uses memory::manager path due to module restructuring.
/// This resolves circular dependency between init and memory modules.
use crate::memory::manager::{MemoryConfig, SurrealDBMemoryManager};
```

**Rationale:**
- Explains WHY this import path is used
- Documents the circular dependency resolution
- Removes confusing "stub" label

## SUBTASK 2: Document Default Config Pattern
**File:** `packages/candle/src/init/globals.rs`  
**Line:** 53

**Current code:**
```rust
stub
fn create_default_config() -> MemoryConfig {
    MemoryConfig::default()
}
```

**Required replacement:**
```rust
/// Creates default memory configuration for lazy initialization.
/// 
/// Used by CONFIG_CACHE static for zero-allocation access patterns.
/// This wrapper function ensures consistent initialization across
/// the application lifecycle.
fn create_default_config() -> MemoryConfig {
    MemoryConfig::default()
}
```

**Rationale:**
- Explains purpose of wrapper function
- Documents connection to CONFIG_CACHE
- Clarifies zero-allocation pattern
- Removes "stub" suggestion

## SUBTASK 3: Verify No Other Stub References
**Action:** Search entire file for any other "stub" references

**Command:**
```bash
grep -ni "stub" packages/candle/src/init/globals.rs
```

**Requirement:**
- Remove ALL "stub" references
- Replace with proper documentation

## DEFINITION OF DONE
- [ ] Line 19 "stub" comment replaced with proper doc comment
- [ ] Line 53 "stub" comment replaced with proper doc comment
- [ ] No remaining "stub" references in file
- [ ] Documentation explains WHY, not just WHAT
- [ ] Code compiles without warnings

## CONSTRAINTS
- ❌ DO NOT write unit tests
- ❌ DO NOT write integration tests
- ❌ DO NOT write benchmarks
- ✅ Focus solely on ./src modifications
- ✅ Do NOT change any actual code, only comments/documentation

## TECHNICAL NOTES
- The actual implementation is correct and production-ready
- Only documentation clarity needs improvement
- This is about removing confusion, not changing behavior
- Consider this a documentation-only task

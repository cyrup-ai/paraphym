# LEGACY_E: UnifiedToolExecutor Migration

## OBJECTIVE
Migrate all 18 usages of deprecated UnifiedToolExecutor to SweetMcpRouter. Remove the legacy tool interface entirely.

## BACKGROUND
**File:** `packages/candle/src/domain/tool/mod.rs:20`
- Comment explicitly says: "Legacy unified tool interface - DEPRECATED in favor of SweetMcpRouter"
- 18 references found in codebase
- New system (SweetMcpRouter) exists and is preferred

## SUBTASK 1: Find all UnifiedToolExecutor usages

```bash
grep -rn "UnifiedToolExecutor" packages/candle/src --include="*.rs"
```

Expected: 18 results

## SUBTASK 2: Understand the migration pattern

**OLD API:**
```rust
use crate::domain::tool::UnifiedToolExecutor;
let executor = UnifiedToolExecutor::new(...);
let result = executor.execute(...).await;
```

**NEW API:**
```rust
use crate::domain::tool::SweetMcpRouter;
let router = SweetMcpRouter::new(...);
let result = router.call_tool(...).await;
```

Check both APIs to understand parameter differences and adjust accordingly.

## SUBTASK 3: Migrate each usage

For each of the 18 usages:
1. Update import: `UnifiedToolExecutor` → `SweetMcpRouter`
2. Update variable names: `executor` → `router`
3. Update method calls: `execute()` → `call_tool()`
4. Verify parameter compatibility
5. Update error handling if `ToolError` → `RouterError`

## SUBTASK 4: Remove UnifiedToolExecutor exports

**File:** `packages/candle/src/domain/tool/mod.rs:20`

Delete:
```rust
// Legacy unified tool interface - DEPRECATED in favor of SweetMcpRouter
pub use unified::{UnifiedToolExecutor, ToolError};
```

## SUBTASK 5: Check if unified.rs can be deleted

After removing exports, check if `unified.rs` is still used:
```bash
grep -rn "use.*unified" packages/candle/src
```

If no results, delete `packages/candle/src/domain/tool/unified.rs`

## VALIDATION COMMANDS
```bash
# Verify no UnifiedToolExecutor usage
grep -rn "UnifiedToolExecutor" packages/candle/src
# Expected: 0 results

# Verify SweetMcpRouter is used instead
grep -rn "SweetMcpRouter" packages/candle/src
# Expected: at least 18 results

# Verify compilation
cargo check -p paraphym_candle
```

## DEFINITION OF DONE
- ✅ All 18 UnifiedToolExecutor usages migrated to SweetMcpRouter
- ✅ UnifiedToolExecutor export removed from mod.rs
- ✅ unified.rs deleted if no longer referenced
- ✅ Code compiles without errors
- ✅ Single tool router system (SweetMcpRouter only)

## EXECUTION ORDER
**Task 5 of 8** - Execute after infrastructure updated (LEGACY_D complete)

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on migration to new API

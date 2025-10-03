# LEGACY_D: Module Re-export Cleanup

## OBJECTIVE
Remove "backward compatibility" module re-exports. Stop aliasing ystream as async_task - use the actual module name.

## SCOPE
Remove re-exports that exist only to maintain "old" module names in an UNRELEASED library.

## SUBTASK 1: Remove async_task re-export from lib.rs
**File:** `packages/candle/src/lib.rs:240-241`

Delete:
```rust
// Alias for backward compatibility - people expect async_task module
pub use ystream as async_task;
```

## SUBTASK 2: Remove async_task re-exports from domain/mod.rs
**File:** `packages/candle/src/domain/mod.rs:37-39`

Delete:
```rust
// Alias for backward compatibility - people expect async_task module
pub use ystream as async_task;
pub use ystream::spawn_task as spawn_async; // Alias for backward compatibility
```

## SUBTASK 3: Update call sites

**Find usages:**
```bash
grep -rn "async_task::" packages/candle/src
grep -rn "spawn_async" packages/candle/src
```

**Known locations:**
- `packages/candle/src/builders/loader.rs:70` - `paraphym_domain::async_task::AsyncStream`
- `packages/candle/src/builders/loader.rs:264` - `paraphym_domain::async_task::AsyncStream`

**Migration:**
```rust
// BEFORE:
use paraphym_domain::async_task::AsyncStream;
// or
fn stream(self) -> paraphym_domain::async_task::AsyncStream<T>

// AFTER:
use ystream::AsyncStream;
// or
fn stream(self) -> ystream::AsyncStream<T>
```

## SUBTASK 4: Remove CandleCompletionProvider trait alias
**File:** `packages/candle/src/domain/completion/traits.rs:45-49`

Delete:
```rust
// Backward compatibility trait alias for existing code
pub trait CandleCompletionProvider: CandleCompletionModel {}

// Blanket implementation
impl<T: CandleCompletionModel> CandleCompletionProvider for T {}
```

**Migration:** Use `CandleCompletionModel` directly everywhere

## VALIDATION COMMANDS
```bash
# Verify no async_task module alias
grep -rn "pub use.*as async_task" packages/candle/src

# Verify no spawn_async alias
grep -rn "spawn_async" packages/candle/src

# Verify no CandleCompletionProvider usage
grep -rn "CandleCompletionProvider" packages/candle/src

# Verify compilation
cargo check -p paraphym_candle
```

## DEFINITION OF DONE
- ✅ No module re-exports with "as" for backward compat
- ✅ All imports use canonical module names (ystream, not async_task)
- ✅ CandleCompletionProvider trait removed
- ✅ Code compiles without errors

## EXECUTION ORDER
**Task 3 of 8** - Execute after LEGACY_C (before type alias removal)

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on module re-export removal

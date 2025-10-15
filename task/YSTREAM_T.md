# YSTREAM_T: Compilation Fixes - Post Comment Cleanup

## STATUS
The lib.rs comment cleanup is ✅ **COMPLETE**. However, the codebase does NOT compile due to incomplete YSTREAM migration in other modules.

## COMPLETED ITEMS ✅
- ✅ Line 32 comment updated: "using tokio streams" (was: "re-exporting ystream")
- ✅ Line 109 redundant comment removed: "AsyncStream removed - using tokio_stream instead"
- ✅ Zero ystream references in lib.rs
- ✅ Zero AsyncStream references in lib.rs
- ✅ tokio_stream re-exports present (lines 118-119)
- ✅ async_stream utilities re-exported (lines 121-127)

## OUTSTANDING ISSUES ❌

### Compilation Failures (103 errors in OTHER modules)

The lib.rs file itself is perfect, but `cargo check` fails due to incomplete YSTREAM migration in these modules:

#### 1. Core Modules - Missing Imports (2 errors)
**Files:** `core/mod.rs`, `core/engine.rs`
**Issue:** Missing `AsyncTask` and `spawn_task` imports
```rust
error[E0432]: unresolved import `crate::AsyncTask`
error[E0432]: unresolved imports `crate::AsyncTask`, `crate::spawn_task`
```
**Fix Needed:** Add proper imports or re-export these types in lib.rs if they should be available from crate root

#### 2. Chat Commands - AsyncStream Usage (18+ errors)
**Files:** `domain/chat/commands/execution.rs`, `domain/chat/commands/types/mod.rs`, `domain/chat/commands/types/events.rs`
**Issue:** Code still uses `AsyncStream` type which no longer exists
```rust
error[E0412]: cannot find type `AsyncStream` in this scope
error[E0433]: failed to resolve: use of undeclared type `AsyncStream`
error[E0432]: unresolved import `crate::AsyncStreamSender`
```
**Fix Needed:** Replace `AsyncStream<T>` with `Pin<Box<dyn Stream<Item = T> + Send>>` or similar tokio_stream pattern

#### 3. Workflow Modules - Missing StreamExt (6+ errors)
**Files:** `workflow/ops.rs`, `workflow/parallel.rs`
**Issue:** `StreamExt` trait not in scope
```rust
error[E0599]: no method named `try_next` found
error[E0599]: `Pin<Box<dyn tokio_stream::Stream>>` is not an iterator
```
**Fix Needed:** Add `use tokio_stream::StreamExt;` imports to these files

#### 4. Memory Module - Lifetime Issues (3 errors)
**File:** `memory/core/systems/episodic.rs:364`
**Issue:** Borrowed data escapes function
```rust
error[E0521]: borrowed data escapes outside of associated function
```
**Fix Needed:** Convert `&str` parameters to `String` before moving into the async closure

#### 5. Context and Domain Core - Missing AsyncTask (2 errors)
**Files:** `domain/context/loader.rs`, `domain/core/mod.rs`
**Issue:** Missing `AsyncTask` imports
```rust
error[E0432]: unresolved import `crate::AsyncTask`
```
**Fix Needed:** Same as #1 - proper import paths

## NEXT STEPS

These compilation failures represent **incomplete work from earlier YSTREAM migration tasks (M, N, O, P)**. The comment cleanup task (T) has been completed perfectly, but the Definition of Done includes "Compiles with 0 errors" which cannot be met until these other modules are fixed.

**Recommended Action:** Create follow-up tasks for each module category above (Core, Chat Commands, Workflow, Memory, Domain) to complete the AsyncStream → tokio Stream migration.

## VERIFICATION

Once fixes are applied:
```bash
cd packages/candle && cargo check
# Should show: "Finished `dev` profile [unoptimized + debuginfo] target(s)"
# With 0 errors
```

## FILE LOCATION
**Primary file:** `packages/candle/src/lib.rs` (✅ COMPLETE)
**Problem files:** See "Outstanding Issues" section above
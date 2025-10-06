# Fix CLI to Expose ALL Builder Methods as Flags - BLOCKING COMPILATION ERRORS

## ❌ STATUS: 2/10 - COMPILATION BLOCKED - NOT PRODUCTION READY

**Compilation Status:** ❌ **FAILED** - 13 compilation errors prevent any functionality from working

**Last Verified:** 2025-10-06 - Code review by Rust expert QA

---

## CRITICAL BLOCKING ISSUES

### 1. Compilation Errors (BLOCKING - Must Fix First)

**Current Status:** Package does not compile - 13 errors

**Errors Found:**
```
error[E0432]: unresolved import `crate::domain::completion::types::CandleCompletionChunk`
error[E0412]: cannot find type `MemoryResult` in module `crate::memory::core`
error[E0433]: failed to resolve: could not find `CandlePrompt` in `completion`
error[E0603]: enum import `CandleMessageRole` is private
error[E0107]: missing generics for struct `agent_role::CandleAgentRoleAgent`
error[E0308]: mismatched types
error[E0599]: no function or associated item named `new` found for struct `role::CandleAgentConversation`
error[E0599]: no method named `clone` found for type parameter `P`
```

**Root Cause:** Changes to `builders/agent_role.rs` introduced type import errors and visibility issues.

**Required Fixes:**
1. Fix all type imports in `builders/agent_role.rs`
2. Make `CandleMessageRole` enum public or use correct import path
3. Add generic parameters to all `CandleAgentRoleAgent` references
4. Fix method signatures and trait bounds

**File:** `packages/candle/src/builders/agent_role.rs`

---

## WHAT IS ACTUALLY COMPLETE (Verified)

### ✅ 1. CLI Argument Parsing (100% Complete)

**File:** `packages/candle/src/cli/args.rs`

**Implemented:**
- `embedding_model: String` field added (line 35)
- Default value `"stella"` in `Default` impl (line 61)
- Parsing logic for `--embedding-model` flag (line 144-149)

**Verification:** ✅ Code exists and is correct

---

### ✅ 2. Tool Warning Log (100% Complete)

**File:** `packages/candle/src/cli/runner.rs` (lines 58-64)

**Implemented:**
```rust
if !self.args.tools.is_empty() {
    eprintln!("⚠️  Warning: {} tool(s) specified but dynamic WASM loading not yet implemented:", self.args.tools.len());
    for tool in &self.args.tools {
        eprintln!("    - {}", tool);
    }
    eprintln!("    Tools will be available in a future release.");
}
```

**Verification:** ✅ Code is correct and matches task requirements

---

### ✅ 3. Memory Manager Enhancement (100% Complete)

**File:** `packages/candle/src/memory/core/manager/surreal.rs` (lines 475-505)

**Implemented:**
```rust
/// Create a new SurrealDB memory manager with a custom embedding model
pub async fn with_embedding_model(
    db: Surreal<Any>,
    embedding_model: Arc<dyn EmbeddingModel>
) -> Result<Self> {
    Ok(Self {
        db,
        embedding_model: Some(embedding_model),
    })
}
```

**Verification:** ✅ Method exists and accepts custom embedding models

---

### ✅ 4. Memory Initialization Code (90% Complete - Can't Verify Due to Compilation)

**File:** `packages/candle/src/cli/runner.rs` (lines 93-141)

**Implemented:**
- Embedding model parsing (handles "stella 1024" format)
- EmbeddingConfig creation
- EmbeddingModelFactory usage
- SurrealDB connection with `"memory://"`
- Database namespace/database initialization
- Memory manager creation with custom embedding model
- Table initialization

**Code Pattern:**
```rust
let embedding_model = EmbeddingModelFactory::create_embedding_model(embedding_config)
    .await
    .context("Failed to create embedding model")?;

let manager = SurrealDBMemoryManager::with_embedding_model(db, embedding_model).await
    .context("Failed to create memory manager")?;
```

**Issues:** Cannot verify functionality until compilation errors are fixed

---

### ✅ 5. Document Ingestion Pipeline (90% Complete - Can't Verify Due to Compilation)

**File:** `packages/candle/src/cli/runner.rs` (lines 143-199)

**Implemented:**
- Document count display
- Smart input resolution via `resolve_smart_input()`
- Content hash calculation using `content_hash()` utility
- MemoryNode creation with proper timestamps (`Utc::now()`)
- Custom metadata via `.with_custom_metadata()` builder method
- Memory ingestion via `memory_manager.create_memory()`
- Error handling with success/failure messages

**Code Quality:** ✅ Proper error handling, no .unwrap() or .expect()

**Issues:** Cannot verify functionality until compilation errors are fixed

---

### ✅ 6. Memory Passed to Builder (90% Complete - Can't Verify Due to Compilation)

**File:** `packages/candle/src/cli/runner.rs` (lines 245, 258)

**Implemented:**
```rust
.memory(memory_manager.clone())
```

Added to both builder branches (with and without max_tokens)

**Issues:** Cannot verify functionality until compilation errors are fixed

---

## DEFINITION OF DONE (Not Met)

**Required for completion:**

1. ❌ **Code must compile** - Currently failing with 13 errors
2. ⚠️ CLI accepts `--embedding-model` flag - Implemented but unverified
3. ⚠️ Memory initialized with specified model - Implemented but unverified
4. ⚠️ Documents ingested into memory - Implemented but unverified
5. ⚠️ Memory passed to builder - Implemented but unverified
6. ✅ Tool flag logs warning - Working
7. ❌ System is functional - Cannot verify until compilation succeeds
8. ❌ No crashes or panics - Cannot verify until compilation succeeds

**Current Status:** 1/8 requirements fully verified (12.5%)

---

## IMMEDIATE ACTION REQUIRED

### Priority 1: Fix Compilation Errors

**File:** `packages/candle/src/builders/agent_role.rs`

**Required Actions:**

1. **Fix Type Imports:**
   - Import or properly reference `CandleCompletionChunk`
   - Import or properly reference `MemoryResult`
   - Import or properly reference `CandlePrompt`

2. **Fix Visibility Issues:**
   - Make `CandleMessageRole` public or use correct module path
   - Verify all type exports in domain modules

3. **Fix Generic Parameters:**
   - Add `<P>` generic parameter to all `CandleAgentRoleAgent` references
   - Update trait bounds as needed

4. **Fix Method Signatures:**
   - Implement `new()` for `CandleAgentConversation` or use correct constructor
   - Add `Clone` bound to provider type `P` where needed

### Priority 2: Verify Functionality

Once compilation succeeds:

1. Test `--embedding-model stella` (default)
2. Test `--embedding-model "stella 2048"` (custom dimensions)
3. Test `--embedding-model bert` (different model)
4. Test `--document ./README.md` (single file)
5. Test `--document https://example.com/doc.txt` (URL)
6. Test `--tool ./plugin.wasm` (warning display)
7. Verify memory contains ingested documents
8. Verify chat can retrieve documents from memory

---

## CODE QUALITY ASSESSMENT

**What's Good:**
- ✅ Proper error handling with `.context()` throughout
- ✅ No `.unwrap()` or `.expect()` in implementation code
- ✅ Good separation of concerns
- ✅ Appropriate use of builder methods (`.with_custom_metadata()`)
- ✅ Smart input resolution for files/URLs/text
- ✅ Proper Arc wrapping for shared state

**What's Broken:**
- ❌ Type system errors preventing compilation
- ❌ Import/visibility issues
- ❌ Generic parameter mismatches
- ❌ Missing trait implementations

---

## ESTIMATED EFFORT TO COMPLETE

**Fix compilation errors:** 2-4 hours
**Verify and test functionality:** 1-2 hours
**Total remaining work:** 3-6 hours

**Complexity:** HIGH (requires understanding of Rust type system and trait bounds)
**Risk:** MEDIUM (core functionality is implemented, just needs type fixes)

---

## FILES MODIFIED (Verified)

1. ✅ `packages/candle/src/cli/args.rs` - embedding_model field added
2. ✅ `packages/candle/src/cli/runner.rs` - memory init and document ingestion
3. ✅ `packages/candle/src/memory/core/manager/surreal.rs` - with_embedding_model() added
4. ❌ `packages/candle/src/builders/agent_role.rs` - has compilation errors

---

**Task Status:** BLOCKED BY COMPILATION ERRORS  
**Priority:** CRITICAL (blocking all other functionality)  
**Next Step:** Fix all type import and visibility issues in agent_role.rs
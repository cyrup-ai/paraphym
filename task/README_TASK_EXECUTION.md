# Task Execution Guide

## Quick Start

**Current Status**: 49 compilation errors in paraphym_candle package
**Next Action**: Start with Phase 1 tasks in order

## Task Files Created

### Master Plan
- **MASTER_COMPILATION_FIXES.md** - Overview of all 49 errors, execution order, progress tracking

### Phase 1: Context and Memory API Fixes (DO THESE FIRST)
1. **FIX_MEMORY_OPS_API.md** (6 errors)
   - Fix `memory_ops.rs` to use streaming CandleContext API
   - Replace `get_documents()` with `load()` stream collection
   - Fix SurrealDBMemoryManager instantiation

2. **FIX_CONTEXT_LOADING_MOD.md** (0 errors, but required for CHAT_INT_3)
   - Complete context loading in `mod.rs`
   - Call `load_context_into_memory()` after initialization
   - Populate `context_documents` from streaming API
   - **Depends on**: FIX_MEMORY_OPS_API.md must be completed first

3. **FIX_DOCUMENT_BAD_CHUNK.md** (5 errors)
   - Fix `processor.rs` type alias issue
   - Replace `Document::bad_chunk` with `CandleDocument::bad_chunk`
   - 5 simple find-and-replace fixes

**Phase 1 Total**: 11 errors resolved

### Phase 2: Memory Manager Trait Fixes (COMPLEX)
4. **FIX_MEMORY_MANAGER_TRAIT.md** (28+ errors)
   - Fix MemoryManager trait implementation across memory subsystem
   - Add missing trait methods (search_by_content, query_by_type)
   - Fix trait signatures (update_quantum_signature)
   - Implement trait for Arc<T> or fix method access
   - Fix type conversions and error handling
   - Add Debug implementations
   - **Warning**: This is a LARGE task, may need sub-tasks

**Phase 2 Total**: 28+ errors resolved

### Phase 3: Final Cleanup
5. Clean up 19 warnings (unused imports, deprecated types)

## Execution Order (STRICT)

```
Step 1: FIX_MEMORY_OPS_API.md
  ↓
Step 2: FIX_CONTEXT_LOADING_MOD.md (depends on Step 1)
  ↓
Step 3: FIX_DOCUMENT_BAD_CHUNK.md (independent)
  ↓
Step 4: FIX_MEMORY_MANAGER_TRAIT.md (complex, allow extra time)
  ↓
Step 5: Final cleanup and verification
```

## How to Use These Task Files

Each task file contains:
- ✅ **OBJECTIVE**: What needs to be fixed
- ✅ **STATUS**: Current error state
- ✅ **ERRORS TO FIX**: Specific compilation errors with line numbers
- ✅ **ROOT CAUSE**: Why the errors are happening
- ✅ **IMPLEMENTATION PLAN**: Exact code changes needed
- ✅ **VERIFICATION**: How to test the fix
- ✅ **DEFINITION OF DONE**: Checklist of completion criteria

## Recommended Workflow

1. **Read the task file completely** before starting
2. **Use sequential_thinking** for complex fixes
3. **Make changes incrementally** (one fix at a time)
4. **Verify after each change**: `cargo check -p paraphym_candle`
5. **Update task file** with completion status
6. **Move to next task** only after verification passes

## Progress Tracking Commands

```bash
# Total error count (should decrease with each completed task)
cargo check -p paraphym_candle 2>&1 | grep -E "error\[E" | wc -l

# Check specific file
cargo check -p paraphym_candle 2>&1 | grep "memory_ops.rs"

# Final verification
cargo check --workspace && echo "✅ ALL ERRORS RESOLVED"
```

## Integration with Existing Tasks

### Completed Tasks (from previous work)
- ✅ **CHAT_INT_1.md** - session.rs parameter usage (DONE)
- ✅ **CHAT_INT_2.md** - Build configuration objects (assumed DONE)

### In Progress
- ⏳ **CHAT_INT_3.md** - Will be COMPLETE after FIX_MEMORY_OPS_API + FIX_CONTEXT_LOADING_MOD
- ⏳ **CHAT_INT_4.md** - Final cleanup (after all fixes)

### Other Tasks (for reference)
- **DECOMP_033.md** - Existing decomposition task
- **FIX_*_NOOP.md** - Various noop/stub fixes (lower priority)

## Quick Reference: Error Distribution

| File/Area | Error Count | Task File |
|-----------|-------------|-----------|
| memory_ops.rs | 6 errors | FIX_MEMORY_OPS_API.md |
| processor.rs | 5 errors | FIX_DOCUMENT_BAD_CHUNK.md |
| coordinator/* | 15+ errors | FIX_MEMORY_MANAGER_TRAIT.md |
| memory subsystem | 13+ errors | FIX_MEMORY_MANAGER_TRAIT.md |
| **TOTAL** | **49 errors** | **All tasks above** |

## Estimated Time to Completion

- **Phase 1**: 2-3 hours (straightforward)
- **Phase 2**: 4-6 hours (complex trait fixes)
- **Phase 3**: 1 hour (cleanup)
- **Total**: 7-10 hours focused development

## Success Criteria

When all tasks are complete:
- [ ] Zero compilation errors: `cargo check --workspace`
- [ ] Successful build: `cargo build --release`
- [ ] All tests pass: `cargo test --workspace`
- [ ] CHAT_INT_3 integration fully functional
- [ ] Context loading working end-to-end
- [ ] Memory manager trait properly implemented

## Need Help?

If you encounter issues:
1. Check MASTER_COMPILATION_FIXES.md for context
2. Read the specific task file completely
3. Use sequential_thinking for complex analysis
4. Verify incrementally (don't batch multiple fixes)
5. Check for dependency issues (Phase 1 before Phase 2)

---

**Last Updated**: 2025-10-20
**Status**: Ready to begin execution
**Next Action**: Start with FIX_MEMORY_OPS_API.md

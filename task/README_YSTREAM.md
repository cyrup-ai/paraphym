# YSTREAM → TOKIO_STREAM MIGRATION

## 📋 OVERVIEW

Converting from ystream (sync/async bridging) to tokio_stream (pure async).
60+ files need conversion in proper dependency order.

## 🎯 CURRENT STATUS

**Progress**: ~35/60+ files complete (58%)
**Current Task**: YSTREAM_L (Agent modules)
**Total Tasks**: 22 (A-V) → 13 remaining (I-V simplified)

## 📚 ACTIVE TASK FILES (8 Remaining)

**See `WORKFLOW_ORDER.md` for optimized execution sequence**

### ✅ COMPLETED & DELETED (A-Q)
All foundation, core, capabilities, domain, and workflow tasks complete.
14 task files deleted (A-K, O-Q).

### 🔄 REMAINING WORK (L-V) - Execute in Order

#### Phase 1: Quick Wins ⚡
1. **`YSTREAM_L.md`** - Agent modules (1 file: `agent/prompt.rs`) - 5 min
2. **`YSTREAM_N.md`** - Chat core (1 file: `orchestration.rs`) - 10 min  
3. **`YSTREAM_S.md`** - CLI (1 file: `cli/runner.rs`) - 15 min

#### Phase 2: Medium Complexity 📦
4. **`YSTREAM_R.md`** - Builders (6 files) - 30 min

#### Phase 3: Complex Work ⚠️
5. **`YSTREAM_M.md`** - Chat commands (heavy macros) - 90 min

#### Phase 4: Critical Path 🔴
6. **`YSTREAM_T.md`** - lib.rs cleanup (FIXES COMPILATION) - 15 min
7. **`YSTREAM_U.md`** - Remove dependency - 2 min
8. **`YSTREAM_V.md`** - Final verification - 10 min

**Total Time**: ~3 hours

## 📖 MASTER INDEX

See `YSTREAM_INDEX.md` for complete dependency tracking and status.

## ⚠️ CRITICAL RULES FOR REMAINING WORK

1. **FOLLOW WORKFLOW_ORDER.md** - Optimized sequence: L→N→S→R→M→T→U→V
2. **QUICK WINS FIRST** - Build momentum with easy tasks (L, N, S)
3. **TACKLE M LAST** - Complex macro work after easier files done
4. **T FIXES COMPILATION** - Must convert all files before running T
5. **U AFTER T** - Can't remove dependency until lib.rs cleanup complete
6. **TEST FREQUENTLY** - Run `cargo check` after each phase

## 🚫 OLD FILES (IGNORE)

- `YSTREAM_D_OLD.md` - Broken (tried engine too early)
- `YSTREAM_E_OLD.md` - Broken (wrong dependencies)
- `YSTREAM_F_OLD.md` - Broken (cleanup task, not conversion)

These had dependency inversions. Use the `_REDO` versions instead.

## 🎓 OPTIMIZED WORKFLOW RATIONALE

**Phase Structure**: Easy → Medium → Complex → Critical

```
Quick Wins (L, N, S)
    ↓ Build momentum, learn patterns
Medium Complexity (R)  
    ↓ Practice on 6 similar files
Complex Work (M)
    ↓ Tackle macros when warmed up
Critical Path (T, U, V)
    ↓ Fix compilation, finalize
COMPLETE ✅
```

**Why This Works**:
- **Psychological**: Early wins create momentum
- **Technical**: Simple patterns before complex ones
- **Dependency**: T requires all conversions complete
- **Risk**: Low-risk first, complex middle, critical last

## 🔍 VERIFICATION

After each task:
```bash
cd /Volumes/samsung_t9/paraphym
cargo check --package paraphym_candle
# Must succeed with 0 errors
```

## 📊 DETAILED STATUS BREAKDOWN

### ✅ Fully Complete (58% - 35/60 files)
- Foundation: async_stream, concurrency, core builders
- Core: engine.rs, generator.rs, extractors
- Capabilities: text-to-text (kimi, phi4, qwen), vision, image generation
- Domain: pool, completion, memory, tools, workflow

### 🔄 Remaining Work (42% - 25/60 files)
- **L**: Agent (1 file - `prompt.rs`)
- **M**: Chat commands (1 file - heavy macro usage) ⚠️ COMPLEX
- **N**: Chat core (1 file - `orchestration.rs`)
- **O-P**: Chat realtime/search ✅ (complete)
- **Q**: Workflow ✅ (complete)
- **R**: Builders (6 files)
- **S**: CLI (1 file)
- **T**: lib.rs cleanup 🔴 CRITICAL (blocks compilation)
- **U**: Remove dependency
- **V**: Verification

### 🔴 Current Blockers
- **Compilation broken**: 3 import errors due to lib.rs re-exports
- **Must complete T before U**: lib.rs cleanup is critical path

See `PICKUP_POINT.md` for detailed continuation instructions.

## 🎯 END GOAL

- ✅ Zero ystream imports
- ✅ Zero AsyncStream usage  
- ✅ 100% tokio_stream
- ✅ Pure async/await
- ✅ No sync/async bridging
- ✅ Better performance

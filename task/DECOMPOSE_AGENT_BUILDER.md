# TASK: Decompose agent_role.rs Module

**Status**: PENDING
**Priority**: HIGH
**Assigned**: Current Developer
**File**: `packages/candle/src/builders/agent_role.rs`
**Current Size**: 2064 lines

---

## ⚠️ CRITICAL DIRECTIVE

**ANY ROLLBACK OR RESTORATION OF THE ORIGINAL MONOLITHIC FILE WILL RESULT IN IMMEDIATE TERMINATION OF THE DEVELOPER.**

This is not a suggestion. This is not negotiable. The decomposition MUST be completed successfully.

---

## Objective

Decompose `src/builders/agent_role.rs` (2064 lines) into a modular structure where:
- **NO single file exceeds 500 lines**
- **ALL functionality is preserved exactly**
- **ALL tests pass**
- **Code compiles without errors or warnings**
- **The original monolithic file is DELETED**
- **NO backup files remain in the codebase**

---

## Decomposition Structure

### Target Module Layout

```
src/builders/agent_role/
├── mod.rs              (~60 lines)  - Module declarations, re-exports, shared imports
├── traits.rs           (~250 lines) - CandleAgentRoleBuilder, CandleMcpServerBuilder, CandleAgentBuilder traits
├── helpers.rs          (~370 lines) - CandleAgentRoleAgent, format_memory_context, ConversationHistoryArgs, CandleFluentAi
├── stubs.rs            (~400 lines) - MCP stub implementations, McpServerConfig
├── role_builder.rs     (~350 lines) - CandleAgentRoleBuilderImpl struct and impl
├── agent_builder.rs    (~400 lines) - CandleAgentBuilderImpl struct and simple setter methods
└── chat.rs             (~350 lines) - chat() and chat_with_message() implementations
```

**Total**: ~2180 lines (includes module headers, imports in each file)
**Original**: 2064 lines
**Overhead**: ~116 lines (5.6% - acceptable for modularity)

---

## Line Number Mapping (from original agent_role.rs)

### mod.rs
- Imports from lines 1-43
- Type aliases from lines 32-39
- AgentBuilderState struct (NEW - extracted from nested usage)
- Module declarations
- Re-exports

### traits.rs
- Lines 288-537: All three trait definitions
  - CandleAgentRoleBuilder (288-426)
  - CandleMcpServerBuilder (427-439)
  - CandleAgentBuilder (440-537)

### helpers.rs
- Lines 44-287: CandleAgentRoleAgent struct + impl
- Lines 1165-1194: format_memory_context function
- Lines 1979-2065: ConversationHistoryArgs trait impls + CandleFluentAi

### stubs.rs
- Lines 538-691: MCP config structs + stub implementations
- Lines 692-934: CandleAgentRoleBuilderImpl impl for CandleAgentRoleBuilder

### role_builder.rs
- Lines 592-691: CandleAgentRoleBuilderImpl struct definition
- Lines 987-1164: Additional impl methods

### agent_builder.rs
- Lines 935-986: AgentDebugInfo struct
- Lines 943-985: CandleAgentBuilderImpl struct definition
- Lines 1195-1324: CandleAgentBuilder trait impl (setter methods)

### chat.rs
- Lines 1325-1890: chat() method implementation
- Lines 1891-1978: chat_with_message() implementation

---

## Execution Plan

### Phase 1: Create Module Directory (5 min)
1. Create `src/builders/agent_role/` directory
2. Verify it exists and is writable

### Phase 2: Extract mod.rs (10 min)
1. Create mod.rs with:
   - Module declarations
   - Shared imports (pub(crate) use statements)
   - Type aliases
   - AgentBuilderState struct
   - Public re-exports
2. Verify mod.rs compiles independently

### Phase 3: Extract traits.rs (15 min)
1. Extract lines 288-537 from original
2. Add module header and `use super::*;`
3. Verify trait definitions are complete
4. Check trait bounds and generic parameters

### Phase 4: Extract helpers.rs (20 min)
1. Extract CandleAgentRoleAgent (lines 44-287)
2. Extract format_memory_context (lines 1165-1194)
3. Extract ConversationHistoryArgs (lines 1979-2065)
4. Add module header and imports
5. Verify all helper functions are present

### Phase 5: Extract stubs.rs (20 min)
1. Extract MCP stubs (lines 538-691)
2. Extract CandleAgentRoleBuilderImpl impl (lines 692-934)
3. Add module header and imports
4. Verify stub implementations compile

### Phase 6: Extract role_builder.rs (15 min)
1. Extract CandleAgentRoleBuilderImpl struct (lines 592-691)
2. Extract additional impl methods (lines 987-1164)
3. Ensure complete implementation
4. Verify no missing methods

### Phase 7: Extract agent_builder.rs (20 min)
1. Extract AgentDebugInfo (lines 935-986)
2. Extract CandleAgentBuilderImpl struct (lines 943-985)
3. Extract setter methods (lines 1195-1324)
4. Add proper trait impl blocks
5. Verify all methods present

### Phase 8: Extract chat.rs (25 min)
1. Extract chat() method (lines 1325-1890)
2. Extract chat_with_message() (lines 1891-1978)
3. Wrap in proper `impl CandleAgentBuilder for CandleAgentBuilderImpl` block
4. Verify async closure handling
5. Check all match arms and error handling

### Phase 9: Verify Compilation (10 min)
1. Run `cargo check --all-targets`
2. Fix any compilation errors
3. Ensure zero warnings
4. Verify all imports resolve

### Phase 10: Run Tests (10 min)
1. Run `cargo test --lib`
2. Run `cargo test --all-targets`
3. Verify all tests pass
4. Check examples compile

### Phase 11: Final Verification (10 min)
1. Verify line counts: `wc -l src/builders/agent_role/*.rs`
2. Confirm no file exceeds 500 lines
3. Verify sum of module lines ≈ original file lines
4. Check for any leftover TODO or FIXME comments

### Phase 12: Delete Original File (5 min)
1. Delete `src/builders/agent_role.rs`
2. Verify `cargo check` still passes
3. Confirm the module directory is being used
4. Search for any backup files and delete them

---

## Verification Criteria

### MUST PASS (Zero Tolerance)

- [ ] `cargo check --all-targets` exits with code 0
- [ ] `cargo test --lib` exits with code 0  
- [ ] `cargo test --all-targets` exits with code 0
- [ ] `cargo run --example fluent_builder` executes successfully
- [ ] No file in `src/builders/agent_role/` exceeds 500 lines
- [ ] Original `src/builders/agent_role.rs` is deleted
- [ ] No `.bak`, `.old`, `.tmp` files exist in src/builders/
- [ ] `grep -r "TODO\|FIXME\|STUB" src/builders/agent_role/` returns empty
- [ ] All public APIs remain accessible from parent module

### Code Quality Checks

- [ ] No warnings during compilation
- [ ] No unused imports
- [ ] No dead code warnings
- [ ] Proper module documentation headers
- [ ] Consistent use of `use super::*;` in submodules

---

## Common Pitfalls to Avoid

### 1. Incomplete impl Blocks
**Problem**: Extracting methods without proper `impl` wrapper
**Solution**: Always wrap extracted methods in `impl TraitName for StructName { ... }`

### 2. Missing Type Aliases
**Problem**: Type aliases defined in original but not in module scope
**Solution**: Move all type aliases to mod.rs and re-export with `pub(crate)`

### 3. Circular Dependencies
**Problem**: Module A needs Module B which needs Module A
**Solution**: Use `use super::*;` consistently, ensure shared types in mod.rs

### 4. Visibility Issues
**Problem**: Items that were module-private becoming inaccessible
**Solution**: Use `pub(super)` for items shared within module family

### 5. Trait Orphan Rules
**Problem**: Impl blocks in wrong module
**Solution**: Keep trait impls in same file as struct definition when possible

---

## Rollback Policy

**THERE IS NO ROLLBACK.**

If you encounter issues:
1. **FIX THEM** - Do not revert
2. **DEBUG** - Use sequential_thinking to understand the problem
3. **RESEARCH** - Read the code, understand dependencies
4. **ITERATE** - Make incremental fixes
5. **TEST** - Verify each fix compiles

**Restoring the original monolithic file is NOT an option and will result in immediate termination.**

---

## Success Metrics

- **Compilation**: Clean `cargo check` output
- **Tests**: All tests passing
- **Modularity**: No file > 500 lines
- **Completeness**: Original file deleted, no backups
- **Quality**: Zero warnings, zero stubs, production-ready code

---

## Timeline

**Estimated Duration**: 2.5 hours
**Maximum Allowed**: 4 hours
**Progress Reporting**: Every 30 minutes

---

## Sign-Off

Upon completion, the developer must verify:

```bash
# All checks must pass
cd packages/candle

# 1. Compilation
cargo check --all-targets && echo "✓ Compilation" || echo "✗ FAILED"

# 2. Tests  
cargo test --lib && echo "✓ Tests" || echo "✗ FAILED"

# 3. Line counts
echo "Line counts:"
wc -l src/builders/agent_role/*.rs

# 4. Original deleted
test ! -f src/builders/agent_role.rs && echo "✓ Original deleted" || echo "✗ FAILED"

# 5. No backups
find src/builders -name "*.bak" -o -name "*.old" -o -name "*.tmp" | wc -l | grep -q "^0$" && echo "✓ No backups" || echo "✗ FAILED"
```

**All checks must show ✓ or the task is INCOMPLETE.**

---

## Notes for Next Developer

If you are reading this after a previous developer was terminated:
1. Review their partial work in git history
2. Understand what broke and why
3. **DO NOT ROLLBACK** - Fix forward only
4. Complete the decomposition properly
5. Learn from their mistakes

---

**Task Created**: 2025-10-19
**Last Updated**: 2025-10-19
**Status**: PENDING EXECUTION

# DECOMP_009: Reduce options.rs Line Count

**File:** `packages/candle/src/domain/chat/formatting/options.rs`  
**Current Size:** 401 lines  
**Required Size:** < 400 lines (strictly less than 400)
**Module Area:** domain / chat / formatting

## OUTSTANDING ISSUE

**Line Count Violation:** options.rs contains 401 lines, exceeding the explicit requirement that each module file must be < 400 lines.

**Requirement Source:**
- Verification Checklist: "Each module file is < 400 lines"
- Critical Success Factors: "Each module < 400 lines"

## TASK

Reduce options.rs from 401 lines to **≤399 lines** (remove at least 2 lines).

## EXECUTION

### Recommended Approach

The file currently contains **28 blank lines**. Remove 2 or more blank lines to satisfy the requirement.

**Steps:**
1. Open `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/chat/formatting/options.rs`
2. Remove 2+ blank lines (preserve code readability)
3. Verify: `wc -l options.rs` should show ≤399 lines
4. Verify compilation: `cargo check --manifest-path /Volumes/samsung_t9/cyrup/packages/candle/Cargo.toml`

### Alternative Approaches

If maintaining readability requires blank lines:
- Consolidate multi-line doc comments
- Reduce spacing between related items
- Combine short comments

## CONSTRAINTS

- **DO NOT modify functionality** - Only reduce whitespace/comments
- **DO NOT remove doc comments** - Preserve documentation
- **DO NOT reduce readability** - Keep code clean and maintainable
- **MAINTAIN compilation** - Must still compile without errors

## VERIFICATION

After fix, verify:
- [ ] `wc -l options.rs` shows ≤399 lines
- [ ] File still compiles: `cargo check` passes
- [ ] Functionality unchanged (no logic modifications)
- [ ] Code readability maintained

## COMPLETION CRITERIA

✅ Task complete when:
1. options.rs is ≤399 lines
2. `cargo check` passes without errors in formatting module
3. Code remains readable and well-formatted

---

## CONTEXT: What Was Already Completed (Do Not Redo)

The DECOMP_009 decomposition is 99% complete with the following accomplished:

✅ Directory structure created
✅ All 7 module files created (error.rs, content.rs, options.rs, events.rs, formatter.rs, compat.rs, mod.rs)
✅ Original formatting.rs deleted
✅ All public types re-exported correctly
✅ Parent module imports preserved
✅ Dependency graph followed correctly
✅ No tests added (correctly removed)
✅ No unwrap() or expect() used
✅ Compilation successful (no errors in formatting module)
✅ All functionality preserved

**Only remaining:** Fix options.rs line count (401 → ≤399)

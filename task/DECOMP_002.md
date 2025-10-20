# DECOMP_002: Decompose `macros.rs`

**File:** `packages/candle/src/domain/chat/macros.rs`  
**Current Size:** 2,032 lines  
**Module Area:** domain / chat

## OBJECTIVE

Decompose the monolithic `macros.rs` (2,032 lines) into smaller, focused, maintainable modules while preserving all existing functionality.

## CONSTRAINTS

- **NO TESTS:** Do not write any unit tests, integration tests, or test code. Testing is handled by a separate team.
- **NO BENCHMARKS:** Do not write any benchmark code. Performance testing is handled by a separate team.
- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is.
- **SINGLE SESSION:** This task must be completable in one focused Claude session.

## SUBTASK 1: Analyze Current Structure

Read and analyze `packages/candle/src/domain/chat/macros.rs` to identify:
- Distinct logical groupings of functionality
- Natural module boundaries
- Shared types, structs, and traits
- Dependencies between different sections
- Public vs private API surface

Document the analysis clearly before proceeding.

## SUBTASK 2: Design Module Structure

Create a decomposition plan that:
- Breaks the file into 3-5 focused modules (or more if needed)
- Groups related functionality together
- Minimizes circular dependencies
- Maintains clear module boundaries
- Preserves the existing public API

Each new module should be:
- < 300 lines of code
- Single responsibility focused
- Well-named and purposeful

## SUBTASK 3: Create New Module Files

For each identified module:
- Create a new `.rs` file in an appropriate location
- Move the relevant code from the original file
- Ensure all necessary imports are included
- Add clear module documentation

## SUBTASK 4: Update the Original File

Transform the original file into a module aggregator:
- Add `mod` declarations for new modules
- Re-export public items as needed with `pub use`
- Ensure the public API remains unchanged
- Keep only coordination logic if any

## SUBTASK 5: Verify Compilation

- Run `cargo check` to ensure no compilation errors
- Fix any broken imports or visibility issues
- Ensure all existing dependents still compile

## DEFINITION OF DONE

- [ ] `macros.rs` is reduced to < 300 lines (ideally much less)
- [ ] All functionality is preserved in new focused modules
- [ ] New modules are each < 300 lines
- [ ] Public API remains unchanged
- [ ] `cargo check` passes without errors
- [ ] No tests written (per constraints)
- [ ] No benchmarks written (per constraints)
- [ ] Code is well-documented with module-level comments

## RESEARCH NOTES

### File Location
`packages/candle/src/domain/chat/macros.rs`

### Key Considerations
- This file is part of the **domain / chat** module area
- File size (2,032 lines) indicates high complexity or multiple responsibilities
- Look for natural seams: distinct structs, trait implementations, helper functions
- Common patterns: separate types, implementations, utilities, constants

### Decomposition Strategy
1. **Read first** - Understand what's actually in the file
2. **Identify clusters** - Find groups of related functionality
3. **Plan modules** - Design 3-5 focused modules
4. **Execute carefully** - Move code systematically
5. **Verify** - Ensure everything still works

### Module Relationships
Pay attention to:
- How this file is imported by other modules
- What it exports publicly
- Internal implementation details that can be made private
- Opportunities to reduce coupling

## SUCCESS CRITERIA

This task is successful when:
1. The original monolithic file is decomposed into smaller, focused modules
2. All functionality is preserved without behavior changes
3. The codebase compiles without errors
4. The code is more maintainable and easier to understand
5. No tests or benchmarks were added (per team policy)

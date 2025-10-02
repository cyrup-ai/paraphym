# INPROD_14: Memory Pool Reset Methods

## SEVERITY: MEDIUM

## OBJECTIVE
Implement reset methods for MemoryNode to avoid recreation cost on pool acquire. Currently accepting the performance penalty of full recreation.

## LOCATION
- `packages/candle/src/domain/memory/pool.rs`

## CURRENT STATE
- Line 70: `// but for now we'll accept the cost of recreation on acquire`
- Pool returns nodes but accepts cost of recreating them
- No reset methods exist to reuse node memory allocations

## SUBTASK 1: Add Reset Method to MemoryNode
- Identify MemoryNode type definition
- Add reset() or clear() method to MemoryNode
- Reset internal state without deallocating memory
- Preserve allocated capacity where possible

## SUBTASK 2: Update Pool Acquire Logic
- Locate pool.rs:70 and acquire method
- Call reset method on pooled nodes before returning
- Reuse existing allocations instead of creating new nodes
- Improve performance by avoiding unnecessary allocations

## SUBTASK 3: Ensure State is Properly Reset
- Clear all node data that should be reset
- Maintain any pooled allocations (Vecs, Strings, etc.)
- Ensure reset nodes are equivalent to newly created ones
- Verify no stale data leaks between uses

## DEFINITION OF DONE
- [ ] MemoryNode has reset method implemented
- [ ] Pool acquire calls reset instead of recreating
- [ ] Allocations are reused for better performance
- [ ] No stale data remains after reset
- [ ] Stub comment removed

## RESEARCH NOTES
- Review MemoryNode structure and fields
- Check for existing reset patterns in the codebase
- Examine what state needs to be cleared vs preserved
- Look at memory pool patterns for reference

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

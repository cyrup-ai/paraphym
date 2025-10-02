# INPROD_10: Episodic Memory Repository Integration

## SEVERITY: HIGH

## OBJECTIVE
Implement actual memory repository integration for episodic memory creation instead of just cloning the memory node.

## LOCATION
- `packages/candle/src/memory/core/systems/episodic.rs`

## CURRENT STATE
- Line 397: `// TODO: In a real implementation, this would use something like:`
- Comment shows desired pattern: `repo.create(&episodic.base.id, &memory_node)?`
- Currently just clones memory_node without persisting to repository

## SUBTASK 1: Integrate Memory Repository for Creation
- Locate episodic.rs:397
- Replace memory clone with actual repository create call
- Use RwLock or appropriate locking for repository access
- Persist memory node to the database/repository

## SUBTASK 2: Handle Repository Errors
- Properly handle repository creation errors
- Return appropriate error types on persistence failure
- Ensure transaction consistency if needed

## SUBTASK 3: Return Repository-Created Memory
- Use the memory node returned from repository (may have assigned IDs, timestamps)
- Don't return the pre-creation clone
- Ensure created_memory reflects actual persisted state

## DEFINITION OF DONE
- [ ] Memory nodes are actually persisted to repository
- [ ] Repository create method is called correctly
- [ ] Repository errors are properly handled
- [ ] Returned memory reflects actual persisted state
- [ ] TODO comment and stub removed

## RESEARCH NOTES
- Review memory repository interface and available methods
- Check for existing RwLock patterns in memory code
- Examine how episodic.base.id should be used
- Look at other memory system repository integrations

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

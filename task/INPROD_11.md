# INPROD_11: Query Optimizer Implementation

## SEVERITY: HIGH

## OBJECTIVE
Implement actual query optimization instead of just passing through the query plan unchanged. The optimizer currently does nothing.

## LOCATION
- `packages/candle/src/memory/query/query_optimizer.rs`

## CURRENT STATE
- Line 195: `// For now, just mark that we checked`
- Optimizer's apply method returns input plan unchanged
- Index usage optimization is not implemented
- Full scans are not replaced with index scans

## SUBTASK 1: Implement Index Detection
- Locate query_optimizer.rs:195 in the apply method
- Check what indexes are available for the query
- Identify opportunities to use indexes instead of full scans
- Analyze query predicates against available indexes

## SUBTASK 2: Replace Full Scans with Index Scans
- Transform QueryPlan to use index scans where beneficial
- Replace full table/collection scans with indexed lookups
- Maintain query correctness while improving performance
- Update plan steps to reflect index usage

## SUBTASK 3: Implement Cost-Based Decision Making
- Calculate cost estimates for different query plans
- Compare full scan cost vs index scan cost
- Choose optimal plan based on estimated costs
- Consider selectivity and data distribution

## DEFINITION OF DONE
- [ ] Optimizer actually analyzes available indexes
- [ ] Full scans are replaced with index scans when beneficial
- [ ] Query plans are actually optimized
- [ ] Cost-based decisions are made
- [ ] Stub comment removed

## RESEARCH NOTES
- Review QueryPlan structure and available operations
- Examine index metadata and how to check availability
- Look for cost estimation patterns in the codebase
- Review query predicate and filter structures

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

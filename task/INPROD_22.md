# INPROD_22: Reasoner Strategy Implementation

## SEVERITY: MEDIUM

## OBJECTIVE
Implement full reasoning strategies instead of hardcoded score calculation. The reasoner is simplified for WASM but should use actual strategy implementations.

## LOCATION
- `packages/sweetmcp/plugins/reasoner/src/lib.rs`

## CURRENT STATE
- Line 91: `// Simplified reasoner for the WASM plugin. In a real implementation,`
- Line 113: `// Calculate score (in a real implementation, this would use the selected strategy)`
- Score is hardcoded: `0.7 + (request.thought_number as f64 * 0.05)`
- Strategy selection is ignored

## SUBTASK 1: Implement Strategy Pattern
- Locate reasoner/src/lib.rs:91 and 113
- Create strategy implementations for each reasoning type
- Implement beam_search, monte_carlo, best_first, etc.
- Use selected strategy from request

## SUBTASK 2: Calculate Scores Using Strategy
- Replace hardcoded score calculation with strategy-based scoring
- Pass thought content and context to strategy
- Use strategy-specific algorithms for score calculation
- Return meaningful scores based on reasoning quality

## SUBTASK 3: Optimize for WASM
- Keep implementations WASM-compatible
- Minimize binary size where possible
- Avoid heavy dependencies that don't compile to WASM

## DEFINITION OF DONE
- [ ] All reasoning strategies are implemented
- [ ] Score calculation uses selected strategy
- [ ] Hardcoded calculation is removed
- [ ] WASM compatibility is maintained
- [ ] Stub comments removed

## RESEARCH NOTES
- Review reasoning strategy types and algorithms
- Check for existing strategy implementations in non-WASM code
- Examine ReasoningRequest structure and available parameters
- Look for WASM-compatible algorithm implementations

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

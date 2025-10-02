# INPROD_6C: Chat Macros - Condition Evaluation Implementation

## SEVERITY: MEDIUM

## OBJECTIVE
Implement sophisticated condition evaluation in the macro system instead of simple string splitting. This affects two duplicate locations.

## LOCATION
- `packages/candle/src/domain/chat/macros.rs`

## CURRENT STATE
- Line 758: `// Simple condition evaluation - in a real implementation, this would be more sophisticated`
- Line 1438: Duplicate of above
- Only supports == with simple string splitting
- No support for !=, <, >, <=, >=, &&, ||, or other operators

## SUBTASK 1: Implement Full Comparison Operators
- Locate macros.rs:758 `_evaluate_condition` method
- Add support for: ==, !=, <, >, <=, >=
- Handle numeric comparisons properly
- Handle string comparisons

## SUBTASK 2: Implement Logical Operators
- Add support for && (AND) and || (OR)
- Add support for ! (NOT)
- Handle operator precedence correctly
- Support parentheses for grouping

## SUBTASK 3: Implement Type-Aware Evaluation
- Detect numeric vs string values
- Convert types appropriately for comparison
- Handle boolean values (true/false)
- Support variable dereferencing in conditions

## SUBTASK 4: Update Second Instance
- Locate macros.rs:1438 `evaluate_condition_sync` function
- Implement same logic as _evaluate_condition
- Ensure consistency between both implementations

## DEFINITION OF DONE
- [ ] All comparison operators work correctly
- [ ] Logical operators (&&, ||, !) are supported
- [ ] Type-aware evaluation is implemented
- [ ] Both instances updated consistently
- [ ] Stub comments removed

## RESEARCH NOTES
- Review variable resolution in resolve_variables_sync
- Check for existing expression evaluation utilities
- Examine what condition formats are used in practice
- Look for operator precedence patterns

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

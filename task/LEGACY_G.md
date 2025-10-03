# LEGACY_G: GraphQL Legacy Wrapper Removal

## OBJECTIVE
Remove legacy GraphQL conversion wrapper function that just delegates to newer implementation.

## SCOPE
File: `packages/sweetmcp/packages/pingora/src/normalize/parsers.rs`

## SUBTASK 1: Remove legacy wrapper function
**Location:** Lines 1332-1336

Delete:
```rust
// legacy marker comment
fn graphql_value_to_json(value: &async_graphql_value::Value) -> ConversionResult<Value> {
    // Use empty variables context for backward compatibility
    graphql_value_to_json_with_variables(value, &Value::Object(serde_json::Map::new()))
}
```

## SUBTASK 2: Find all calls to legacy wrapper

```bash
grep -rn "graphql_value_to_json(" packages/sweetmcp/packages/pingora/src
```

Exclude calls to `graphql_value_to_json_with_variables` (the new function)

## SUBTASK 3: Update call sites

For each call to `graphql_value_to_json(value)`:

**OPTION 1 (Simple replacement):**
```rust
// BEFORE:
graphql_value_to_json(value)

// AFTER:
graphql_value_to_json_with_variables(value, &Value::Object(serde_json::Map::new()))
```

**OPTION 2 (Better - determine if variables exist):**
```rust
// If actual variables are available in context:
graphql_value_to_json_with_variables(value, &actual_variables)
```

Review each call site to determine if actual GraphQL variables should be passed.

## VALIDATION COMMANDS
```bash
# Verify legacy function removed
grep -n "^fn graphql_value_to_json(" packages/sweetmcp/packages/pingora/src/normalize/parsers.rs
# Expected: 0 results

# Verify no calls to removed function
grep -rn "graphql_value_to_json(" packages/sweetmcp/packages/pingora/src | grep -v "graphql_value_to_json_with_variables"
# Expected: 0 results

# Verify compilation
cargo check -p sweetmcp-pingora
```

## DEFINITION OF DONE
- ✅ Legacy wrapper function deleted
- ✅ All call sites updated to use new API
- ✅ Variable contexts properly passed where available
- ✅ Code compiles without errors

## EXECUTION ORDER
**Task 7 of 8** - Independent, can execute anytime after LEGACY_E

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on wrapper removal and call site updates

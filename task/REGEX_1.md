# REGEX_1: Add Regex Pattern Validation to Formatting Rules

## OBJECTIVE

Implement regex pattern syntax validation in the formatting rules validator to prevent runtime panics from invalid regex patterns.

## BACKGROUND

The `FormattingRule::validate()` method in `packages/candle/src/domain/chat/formatting/options.rs` currently validates that rule names and patterns are non-empty, but does not validate that the pattern is a valid regex. This means invalid regex patterns will cause runtime panics when formatting rules are applied.

The codebase already has excellent regex validation patterns in `packages/candle/src/domain/chat/commands/validation/parameter_validators.rs` (line 169) that can be followed.

## CONSTRAINTS

- **NO TESTS**: Do not write unit tests, integration tests, or test code. Another team handles testing.
- **NO BENCHMARKS**: Do not write benchmark code. Another team handles performance testing.
- **FOCUS**: Only modify `./src` files to implement the feature.

## SUBTASK 1: Review Existing Validation Pattern

**Location**: `packages/candle/src/domain/chat/commands/validation/parameter_validators.rs` (line 169)

**What to find**: The existing regex validation pattern that compiles a regex and converts errors to appropriate error types.

**Expected pattern**:
```rust
Regex::new(&pattern).map_err(|e| SomeError::ConfigurationError {
    detail: format!("Invalid regex pattern '{}': {}", pattern, e),
})?;
```

## SUBTASK 2: Implement Regex Validation

**Location**: `packages/candle/src/domain/chat/formatting/options.rs` (line 396)

**Current code**:
```rust
pub fn validate(&self) -> FormatResult<()> {
    if self.name.is_empty() {
        return Err(FormatError::ConfigurationError {
            detail: "Rule name cannot be empty".to_string(),
        });
    }
    if self.pattern.is_empty() {
        return Err(FormatError::ConfigurationError {
            detail: "Rule pattern cannot be empty".to_string(),
        });
    }
    // TODO: Validate regex pattern syntax
    Ok(())
}
```

**What to change**:
1. Replace the TODO comment with actual regex validation
2. Use `Regex::new(&self.pattern)` to compile the pattern
3. Convert compilation errors to `FormatError::ConfigurationError`
4. Include both the pattern and the regex error in the error message

**Expected implementation**:
```rust
// After the pattern.is_empty() check, add:
Regex::new(&self.pattern).map_err(|e| FormatError::ConfigurationError {
    detail: format!("Invalid regex pattern '{}': {}", self.pattern, e),
})?;
Ok(())
```

## SUBTASK 3: Verify Compilation

**Commands**:
```bash
cargo check -p paraphym_candle
cargo clippy -p paraphym_candle
cargo fmt -p paraphym_candle
```

**What to verify**:
- Code compiles without errors
- No new clippy warnings
- TODO comment is removed
- Error messages are descriptive

## DEFINITION OF DONE

- [ ] `Regex::new()` is called on `self.pattern`
- [ ] Compilation errors are caught and converted to `FormatError::ConfigurationError`
- [ ] Error message includes both the pattern and the regex error details
- [ ] TODO comment is removed
- [ ] Code compiles without errors
- [ ] No new clippy warnings

## WHY THIS MATTERS

Without validation, invalid regex patterns will cause runtime panics when formatting rules are applied. This is a production safety issue that needs to be caught at configuration time, not runtime.

## REFERENCE FILES

- **Pattern to follow**: `packages/candle/src/domain/chat/commands/validation/parameter_validators.rs` (line 169)
- **File to modify**: `packages/candle/src/domain/chat/formatting/options.rs` (line 396)

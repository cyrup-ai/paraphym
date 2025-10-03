# LEGACY_C: Deprecated Methods in CandleUsage

## OBJECTIVE
Remove 3 deprecated methods from CandleUsage that exist only for "backward compatibility" in an UNRELEASED library.

## SCOPE
File: `packages/candle/src/domain/model/usage.rs`

## SUBTASK 1: Remove prompt_tokens() method
**Location:** Lines ~54-63

Delete:
```rust
#[inline]
#[deprecated(
    since = "0.1.0",
    note = "Use `input_tokens` instead for HTTP3 API standardization"
)]
#[must_use]
pub const fn prompt_tokens(&self) -> u32 {
    self.input_tokens
}
```

**Migration:**
```bash
# Find usages
grep -rn "\.prompt_tokens()" packages/candle/src

# Replace: .prompt_tokens() → .input_tokens
```

## SUBTASK 2: Remove completion_tokens() method
**Location:** Lines ~65-74

Delete:
```rust
#[inline]
#[deprecated(
    since = "0.1.0",
    note = "Use `output_tokens` instead for HTTP3 API standardization"
)]
#[must_use]
pub const fn completion_tokens(&self) -> u32 {
    self.output_tokens
}
```

**Migration:**
```bash
# Find usages
grep -rn "\.completion_tokens()" packages/candle/src

# Replace: .completion_tokens() → .output_tokens
```

## SUBTASK 3: Remove from_legacy() constructor
**Location:** Lines ~76-85

Delete:
```rust
#[inline]
#[deprecated(
    since = "0.1.0",
    note = "Use `new` instead for standardized API"
)]
pub fn from_legacy(prompt_tokens: u32, completion_tokens: u32) -> Self {
    Self::new(prompt_tokens, completion_tokens)
}
```

**Migration:**
```bash
# Find usages
grep -rn "from_legacy(" packages/candle/src

# Replace: CandleUsage::from_legacy(a, b) → CandleUsage::new(a, b)
```

## VALIDATION COMMANDS
```bash
# Verify no deprecated attributes remain in usage.rs
grep -n "#\[deprecated" packages/candle/src/domain/model/usage.rs

# Verify no calls to removed methods
grep -rn "\.prompt_tokens()\|\.completion_tokens()\|from_legacy(" packages/candle/src

# Verify compilation
cargo check -p paraphym_candle
```

## DEFINITION OF DONE
- ✅ All 3 deprecated methods removed from usage.rs
- ✅ All call sites updated to use new API
- ✅ Code compiles without errors
- ✅ No #[deprecated] attributes in usage.rs

## EXECUTION ORDER
**Task 2 of 8** - Execute early (update call sites before removing type aliases)

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on method removal and call site updates

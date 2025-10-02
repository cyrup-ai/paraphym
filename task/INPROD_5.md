# INPROD_5: Fix Operator Associativity Bug in Template Parser

## SEVERITY: HIGH

## STATUS: INCOMPLETE - CRITICAL BUG IN IMPLEMENTATION

## Core Objective

Fix left-associativity for binary operators (-, /, %) in the template parser to ensure expressions evaluate correctly from left to right.

## The Problem

The template parser incorrectly handles operator associativity. When parsing expressions like `10 - 3 - 2`, the parser must build an AST that evaluates as `(10 - 3) - 2 = 5`, not `10 - (3 - 2) = 9`.

### Current Bug Location

File: [packages/candle/src/domain/chat/templates/parser.rs](../packages/candle/src/domain/chat/templates/parser.rs)

The `find_operator()` function was correctly updated to return the **rightmost** operator position (line 570-594), but the recursive parsing calls were NOT updated to match this strategy.

## How Left-Associativity Works

For left-associative operators, when we find the rightmost operator:
- Everything to the LEFT should be parsed at the SAME precedence level (to allow chaining)
- Everything to the RIGHT should be parsed at a HIGHER precedence level (to respect precedence)

Example: `10 - 3 - 2`
1. Find rightmost "-" at position 6
2. Split: left="10 - 3", right="2"
3. Parse left with `parse_additive` (same level) → finds "-" at pos 3, recursively builds `(10 - 3)`
4. Parse right with `parse_multiplicative` (higher level) → returns `2`
5. Result: `Expression{-, [(10 - 3), 2]}` → evaluates to 5

## Required Code Changes

### 1. Fix `parse_additive` method (Line 513-522)

**CURRENT BROKEN CODE:**
```rust
fn parse_additive(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
    if let Some(pos) = Self::find_operator(content, &["+", "-"]) {
        let (left_str, op, right_str) = Self::extract_operator(content, pos, &["+", "-"])?;
        let left = self.parse_multiplicative(&left_str, depth)?;  // ❌ WRONG
        let right = self.parse_additive(&right_str, depth)?;      // ❌ WRONG
        return Ok(TemplateAst::Expression {
            operator: op,
            operands: Arc::new([left, right]),
        });
    }
```

**REQUIRED FIX:**
```rust
fn parse_additive(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
    if let Some(pos) = Self::find_operator(content, &["+", "-"]) {
        let (left_str, op, right_str) = Self::extract_operator(content, pos, &["+", "-"])?;
        let left = self.parse_additive(&left_str, depth)?;        // ✅ SAME precedence
        let right = self.parse_multiplicative(&right_str, depth)?; // ✅ HIGHER precedence
        return Ok(TemplateAst::Expression {
            operator: op,
            operands: Arc::new([left, right]),
        });
    }
```

### 2. Fix `parse_multiplicative` method (Line 526-535)

**CURRENT BROKEN CODE:**
```rust
fn parse_multiplicative(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
    if let Some(pos) = Self::find_operator(content, &["*", "/", "%"]) {
        let (left_str, op, right_str) = Self::extract_operator(content, pos, &["*", "/", "%"])?;
        let left = self.parse_primary(&left_str, depth)?;         // ❌ WRONG
        let right = self.parse_multiplicative(&right_str, depth)?; // ❌ WRONG
        return Ok(TemplateAst::Expression {
            operator: op,
            operands: Arc::new([left, right]),
        });
    }
```

**REQUIRED FIX:**
```rust
fn parse_multiplicative(&self, content: &str, depth: usize) -> TemplateResult<TemplateAst> {
    if let Some(pos) = Self::find_operator(content, &["*", "/", "%"]) {
        let (left_str, op, right_str) = Self::extract_operator(content, pos, &["*", "/", "%"])?;
        let left = self.parse_multiplicative(&left_str, depth)?;  // ✅ SAME precedence
        let right = self.parse_primary(&right_str, depth)?;       // ✅ HIGHER precedence
        return Ok(TemplateAst::Expression {
            operator: op,
            operands: Arc::new([left, right]),
        });
    }
```

## Why This Fix Works

The recursive descent parser uses the following precedence hierarchy:
1. `parse_expression` → handles comparisons (lowest precedence)
2. `parse_additive` → handles +, - (medium precedence)
3. `parse_multiplicative` → handles *, /, % (higher precedence)
4. `parse_primary` → handles literals, variables, parentheses (highest precedence)

By finding the rightmost operator and recursively calling:
- The SAME precedence level on the left side
- A HIGHER precedence level on the right side

We ensure left-to-right evaluation while respecting operator precedence.

## Implementation Pattern Already in Use

The template system already uses similar recursive descent patterns in:
- [packages/candle/src/domain/chat/templates/core.rs](../packages/candle/src/domain/chat/templates/core.rs) - Line 445-550: `evaluate_expression` method recursively evaluates the AST
- [packages/candle/src/domain/chat/templates/compiler.rs](../packages/candle/src/domain/chat/templates/compiler.rs) - Line 54-75: Recursive AST compilation

## Definition of Done

The implementation is complete when:

1. Line 515 in parser.rs calls:
   - `self.parse_additive(&left_str, depth)` for left operand
   - `self.parse_multiplicative(&right_str, depth)` for right operand

2. Line 528 in parser.rs calls:
   - `self.parse_multiplicative(&left_str, depth)` for left operand
   - `self.parse_primary(&right_str, depth)` for right operand

3. The following expressions parse and evaluate correctly:
   - `{{ 10 - 3 - 2 }}` → builds AST: `Expr(-, [Expr(-, [10, 3]), 2])` → evaluates to "5"
   - `{{ 20 / 4 / 2 }}` → builds AST: `Expr(/, [Expr(/, [20, 4]), 2])` → evaluates to "2.5"
   - `{{ 17 % 5 % 2 }}` → builds AST: `Expr(%, [Expr(%, [17, 5]), 2])` → evaluates to "0"

## Verification Steps

After making the changes:

1. Build the project: `cargo build -p paraphym_candle`
2. Create a simple test file with template expressions
3. Run the template parser on expressions like "{{ 10 - 3 - 2 }}"
4. Verify the result is "5" not "9" or an error

## Additional Context

The `find_operator` function (lines 570-594) correctly returns the rightmost operator position. This is the foundation for left-associativity - we just need to update the recursive calls to match this strategy.

The expression evaluation happens in [core.rs:445-550](../packages/candle/src/domain/chat/templates/core.rs#L445-L550) where `evaluate_expression` recursively renders operands and applies the operator. The fix ensures the correct AST structure is built during parsing.
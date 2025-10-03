# LEGACY_A: Parser Consolidation

## OBJECTIVE
Replace old parser.rs with the improved parser_new.rs implementation. The "new" parser contains genuine quality improvements that were abandoned mid-refactor.

## BACKGROUND
Analysis revealed parser_new.rs is NOT dead code - it's an IMPROVED implementation with 214 lines of changes including:
- Better Rust idioms: `strip_prefix()` instead of `starts_with()` + manual slicing
- Cleaner pattern matching with closures and `matches!()` macro
- Better method design: `&self` instead of associated function for `find_block_end`
- Improved control flow with better-structured else blocks
- Reordered parsing logic: function calls before expressions (fixes edge cases)

Classic lazy development: someone improved the parser, then abandoned it without completing migration.

## SUBTASK 1: Replace parser.rs with improved version

**Current State:**
- `packages/candle/src/domain/chat/templates/parser.rs` (901 lines - OLD)
- `packages/candle/src/domain/chat/templates/parser_new.rs` (845 lines - IMPROVED)

**Action:**
```bash
mv packages/candle/src/domain/chat/templates/parser_new.rs \
   packages/candle/src/domain/chat/templates/parser.rs
```

**Why:** mod.rs already imports `parser` (line 13), so this is a drop-in replacement.

## SUBTASK 2: Verify consolidation

```bash
# Confirm parser_new.rs is gone
test ! -f packages/candle/src/domain/chat/templates/parser_new.rs && echo "✅ Consolidated"

# Verify compilation
cargo check -p paraphym_candle
```

## DEFINITION OF DONE
- ✅ parser_new.rs no longer exists
- ✅ parser.rs contains the improved implementation
- ✅ Code compiles without errors
- ✅ Single canonical parser implementation

## EXECUTION ORDER
**Task 1 of 8** - Execute this first (safe, no dependencies)

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on file consolidation

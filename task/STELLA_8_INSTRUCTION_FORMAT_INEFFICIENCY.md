# Task: Eliminate Unnecessary String Allocations in STELLA Instruction Formatting

## Severity: LOW
**Impact**: Minor memory churn, ~1-2% overhead on single-text embeddings

## Core Objective

Eliminate unnecessary `Vec<String>` allocation and `String::clone()` when formatting a single text for STELLA embedding by adopting the proven pattern already used in NVEmbed.

## Problem Analysis

### Current Inefficient Pattern

**Location**: [`packages/candle/src/capability/text_embedding/stella/loaded.rs:179`](../../packages/candle/src/capability/text_embedding/stella/loaded.rs)

```rust
// Current inefficient code (line 179)
let formatted_text = format_with_instruction(&[&text], task.as_deref())[0].clone();
```

**What happens:**
1. Creates a slice `&[&text]` with one element
2. Calls `format_with_instruction()` which:
   - Iterates and formats each text
   - Collects into `Vec<String>` (heap allocation)
3. Indexes `[0]` to get first element
4. Clones the String (another allocation)
5. Drops the Vec

**Wasted allocations per single embedding:**
- Vec allocation (~24 bytes on 64-bit + heap overhead)
- String clone (full text duplication)

### Existing Solution Pattern in Codebase

**NVEmbed already implements the optimal pattern**: [`packages/candle/src/capability/text_embedding/nvembed/instruction.rs:15-36`](../../packages/candle/src/capability/text_embedding/nvembed/instruction.rs)

```rust
/// Format text with task-specific instruction prefix for NVEmbed v2
#[inline]
pub(crate) fn format_with_instruction(text: &str, task: Option<&str>) -> String {
    match task {
        Some("search_query") => format!(
            "Instruct: Given a web search query...\nQuery: {}",
            text
        ),
        Some("search_document") => format!(
            "Instruct: Given a web search query...\nPassage: {}",
            text
        ),
        // ... other cases ...
        _ => text.to_string(),
    }
}
```

**Why this is superior:**
- Takes single `&str` as input
- Returns `String` directly
- Zero unnecessary allocations
- Simple, clear API

## Required Changes

### File 1: `packages/candle/src/capability/text_embedding/stella/instruction.rs`

**Current structure:**
- Single public function: `format_with_instruction(&[&str], Option<&str>) -> Vec<String>`
- Task validation with logging
- Instruction matching via long match statement

**Required refactoring:**

#### Step 1: Extract instruction matching logic into private helper

```rust
/// Get the instruction string for a given task (or default)
fn get_instruction(task: Option<&str>) -> &'static str {
    // Validate task parameter and warn if invalid
    if let Some(t) = task {
        if !VALID_TASKS.contains(&t) {
            log::warn!(
                "Unknown embedding task '{}'. Using default 's2p'. Valid tasks: {}",
                t,
                VALID_TASKS.join(", ")
            );
        }
    }
    
    match task {
        Some("s2p") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        }
        Some("s2s") => "Retrieve semantically similar text.",
        Some("search_query") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        }
        Some("search_document") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        }
        Some("classification") => "Retrieve semantically similar text.",
        Some("clustering") => "Retrieve semantically similar text.",
        Some("retrieval") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        }
        _ => "Given a web search query, retrieve relevant passages that answer the query.",
    }
}
```

**Location in file**: Add after `VALID_TASKS` constant (around line 10)

#### Step 2: Add new single-text formatting function

```rust
/// Format a single text with task-specific instruction prefix
///
/// Optimized for single-text embeddings - avoids Vec allocation.
/// For batch operations, use `format_with_instruction()` instead.
///
/// # Task Types
/// - `"s2p"`, `"search_query"`, `"search_document"`, or `"retrieval"`: Search query → passage retrieval
/// - `"s2s"`, `"classification"`, or `"clustering"`: Semantic similarity  
/// - `None`: Defaults to search query mode (`"s2p"`)
///
/// # Validation
/// Invalid tasks trigger a warning and fall back to default `"s2p"` instruction.
///
/// # Examples
/// ```ignore
/// let formatted = format_single_with_instruction("What is Rust?", Some("search_query"));
/// // Returns: "Instruct: Given a web search query...\nQuery: What is Rust?"
/// ```
#[inline]
pub(crate) fn format_single_with_instruction(text: &str, task: Option<&str>) -> String {
    let instruct = get_instruction(task);
    format!("Instruct: {}\nQuery: {}", instruct, text)
}
```

**Location in file**: Add after `get_instruction()` helper (around line 70)

#### Step 3: Refactor existing batch function to use helper

```rust
/// Format multiple texts with task-specific instruction prefix
///
/// For single-text embeddings, prefer `format_single_with_instruction()` to avoid Vec allocation.
///
/// # Task Types
/// Same as `format_single_with_instruction()`
///
/// # Examples
/// ```ignore
/// let texts = vec!["What is Rust?", "How does async work?"];
/// let formatted = format_with_instruction(&texts, Some("search_query"));
/// ```
pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    let instruct = get_instruction(task);
    texts
        .iter()
        .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
        .collect()
}
```

**Location in file**: Replace existing function starting at line 32

### File 2: `packages/candle/src/capability/text_embedding/stella/loaded.rs`

**Change 1**: Update import statement (line 4)

```rust
// Old
use super::instruction::format_with_instruction;

// New
use super::instruction::{format_single_with_instruction, format_with_instruction};
```

**Change 2**: Update single-text embedding call (line 179)

```rust
// Old (inefficient)
let formatted_text = format_with_instruction(&[&text], task.as_deref())[0].clone();

// New (optimized)
let formatted_text = format_single_with_instruction(&text, task.as_deref());
```

**Change 3**: Batch embedding call remains unchanged (line 264)

```rust
// Already optimal - no changes needed
let formatted_texts = format_with_instruction(&text_refs, task.as_deref());
```

## Implementation Checklist

### Phase 1: Refactor `instruction.rs`

- [ ] Extract `get_instruction()` helper function with validation logic
- [ ] Add `format_single_with_instruction()` for single-text case
- [ ] Refactor `format_with_instruction()` to use `get_instruction()` helper
- [ ] Verify all existing tests still pass (no test changes needed)

### Phase 2: Update `loaded.rs`

- [ ] Update import to include `format_single_with_instruction`
- [ ] Replace line 179 with call to `format_single_with_instruction()`
- [ ] Verify line 264 batch call remains unchanged
- [ ] Verify `embed()` method works correctly
- [ ] Verify `batch_embed()` method works correctly

### Phase 3: Code Quality

- [ ] Run `cargo clippy` on modified files
- [ ] Run `cargo fmt` on modified files
- [ ] Verify no breaking API changes (all public APIs unchanged)
- [ ] Verify backward compatibility maintained

## Definition of Done

✅ **Functional Requirements:**
- Single-text embeddings use `format_single_with_instruction()` (no Vec allocation)
- Batch embeddings continue using `format_with_instruction()` (unchanged behavior)
- All task validation and instruction matching logic preserved
- All existing tests pass without modification

✅ **Code Quality:**
- No code duplication (instruction logic extracted to helper)
- Clear separation of concerns (single vs batch operations)
- Follows existing patterns in codebase (matches NVEmbed approach)
- Inline documentation added for new functions

✅ **Performance:**
- Single-text embeddings: 1 fewer allocation (Vec eliminated)
- Single-text embeddings: 1 fewer String clone eliminated
- Batch embeddings: Zero performance change (same code path)

✅ **Maintainability:**
- No breaking changes to public API
- Backward compatible with all existing callers
- Easy to understand intent (function names self-documenting)
- Consistent with codebase patterns

## Impact Assessment

### Performance Improvement

For 1000 single-text embeddings:
- **Before**: 1000 Vec allocations + 1000 String clones
- **After**: 0 Vec allocations + 0 String clones
- **Saved**: ~24-48KB memory churn (Vec overhead) + full text duplication costs

Total time savings: ~1-2% of embedding time (minor but measurable)

### Risk Assessment

**Risk Level**: VERY LOW

**Why:**
- No changes to core embedding logic
- No changes to instruction text or formatting
- No changes to public APIs
- Only optimization of allocation pattern
- Proven pattern already used in NVEmbed

### Backward Compatibility

**Guaranteed**: Yes

- Existing batch function signature unchanged
- Existing behavior preserved
- Only adds new optimized path for single-text case
- All existing callers continue to work

## Code References

### Files Modified (2)

1. **[`packages/candle/src/capability/text_embedding/stella/instruction.rs`](../../packages/candle/src/capability/text_embedding/stella/instruction.rs)**
   - Lines 10-70: Add `get_instruction()` helper
   - Lines 70-85: Add `format_single_with_instruction()`
   - Lines 32-66: Refactor `format_with_instruction()` to use helper

2. **[`packages/candle/src/capability/text_embedding/stella/loaded.rs`](../../packages/candle/src/capability/text_embedding/stella/loaded.rs)**
   - Line 4: Update imports
   - Line 179: Replace with `format_single_with_instruction()`

### Reference Implementations

**Pattern to follow**: [`packages/candle/src/capability/text_embedding/nvembed/instruction.rs`](../../packages/candle/src/capability/text_embedding/nvembed/instruction.rs)
- Already implements single-text `format_with_instruction()` returning `String`
- Proven pattern used in production

**Related code**: [`packages/candle/src/capability/text_embedding/gte_qwen/instruction.rs`](../../packages/candle/src/capability/text_embedding/gte_qwen/instruction.rs)
- Different approach (inline formatting in forward pass)
- Not applicable here due to different architecture

## Implementation Notes

### Why Not Use Iterator Instead?

Option 2 in the original analysis suggested returning an iterator:

```rust
pub(crate) fn format_with_instruction<'a>(
    texts: &'a [&'a str],
    task: Option<&str>,
) -> impl Iterator<Item = String> + 'a
```

**Rejected because:**
- More complex type signatures
- Harder to use for callers  
- Doesn't match existing codebase patterns
- NVEmbed's approach is simpler and proven

### Why Extract get_instruction() Helper?

**DRY Principle**: Instruction matching logic appears in both single and batch functions

**Benefits:**
- Single source of truth for task validation
- Single source of truth for instruction text
- Easier to maintain (change instruction text in one place)
- Smaller functions (easier to test and understand)

### Why Keep Separate Functions?

**Type Safety**: `&str` vs `&[&str]` clearly communicate intent

**Performance**: Compiler can better optimize separate paths

**Clarity**: Function names self-document usage (`format_single_` vs `format_with_`)

**Consistency**: Matches established patterns in similar codebases

## Testing Strategy

**No new tests required** - this is a refactoring with zero behavior changes.

**Existing tests verify correctness:**
- `test_valid_tasks_no_warning` - validates all task types work
- `test_none_task_uses_default` - validates default behavior
- `test_invalid_task_warning` - validates error handling
- `test_multiple_texts` - validates batch function
- All tests in `loaded.rs` - validates integration

**Manual verification:**
- Run `cargo test --package candle --lib capability::text_embedding::stella::instruction`
- Run integration tests that use STELLA embeddings
- Verify single and batch embeddings produce identical results

## Execution Plan

### Time Estimate: 15-20 minutes

1. **Refactor instruction.rs** (10 min)
   - Extract get_instruction() helper
   - Add format_single_with_instruction()  
   - Refactor format_with_instruction()

2. **Update loaded.rs** (3 min)
   - Update imports
   - Change line 179 to use new function

3. **Verify** (5 min)
   - Run cargo check
   - Run cargo clippy
   - Run cargo test
   - Confirm no regressions

### Success Criteria

✅ All existing tests pass  
✅ `cargo clippy` shows no new warnings  
✅ `cargo fmt` applied successfully  
✅ Code compiles without errors  
✅ Single-text embeddings produce identical results  
✅ Batch embeddings produce identical results  

---

**Task Status**: Ready for implementation  
**Complexity**: Low  
**Risk**: Very Low  
**Priority**: Low (optimization, not critical)

# Issue: Instruction Formatting Creates Unnecessary String Allocations

## Severity: LOW
**Impact**: Minor memory churn, ~1-2% overhead

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/instruction.rs:24-27`

## Problem Description

The `format_with_instruction` function creates a new `Vec<String>` and then immediately extracts the first element:

```rust
// In loaded.rs:234
let formatted_text = format_with_instruction(&[&text], task.as_deref())[0].clone();
```

```rust
// In instruction.rs:24-27
texts
    .iter()
    .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
    .collect()  // ← Creates Vec<String>
```

## Inefficiency

For single-text embedding:
1. Create slice `&[&text]` with one element
2. Call `format_with_instruction` which:
   - Iterates over the slice
   - Formats each text
   - Collects into `Vec<String>`
3. Index into Vec `[0]`
4. Clone the String
5. Drop the Vec

**Wasted allocations**: Vec allocation + String clone

## Better Approach

### Option 1: Separate Single/Batch Functions

```rust
// For single text
pub(crate) fn format_single_with_instruction(text: &str, task: Option<&str>) -> String {
    let instruct = get_instruction(task);
    format!("Instruct: {}\nQuery: {}", instruct, text)
}

// For batch
pub(crate) fn format_batch_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    let instruct = get_instruction(task);
    texts
        .iter()
        .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
        .collect()
}

fn get_instruction(task: Option<&str>) -> &'static str {
    match task {
        Some("s2p") => "Given a web search query, retrieve relevant passages that answer the query.",
        // ... rest of matches
    }
}
```

**Pros**:
- No unnecessary Vec allocation for single text
- Clear API
- Slightly faster

**Cons**:
- Code duplication (instruction matching)
- Two functions to maintain

### Option 2: Return Iterator

```rust
pub(crate) fn format_with_instruction<'a>(
    texts: &'a [&'a str],
    task: Option<&str>,
) -> impl Iterator<Item = String> + 'a {
    let instruct = get_instruction(task);
    texts
        .iter()
        .map(move |text| format!("Instruct: {}\nQuery: {}", instruct, text))
}

// Usage:
let formatted_text = format_with_instruction(&[&text], task.as_deref())
    .next()
    .unwrap();
```

**Pros**:
- Lazy evaluation
- No Vec allocation
- Single function

**Cons**:
- More complex type signature
- Caller must handle iterator

## Current Impact

For 1000 single-text embeddings:
- Wasted: 1000 Vec allocations (~24KB each on 64-bit)
- Wasted: 1000 String clones
- Total overhead: ~1-2% of total time

**Not critical** but could be optimized.

## Recommendation

**Option 1** - clearer API and measurable (small) performance gain.

Alternatively, **keep as-is** since the overhead is minimal and the current code is simple and readable.

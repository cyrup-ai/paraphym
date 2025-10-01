# TASK: Algorithm - Dominant Color O(n) Optimization

**PRIORITY:** HIGH  
**ESTIMATED TIME:** Single session  
**SCOPE:** Replace O(n²) algorithm with O(n) count-based approach

---

## OBJECTIVE

Optimize the dominant color selection algorithm from O(n²) nested iteration to O(n) single-pass counting. Currently uses inefficient nested filtering that scans the array multiple times.

---

## CONTEXT

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Lines:** 95-98

**Current Problem:**
```rust
let dominant_color = *column_colors
    .iter()
    .max_by_key(|&&c| column_colors.iter().filter(|&&x| x == c).count())
    .unwrap_or(&0);
```

**Algorithm Complexity:**
- Outer iteration: 6 elements
- Inner iteration for each: 6 elements (count filter)
- Total comparisons: 6 × 6 = 36 per column
- For 800×600 image: ~800 columns × 100 sixel rows = 80,000 columns
- Total operations: 80,000 × 36 = 2.88 million unnecessary comparisons

---

## SUBTASKS

### SUBTASK 1: Analyze Current Algorithm

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Lines:** 95-98

**Current implementation breakdown:**
```rust
// For EACH color c in column_colors:
//   - Scan ALL of column_colors again
//   - Count how many match c
//   - Track maximum count
// Return the color with max count
```

**Example with column_colors = [2, 2, 2, 5, 5, 0]:**
1. Element 0 (value=2): Scan all 6, count 2's → 3
2. Element 1 (value=2): Scan all 6, count 2's → 3
3. Element 2 (value=2): Scan all 6, count 2's → 3
4. Element 3 (value=5): Scan all 6, count 5's → 2
5. Element 4 (value=5): Scan all 6, count 5's → 2
6. Element 5 (value=0): Scan all 6, count 0's → 1

Total: 36 comparisons to determine color 2 is dominant

**Why this is inefficient:** Same counts computed multiple times

---

### SUBTASK 2: Implement O(n) Count Array Algorithm

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Lines:** Replace lines 95-98

**Remove:**
```rust
let dominant_color = *column_colors
    .iter()
    .max_by_key(|&&c| column_colors.iter().filter(|&&x| x == c).count())
    .unwrap_or(&0);
```

**Replace with:**
```rust
let dominant_color = {
    // Count occurrences of each color (max 16 palette colors)
    let mut counts = [0u8; 16];
    for &color in &column_colors[..] {
        // Bounds check: only count valid palette indices
        if (color as usize) < 16 {
            counts[color as usize] += 1;
        }
    }
    
    // Find color with maximum count
    counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, &count)| count)
        .map(|(idx, _)| idx as u16)
        .unwrap_or(0)
};
```

**What this does:**
1. Create count array for 16 palette colors (all zeros)
2. Single pass: count each color's occurrences
3. Single pass: find index with maximum count
4. Return that index as dominant color

**Algorithm complexity:**
- Pass 1: 6 iterations (count colors)
- Pass 2: 16 iterations (find max)
- Total: 22 operations (vs 36 before)

---

### SUBTASK 3: Handle Edge Cases

**Edge cases handled:**

1. **Empty array:**
   - `column_colors` is never empty (always size 6)
   - But if all colors invalid: `unwrap_or(0)` returns black

2. **Out of bounds colors:**
   - Check: `if (color as usize) < 16`
   - Invalid colors (>15) are ignored
   - Prevents panic from array access

3. **Tie (multiple colors with same count):**
   - `max_by_key` returns first maximum
   - Tie-breaking: lower palette index wins
   - Consistent with original behavior

4. **All zeros (no colors counted):**
   - All counts remain 0
   - First index (0 = black) has count 0
   - Returns 0 as default

---

## RESEARCH NOTES

### Algorithm Comparison

**O(n²) Approach (Original):**
```
For each element:
    Count occurrences across entire array
Find element with max count
Time: O(n²) where n = array size
Space: O(1)
```

**O(n) Approach (New):**
```
Create count array[16]
For each element:
    Increment count[element]
Find max in count array
Time: O(n + m) where n = array size, m = palette size
Space: O(m) = O(16) = O(1)
```

### Performance Impact
- **Before:** 36 comparisons per column
- **After:** 22 operations per column
- **Speedup:** ~1.6x for dominant color selection
- **For 80,000 columns:** Saves ~1.12 million operations

### Correctness Proof
Both algorithms find the mode (most frequent element):

**Property:** Most frequent element has maximum count  
**Original:** Explicitly counts each element's frequency  
**New:** Same counting, just optimized storage

**Tie-breaking:**
- Original: `max_by_key` with iteration order
- New: `max_by_key` with enumeration order
- Both return first maximum found

---

## CONSTRAINTS

**CRITICAL:**
- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles performance testing)
- DO NOT change tie-breaking behavior
- DO NOT modify palette size (16 colors)
- Preserve exact functional behavior

**Requirements:**
- No `unwrap()` in src/ - VIOLATION! Use `unwrap_or(0)` instead
- No `expect()` in src/ or examples
- Bounds check required for array access
- Code must compile without errors

---

## VERIFICATION

### Compilation Check
```bash
cargo check -p rio-ext-test
```

### Algorithm Verification
Test cases to mentally verify:

1. **All same color:** [5,5,5,5,5,5] → 5 ✓
2. **Clear majority:** [1,2,2,2,3,4] → 2 ✓
3. **Tie:** [1,1,2,2,3,4] → 1 (first max) ✓
4. **Out of bounds:** [1,1,99,2,2,2] → 2 (99 ignored) ✓
5. **Default:** [16,17,18,19,20,21] → 0 (all invalid) ✓

### Code Review Checklist
- [ ] Count array created: `let mut counts = [0u8; 16];`
- [ ] Bounds check: `if (color as usize) < 16`
- [ ] Single count pass over column_colors
- [ ] Single max pass over counts array
- [ ] Uses `unwrap_or(0)` not `unwrap()`
- [ ] No nested iteration remaining
- [ ] Block expression used (braces around replacement)
- [ ] No compilation errors
- [ ] Complexity is O(n + m) not O(n²)

### Success Criteria
✅ O(n²) algorithm replaced with O(n)  
✅ Bounds checking added  
✅ Tie-breaking behavior preserved  
✅ No unwrap() used  
✅ Code compiles cleanly  
✅ Performance improved ~1.6x

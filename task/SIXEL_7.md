# TASK: Refactoring - Extract Helper Functions

**PRIORITY:** MEDIUM  
**ESTIMATED TIME:** Single session  
**SCOPE:** Decompose monolithic function into testable, maintainable helpers

---

## OBJECTIVE

Refactor the 160-line `encode_sixel` function by extracting helper functions for color matching and dominant color selection. This improves testability, readability, and maintainability by separating concerns.

---

## CONTEXT

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`

**Current Problem:**
- `encode_sixel` function is ~160 lines
- Contains lambda closures for complex logic
- Difficult to test individual components
- Mixing high-level orchestration with low-level algorithms

**Goals:**
1. Extract `find_closest_color` lambda → standalone function
2. Extract `dominant_color` logic → standalone function  
3. Simplify main function to orchestration only

---

## SUBTASKS

### SUBTASK 1: Extract Color Matching Helper

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Location:** Add before `encode_sixel` function

**Current code (lambda in encode_sixel):**
```rust
let find_closest_color = |r: u8, g: u8, b: u8| -> u16 {
    // Weighted Euclidean distance for perceptual color matching
    // Weights based on human eye sensitivity (ITU-R BT.601 standard)
    
    let mut min_dist = f32::MAX;
    let mut closest = 0;

    for (i, &(cr, cg, cb)) in PALETTE.iter().enumerate() {
        // Apply perceptual weights: 30% red, 59% green, 11% blue
        let dr = (r as f32 - cr as f32) * 0.30;
        let dg = (g as f32 - cg as f32) * 0.59;
        let db = (b as f32 - cb as f32) * 0.11;
        
        let dist = dr * dr + dg * dg + db * db;
        
        if dist < min_dist {
            min_dist = dist;
            closest = i;
        }
    }

    closest as u16
};
```

**Extract to standalone function:**
```rust
/// Find the closest palette color to given RGB using perceptual distance
/// 
/// Uses weighted Euclidean distance based on human eye sensitivity (ITU-R BT.601):
/// - Red: 30% weight
/// - Green: 59% weight (human eye most sensitive)
/// - Blue: 11% weight
/// 
/// # Arguments
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255) 
/// * `b` - Blue component (0-255)
/// * `palette` - Array of RGB tuples in (r,g,b) format
/// 
/// # Returns
/// Index of closest color in palette (0-15 for standard VT340 palette)
fn find_closest_color_in_palette(r: u8, g: u8, b: u8, palette: &[(i32, i32, i32)]) -> u16 {
    let mut min_dist = f32::MAX;
    let mut closest = 0;

    for (i, &(cr, cg, cb)) in palette.iter().enumerate() {
        // Apply perceptual weights: 30% red, 59% green, 11% blue
        let dr = (r as f32 - cr as f32) * 0.30;
        let dg = (g as f32 - cg as f32) * 0.59;
        let db = (b as f32 - cb as f32) * 0.11;
        
        let dist = dr * dr + dg * dg + db * db;
        
        if dist < min_dist {
            min_dist = dist;
            closest = i;
        }
    }

    closest as u16
}
```

**What to change in encode_sixel:**
- Remove lambda definition
- Replace: `find_closest_color(pixel[0], pixel[1], pixel[2])`
- With: `find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE)`

---

### SUBTASK 2: Extract Dominant Color Helper

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Location:** Add before `encode_sixel` function

**Current code (in encode_sixel, after SIXEL_4 changes):**
```rust
let dominant_color = {
    let mut counts = [0u8; 16];
    for &color in &column_colors[..] {
        if (color as usize) < 16 {
            counts[color as usize] += 1;
        }
    }
    
    counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, &count)| count)
        .map(|(idx, _)| idx as u16)
        .unwrap_or(0)
};
```

**Extract to standalone function:**
```rust
/// Find the most common color in a sixel column
/// 
/// Analyzes up to 6 pixels and returns the index of the most frequently
/// occurring color using O(n) count-based algorithm.
/// 
/// # Arguments
/// * `colors` - Array of color indices (0-15) from palette
/// 
/// # Returns
/// Index of dominant (most common) color. Returns 0 if array is empty
/// or all colors are out of bounds (>15).
/// 
/// # Algorithm
/// O(n + m) where n = array length, m = palette size (16)
/// - Single pass to count occurrences
/// - Single pass to find maximum
/// - Ties broken by first occurrence (lower index)
fn find_dominant_color(colors: &[u16]) -> u16 {
    let mut counts = [0u8; 16];
    
    // Count occurrences of each valid palette color
    for &color in colors {
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
}
```

**What to change in encode_sixel:**
- Remove block expression for dominant_color
- Replace: `let dominant_color = { ... }`
- With: `let dominant_color = find_dominant_color(&column_colors);`

---

### SUBTASK 3: Update encode_sixel to Use Helpers

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Function:** `encode_sixel`

**Changes required:**

1. **Remove lambda definition** (previously extracted to SUBTASK 1)

2. **Update color calculation calls:**
   ```rust
   // OLD:
   column_colors[i] = find_closest_color(pixel[0], pixel[1], pixel[2]);
   
   // NEW:
   column_colors[i] = find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE);
   ```

3. **Update dominant color calculation:**
   ```rust
   // OLD:
   let dominant_color = {
       let mut counts = [0u8; 16];
       // ... block expression ...
   };
   
   // NEW:
   let dominant_color = find_dominant_color(&column_colors);
   ```

**Result:**
- Main function focuses on orchestration
- Complex algorithms isolated in testable functions
- Each function has single responsibility

---

### SUBTASK 4: Verify Function Signatures

**Ensure compatibility:**

1. **find_closest_color_in_palette:**
   - Takes: `(u8, u8, u8, &[(i32, i32, i32)])`
   - Returns: `u16` (palette index)
   - Call site: `find_closest_color_in_palette(pixel[0], pixel[1], pixel[2], &PALETTE)`

2. **find_dominant_color:**
   - Takes: `&[u16]` (slice of color indices)
   - Returns: `u16` (dominant color index)
   - Call site: `find_dominant_color(&column_colors)`

3. **Both functions are private (not pub)**
   - Module-internal helpers
   - Not part of public API

---

## RESEARCH NOTES

### Benefits of Extraction

**Before (monolithic):**
- 160-line function
- Nested logic hard to follow
- Can't unit test individual algorithms
- Changes risk breaking unrelated parts

**After (decomposed):**
- ~80-line main function (orchestration)
- ~30-line color matching function
- ~20-line dominant color function
- Each testable independently
- Changes isolated by concern

### Separation of Concerns

**Color Matching (find_closest_color_in_palette):**
- Concern: Perceptual color distance calculation
- Input: RGB values + palette
- Output: Closest palette index
- Testable: Try various RGB inputs, verify correct index

**Color Selection (find_dominant_color):**
- Concern: Mode detection (most frequent element)
- Input: Array of color indices
- Output: Dominant index
- Testable: Try various distributions, verify correct mode

**Orchestration (encode_sixel):**
- Concern: Sixel format encoding
- Input: Image
- Output: Sixel string
- Uses helpers for algorithms

### Performance Impact
- Function call overhead: negligible (likely inlined by compiler)
- Algorithm unchanged: same performance
- Code clarity: greatly improved

### Testing Strategy (Future)
Once extracted, tests can target individual functions:

```rust
// Color matching tests (separate team will write)
#[test]
fn test_black_maps_to_black() {
    let idx = find_closest_color_in_palette(0, 0, 0, &PALETTE);
    assert_eq!(idx, 0);
}

// Dominant color tests
#[test]
fn test_all_same_returns_that_color() {
    let colors = [5u16; 6];
    let result = find_dominant_color(&colors);
    assert_eq!(result, 5);
}
```

---

## CONSTRAINTS

**CRITICAL:**
- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles performance testing)
- DO NOT change algorithm logic
- DO NOT modify function behavior
- Keep functions private (not pub)

**Requirements:**
- No `unwrap()` in src/ (project constraint)
- No `expect()` in src/ or examples  
- Extract functions must be module-private
- Behavior must be identical to original
- Code must compile without errors

---

## VERIFICATION

### Compilation Check
```bash
cargo check -p rio-ext-test
```

### Function Extraction Verification
```bash
# Verify helper functions exist
rg "fn find_closest_color_in_palette" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs
rg "fn find_dominant_color" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Verify no lambda in encode_sixel (should find none)
rg "let find_closest_color = \|" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Verify helpers are called
rg "find_closest_color_in_palette\(" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs
rg "find_dominant_color\(" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Verify functions are private (should find no 'pub fn find_')
rg "pub fn find_" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs
```

### Code Review Checklist
- [ ] `find_closest_color_in_palette` function extracted
- [ ] Takes `(u8, u8, u8, &[(i32, i32, i32)])` parameters
- [ ] Returns `u16` palette index
- [ ] Has documentation comments
- [ ] `find_dominant_color` function extracted
- [ ] Takes `&[u16]` parameter
- [ ] Returns `u16` dominant index
- [ ] Has documentation comments
- [ ] Both functions are private (not pub)
- [ ] encode_sixel calls helpers correctly
- [ ] No lambda definitions in encode_sixel
- [ ] Behavior identical to original
- [ ] No compilation errors

### Success Criteria
✅ Helper functions extracted successfully  
✅ Main function simplified to orchestration  
✅ Each function has single responsibility  
✅ Functions are module-private  
✅ Behavior preserved exactly  
✅ Code compiles cleanly  
✅ Maintainability greatly improved

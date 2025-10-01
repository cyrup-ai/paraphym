# TASK: Error Handling - Remove unwrap() & Add Validation

**PRIORITY:** MEDIUM  
**ESTIMATED TIME:** Single session  
**SCOPE:** Fix constraint violations and add edge case handling

---

## OBJECTIVE

Remove all `unwrap()` calls from src/ (project constraint violation) and add validation for edge cases like zero-dimension images. Ensures code follows safety guidelines and handles boundary conditions gracefully.

---

## CONTEXT

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`

**Current Problems:**

1. **unwrap() Violations (Lines 107, 135)**
   - Project constraint: "never use unwrap() in src/"
   - Currently: `char::from_u32(63 + last_sixel_value).unwrap()`
   - Technically safe (63-126 are all valid chars) but violates constraint

2. **No Edge Case Validation**
   - Zero-width or zero-height images not handled
   - Could produce invalid sixel sequences
   - No early return for degenerate cases

---

## SUBTASKS

### SUBTASK 1: Replace unwrap() at Line 107

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Line:** 107

**Current code:**
```rust
result.push(char::from_u32(63 + last_sixel_value).unwrap());
```

**Context:**
- `last_sixel_value` is 6-bit (0-63) from sixel encoding
- `63 + last_sixel_value` ranges from 63 to 126
- Char 63 = '?', Char 126 = '~'
- All values in this range are valid ASCII/Unicode
- `unwrap()` will never panic BUT violates project constraints

**Replace with:**
```rust
result.push(char::from_u32(63 + last_sixel_value).unwrap_or('?'));
```

**Why '?' as fallback:**
- Char 63 ('?') is the minimum sixel value (represents sixel_value = 0)
- Fallback maintains valid sixel structure
- Will never trigger in practice (all values 63-126 are valid)

---

### SUBTASK 2: Replace unwrap() at Line 135

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Line:** 135

**Current code:**
```rust
result.push(char::from_u32(63 + last_sixel_value).unwrap());
```

**Replace with:**
```rust
result.push(char::from_u32(63 + last_sixel_value).unwrap_or('?'));
```

**Same reasoning:**
- Identical situation to line 107
- Same safety invariant (63-126 range)
- Same fallback strategy ('?')

---

### SUBTASK 3: Add Zero-Dimension Validation

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Location:** Beginning of `encode_sixel` function (after line 3)

**Current start:**
```rust
pub fn encode_sixel(img: &image::RgbImage) -> String {
    // Start with DCS sequence + sixel + raster attributes with image dimensions
    let mut result = String::from("\x1BPq");
    // ... rest of function
}
```

**Add validation at start:**
```rust
pub fn encode_sixel(img: &image::RgbImage) -> String {
    // Handle edge case: empty images
    if img.width() == 0 || img.height() == 0 {
        // Return minimal valid sixel sequence
        return String::from("\x1BPq\x1B\\");
    }
    
    // Start with DCS sequence + sixel + raster attributes with image dimensions
    let mut result = String::from("\x1BPq");
    // ... rest of function
}
```

**What this does:**
- Checks for zero-dimension images before processing
- Returns minimal valid sixel: `\x1BPq\x1B\\`
  - `\x1BPq` = DCS + sixel introducer
  - `\x1B\\` = String Terminator (ST)
- Prevents potential issues with:
  - Division/modulo by zero
  - Empty iteration ranges
  - Invalid raster attributes

---

### SUBTASK 4: Verify Safety Invariants

**Document the safety reasoning:**

1. **char::from_u32 safety (lines 107, 135):**
   ```
   last_sixel_value ∈ [0, 63]  (6-bit value, proven by: |= 1 << i where i ∈ [0,5])
   63 + last_sixel_value ∈ [63, 126]
   All values [63, 126] are valid Unicode scalar values
   Therefore: from_u32 will always return Some(char)
   unwrap_or('?') is defensive programming, fallback never triggers
   ```

2. **Image dimension safety:**
   ```
   Empty images (0×0, 0×h, w×0) produce no pixels
   Without validation: could iterate zero times or access invalid data
   With validation: early return with valid minimal sixel
   ```

---

## RESEARCH NOTES

### Sixel Character Encoding
- Sixel uses chars 63-126 (? to ~) to represent pixel patterns
- Each char encodes 6 vertical pixels as bits
- Formula: `char = 63 + sixel_value` where sixel_value ∈ [0,63]
- Range 63-126 is guaranteed valid ASCII/Unicode

### Minimal Valid Sixel
```
\x1BPq      DCS introducer + 'q' (sixel mode)
\x1B\\      String Terminator
```
This is the shortest valid sixel sequence - represents an empty image.

### Project Constraints
From project documentation:
- "never use unwrap() (period!)" in src/
- "never use expect()" in src/ or examples
- Use expect() only in tests/
- Prefer Result or defensive fallbacks

### Alternative Approaches Considered

**Option 1: Remove from_u32, use direct arithmetic**
```rust
result.push((63 + last_sixel_value) as u8 as char);
```
❌ Unsafe: Could produce invalid chars if sixel_value > 63

**Option 2: Use debug_assert + unwrap_or**
```rust
debug_assert!(last_sixel_value <= 63);
result.push(char::from_u32(63 + last_sixel_value).unwrap_or('?'));
```
✅ Good: Documents invariant and satisfies constraint

**Option 3: Use expect() with message**
```rust
result.push(char::from_u32(63 + last_sixel_value)
    .expect("sixel value must be 0-63"));
```
❌ Violates constraint: no expect() in src/

**Chosen: unwrap_or with safe fallback**
✅ Satisfies constraint, defensive, maintains invariant

---

## CONSTRAINTS

**CRITICAL:**
- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles performance testing)
- DO NOT use unwrap() anywhere in src/
- DO NOT use expect() in src/ or examples
- Must handle zero-dimension images

**Requirements:**
- Replace ALL unwrap() with unwrap_or()
- Add validation for edge cases
- Maintain valid sixel output
- Code must compile without errors

---

## VERIFICATION

### Compilation Check
```bash
cargo check -p rio-ext-test
```

### Constraint Verification
```bash
# Verify NO unwrap() in src/ (should return empty)
rg "unwrap\(\)" packages/sweetmcp/packages/sixel6vt/src/

# Verify unwrap_or used (should find 2 occurrences)
rg "unwrap_or\(" packages/sweetmcp/packages/sixel6vt/src/

# Verify NO expect() in src/ (should return empty)
rg "expect\(" packages/sweetmcp/packages/sixel6vt/src/
```

### Edge Case Testing (Manual)
You can temporarily test with:
```rust
let empty_0x0 = RgbImage::new(0, 0);
let result = encode_sixel(&empty_0x0);
assert_eq!(result, "\x1BPq\x1B\\");

let empty_0x100 = RgbImage::new(0, 100);
let result = encode_sixel(&empty_0x100);
assert_eq!(result, "\x1BPq\x1B\\");

let empty_100x0 = RgbImage::new(100, 0);
let result = encode_sixel(&empty_100x0);
assert_eq!(result, "\x1BPq\x1B\\");
```

### Code Review Checklist
- [ ] Line 107: Uses `unwrap_or('?')` not `unwrap()`
- [ ] Line 135: Uses `unwrap_or('?')` not `unwrap()`
- [ ] Start of function: Checks `width() == 0 || height() == 0`
- [ ] Empty images return `"\x1BPq\x1B\\"`
- [ ] No unwrap() anywhere in src/
- [ ] No expect() in src/
- [ ] Fallback char is '?' (63, minimum sixel)
- [ ] No compilation errors
- [ ] Clippy clean

### Success Criteria
✅ All unwrap() removed from src/  
✅ unwrap_or('?') used with safe fallback  
✅ Zero-dimension validation added  
✅ Edge cases return valid minimal sixel  
✅ Code compiles cleanly  
✅ Project constraints satisfied

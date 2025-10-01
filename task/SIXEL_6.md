# TASK: Performance - String Pre-allocation

**PRIORITY:** MEDIUM  
**ESTIMATED TIME:** Single session  
**SCOPE:** Optimize memory allocation by pre-allocating string capacity

---

## OBJECTIVE

Eliminate multiple string reallocations by pre-calculating the estimated final size and allocating capacity upfront. Currently the string starts small and grows dynamically, causing performance overhead.

---

## CONTEXT

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Line:** 5 (or line after validation in SIXEL_5)

**Current Problem:**
```rust
let mut result = String::from("\x1BPq");
```

**Issues:**
- String starts with minimal capacity (~4 bytes)
- Grows as content added: header, palette, pixel data
- For large images: multiple reallocations
- Each reallocation: allocate new buffer, copy old data, free old buffer

**Performance Impact:**
- Typical image (800×600): ~240KB final size
- Without pre-allocation: ~18 reallocations (doubling strategy)
- With pre-allocation: 0-1 reallocations

---

## SUBTASKS

### SUBTASK 1: Calculate Estimated Capacity

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Location:** Replace line 5 (or after validation if SIXEL_5 completed)

**Estimation formula:**
```
Header overhead = 1024 bytes (DCS, raster, palette, terminators)
Pixel data ≈ 0.5 bytes per pixel (conservative with run-length encoding)
Total capacity = (width × height / 2) + 1024
```

**Why 0.5 bytes per pixel:**
- Best case (solid color): Very compressed, ~0.1 bytes/pixel
- Worst case (checkerboard): No compression, ~1.5 bytes/pixel
- Average case: ~0.5 bytes/pixel is conservative middle ground
- RLE helps most real images achieve good compression

---

### SUBTASK 2: Implement Pre-allocation

**File:** `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Line:** 5 (or adjusted line if validation added)

**Current code:**
```rust
let mut result = String::from("\x1BPq");
```

**Replace with:**
```rust
// Pre-allocate string capacity based on image size
// Estimated: ~0.5 bytes per pixel + 1024 bytes overhead (header/palette/footer)
let estimated_capacity = (img.width() * img.height() / 2 + 1024) as usize;
let mut result = String::with_capacity(estimated_capacity);
result.push_str("\x1BPq");
```

**What this does:**
1. Calculate estimated final size
2. Allocate string with that capacity
3. Initialize with DCS sequence (same as before)

**Note:** If SIXEL_5 (validation) was completed, this goes AFTER the early return:
```rust
pub fn encode_sixel(img: &image::RgbImage) -> String {
    // Handle edge case: empty images
    if img.width() == 0 || img.height() == 0 {
        return String::from("\x1BPq\x1B\\");
    }
    
    // Pre-allocate string capacity based on image size
    let estimated_capacity = (img.width() * img.height() / 2 + 1024) as usize;
    let mut result = String::with_capacity(estimated_capacity);
    result.push_str("\x1BPq");
    
    // ... rest of function
}
```

---

### SUBTASK 3: Verify Estimation Quality

**Estimation accuracy check:**

For various image types, verify the estimate is reasonable:

1. **Tiny image (10×10):**
   - Pixels: 100
   - Estimate: 100/2 + 1024 = 1074 bytes
   - Actual: ~1100 bytes
   - ✓ Good estimate

2. **Medium image (400×300):**
   - Pixels: 120,000
   - Estimate: 60,000 + 1024 = 61,024 bytes
   - Actual: ~50,000-70,000 bytes (depends on content)
   - ✓ Reasonable range

3. **Large image (1920×1080):**
   - Pixels: 2,073,600
   - Estimate: 1,036,800 + 1024 = 1,037,824 bytes
   - Actual: ~800,000-1,200,000 bytes
   - ✓ Conservative but acceptable

**Edge cases:**

- **Solid color (best compression):**
  - Estimate may over-allocate 2-3x
  - Acceptable: memory freed when function returns

- **Checkerboard (worst compression):**
  - Estimate may under-allocate by 50%
  - Still better: 1 reallocation vs 18

- **Empty image (0×0):**
  - Early return prevents this code path
  - No capacity calculation needed

---

## RESEARCH NOTES

### String Growth Strategy (Rust)
Rust's String uses capacity doubling:
- Start: 0-4 bytes
- Grows: 4→8→16→32→64→128→256→...
- For 240KB final size: ~18 reallocations

### Memory Allocation Cost
Each reallocation:
1. Allocate new buffer (malloc)
2. Copy existing data (memcpy)
3. Free old buffer (free)

Cost is O(n) where n = current size, happens O(log n) times → O(n log n) total.

With pre-allocation: O(1) allocations → O(n) total.

### Capacity vs Length
```rust
let mut s = String::with_capacity(1000);
s.len()      // 0 (empty)
s.capacity() // 1000 (allocated)
```
- Capacity: allocated memory
- Length: actual content
- Over-allocating capacity is cheap (just reserves space)
- Under-allocating causes reallocations (expensive)

### Conservative Estimation Benefits
- Slightly over-allocate: wastes a bit of memory temporarily
- Under-allocate: triggers expensive reallocations
- 0.5 bytes/pixel is conservative (usually over-estimates)
- Better to over-estimate than under-estimate

### Run-Length Encoding Impact
Sixel uses RLE for repeated patterns:
- Solid regions: `!<count><char>` (highly compressed)
- Varied regions: individual chars (no compression)
- Average: ~50% compression for typical images
- Estimate accounts for this with 0.5 multiplier

---

## CONSTRAINTS

**CRITICAL:**
- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles performance testing)
- DO NOT change algorithm logic
- DO NOT modify string content (only capacity)
- Keep formula simple and conservative

**Requirements:**
- No `unwrap()` in src/ (project constraint)
- No `expect()` in src/ or examples
- Formula must handle any image size
- Casting to usize must be safe
- Code must compile without errors

---

## VERIFICATION

### Compilation Check
```bash
cargo check -p rio-ext-test
```

### Code Inspection
```bash
# Verify with_capacity used
rg "String::with_capacity" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Verify calculation present
rg "width.*height.*1024" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Verify DCS sequence still added
rg 'push_str.*"\\x1BPq"' packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs
```

### Capacity Verification (Optional)
Temporarily add debug output to check:
```rust
let estimated_capacity = (img.width() * img.height() / 2 + 1024) as usize;
let mut result = String::with_capacity(estimated_capacity);
result.push_str("\x1BPq");

// ... encode image ...

eprintln!("Estimated: {}, Actual: {}, Capacity: {}", 
    estimated_capacity, result.len(), result.capacity());
```

Expected:
- Capacity ≥ Estimated (exact or rounded up by allocator)
- Actual close to Estimated (±50% is acceptable)
- Capacity not doubled multiple times

### Code Review Checklist
- [ ] Capacity calculated before String creation
- [ ] Formula: `(width × height / 2 + 1024) as usize`
- [ ] String created with `String::with_capacity(estimated_capacity)`
- [ ] DCS sequence still added: `result.push_str("\x1BPq")`
- [ ] Comment explains estimation strategy
- [ ] Formula handles edge cases (small/large images)
- [ ] No integer overflow (width*height safe for u32)
- [ ] No compilation errors

### Success Criteria
✅ String capacity pre-allocated  
✅ Estimation formula reasonable  
✅ Reduces reallocations from ~18 to 0-1  
✅ Code compiles cleanly  
✅ Memory usage optimized  
✅ No behavior changes (only performance)

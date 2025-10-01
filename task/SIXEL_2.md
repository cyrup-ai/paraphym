# TASK: Module Constants - Test Code Completion

**STATUS:** 🟡 **INCOMPLETE - One Test Remains**  
**PRIORITY:** HIGH  
**SCOPE:** Replace hardcoded values in test_vertical_stripes_no_skip with SIXEL_HEIGHT constant

---

## ⚠️ OUTSTANDING ISSUE

**Production code is COMPLETE and perfect (10/10)**  
**Tests 1-3 are COMPLETE**  
**Test 4 has 5 hardcoded values that must use SIXEL_HEIGHT**

### Required Changes in Test Code

**File**: `packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`

#### Test: `test_vertical_stripes_no_skip` (Lines 513-538)

**Current:**
```rust
let mut img = image::RgbImage::new(12, 6);

// Columns 0-5: color 1 (dark blue)
for y in 0..6 {
    for x in 0..6 {
        img.put_pixel(x, y, image::Rgb([20u8, 20u8, 80u8]));
    }
}

// Columns 6-11: color 2 (dark green)
for y in 0..6 {
    for x in 6..12 {
        img.put_pixel(x, y, image::Rgb([20u8, 80u8, 20u8]));
    }
}
```

**Should be:**
```rust
const TEST_IMG_WIDTH: u32 = 2 * SIXEL_HEIGHT;  // 12 pixels
let mut img = image::RgbImage::new(TEST_IMG_WIDTH, SIXEL_HEIGHT);

// Columns 0-5: color 1 (dark blue)
for y in 0..SIXEL_HEIGHT {
    for x in 0..(TEST_IMG_WIDTH / 2) {
        img.put_pixel(x, y, image::Rgb([20u8, 20u8, 80u8]));
    }
}

// Columns 6-11: color 2 (dark green)
for y in 0..SIXEL_HEIGHT {
    for x in (TEST_IMG_WIDTH / 2)..TEST_IMG_WIDTH {
        img.put_pixel(x, y, image::Rgb([20u8, 80u8, 20u8]));
    }
}
```

**Rationale:** Image height of 6 represents one sixel row. Width of 12 is 2×SIXEL_HEIGHT. Using constants makes semantic relationships explicit and maintains consistency with production code standards.

---

## QA RATING: 9/10

### Rating Justification

**Strengths (Production Code - 10/10):**
- ✅ PALETTE constant perfectly implemented at module level with all 16 VT340 colors
- ✅ SIXEL_HEIGHT constant with excellent documentation
- ✅ All production code uses SIXEL_HEIGHT correctly (7 locations)
- ✅ Both find_closest_color closures use PALETTE.iter() - zero allocation overhead
- ✅ Helper function `palette_to_sixel_header()` added for DRY compliance
- ✅ All color values use PALETTE constant
- ✅ Code compiles and all 4 tests pass

**Strengths (Tests 1-3 - 10/10):**
- ✅ test_geometric_encoding uses `TEST_IMG_SIZE = 2 * SIXEL_HEIGHT`
- ✅ test_region_detection uses `img_height = SIXEL_HEIGHT`
- ✅ test_region_merging uses `height: SIXEL_HEIGHT`

**Critical Deficiency (Test 4 - INCOMPLETE):**
- ❌ test_vertical_stripes_no_skip has 5 hardcoded values (lines 516, 519, 520, 526, 527)
- ❌ Violates stated requirement: "All occurrences of hardcoded 6 replaced with SIXEL_HEIGHT"
- ❌ Tests are part of production quality code
- ❌ Inconsistent with the excellent standards established in production code and tests 1-3

**Why This Matters:**
1. **Semantic Correctness:** Test image dimensions represent sixel rows, not arbitrary numbers
2. **Maintainability:** If SIXEL_HEIGHT changes, all code should adapt
3. **Code Consistency:** Production and tests 1-3 use constants; test 4 should too
4. **Task Completeness:** Requirements stated "ALL occurrences" - this is incomplete

**Penalty Calculation:**
- Production implementation: 10/10 (perfect)
- Test implementation: 8/10 (75% complete - 3 of 4 tests use constants)
- **Overall: 9/10**

---

## VERIFICATION COMMANDS

After fixing test 4:

```bash
cd /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt

# Should find ONLY the const definition and comments
grep -n "= 6" src/renderer/mod.rs

# Should find NO hardcoded ranges in tests
grep -n "0\.\.6\|\.\.6\|new(6\|new(12" src/renderer/mod.rs | grep test_

# Verify all tests still pass
cargo test --lib

# Verify SIXEL_HEIGHT usage count (should be 18+ uses including test 4)
grep -o "SIXEL_HEIGHT" src/renderer/mod.rs | wc -l
```

---

## DEFINITION OF DONE

This task is complete when:

1. ✅ **PALETTE Constant**: Module-level const with all 16 colors (DONE)
2. ✅ **SIXEL_HEIGHT Constant**: Module-level const = 6 (DONE)
3. ✅ **Production Code**: All uses SIXEL_HEIGHT (DONE - 7/7 locations)
4. ❌ **Test Code**: All uses SIXEL_HEIGHT (INCOMPLETE - 3/4 tests complete)
5. ✅ **Helper Function**: palette_to_sixel_header() added (DONE)
6. ✅ **Compiles Cleanly**: cargo test --lib passes (DONE - 4/4 tests passing)
7. ✅ **Tests 1-3 Fixed**: Use SIXEL_HEIGHT properly (DONE)
8. ❌ **Test 4 Fixed**: Must use SIXEL_HEIGHT (OUTSTANDING)

**Remaining Work:** Fix 5 hardcoded values in test_vertical_stripes_no_skip (lines 516, 519, 520, 526, 527)

---

## FILE LOCATION

**Primary File**: `/Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`  
**Lines to fix:** 516, 519, 520, 526, 527

**Estimated Fix Time:** 2 minutes

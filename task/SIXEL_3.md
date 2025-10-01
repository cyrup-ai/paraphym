# TASK: DRY - Palette Deduplication

**PRIORITY:** HIGH  
**ESTIMATED TIME:** Single session  
**SCOPE:** Eliminate palette duplication (DRY violation)

---

## OBJECTIVE

Fix DRY (Don't Repeat Yourself) violation where palette generation logic exists in TWO different forms. Create a single reusable helper function that generates sixel header palette strings from the PALETTE constant, then use it in both `regions_to_sixel()` and `encode_sixel_legacy()`.

---

## CONTEXT

**File:** [`packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs)

**Current Problem:**
Palette generation logic is duplicated in TWO locations:

1. **`regions_to_sixel()` function (lines 171-178):** Uses iterator with format! macro
2. **`encode_sixel_legacy()` function (lines 273-287):** Uses 16 hardcoded push_str calls

**Issues:**
- Changing palette format requires updating two locations
- Risk of mismatch between implementations
- Violates DRY principle
- Legacy function has verbose hardcoded strings (15 lines of code)
- Maintenance burden and error-prone

**Example of Current Duplication:**

Location 1 - `regions_to_sixel()` (lines 171-178):
```rust
// Define 16-color palette (VT-340 compatible)
for (i, &(r, g, b)) in PALETTE.iter().enumerate() {
    result.push_str(&format!("#{};2;{};{};{}", i, r, g, b));
}
```

Location 2 - `encode_sixel_legacy()` (lines 273-287):
```rust
// Define a basic 16-color palette (8-bit RGB values)
// Using VT-340 compatible palette for better Rio compatibility
result.push_str("#0;2;0;0;0"); // 0: Black
result.push_str("#1;2;20;20;80"); // 1: Dark Blue
result.push_str("#2;2;20;80;20"); // 2: Dark Green
result.push_str("#3;2;20;80;80"); // 3: Dark Cyan
result.push_str("#4;2;80;20;20"); // 4: Dark Red
result.push_str("#5;2;80;20;80"); // 5: Dark Magenta
result.push_str("#6;2;80;80;20"); // 6: Brown
result.push_str("#7;2;80;80;80"); // 7: Light Gray
result.push_str("#8;2;40;40;40"); // 8: Dark Gray
result.push_str("#9;2;40;40;100"); // 9: Light Blue
result.push_str("#10;2;40;100;40"); // 10: Light Green
result.push_str("#11;2;40;100;100"); // 11: Light Cyan
result.push_str("#12;2;100;40;40"); // 12: Light Red
result.push_str("#13;2;100;40;100"); // 13: Light Magenta
result.push_str("#14;2;100;100;40"); // 14: Yellow
result.push_str("#15;2;100;100;100"); // 15: White
```

---

## TECHNICAL SPECIFICATION

### Sixel Color Definition Format

**Source:** [VT330/VT340 Programmer Reference Manual, Chapter 14](https://vt100.net/docs/vt3xx-gp/chapter14.html)

**Format:** `#<color_index>;2;<red>;<green>;<blue>`

- `#` - Color Introducer (2/3 character in sixel protocol)
- `<color_index>` - Color palette index (0-255, we use 0-15)
- `2` - RGB color coordinate system indicator
- `<red>`, `<green>`, `<blue>` - RGB intensity values (0-100 percent range per VT340 spec)

**Note:** The sixel specification uses 0-100 range for RGB values, NOT the typical 0-255 byte range.

### Performance Analysis

**Current State:**
- Palette generation happens once per `encode_sixel()` call (once per image)
- `regions_to_sixel()`: Dynamic generation with iterator (7 lines, 16 allocations)
- `encode_sixel_legacy()`: Hardcoded strings (15 lines, 16 allocations)
- Cost: Negligible (happens once, not per pixel)

**After Refactoring:**
- Single helper function used by both implementations
- Same performance (once per image encoding)
- Massive maintainability improvement
- Reduced code duplication: ~22 lines → ~8 lines
- Trade-off: Tiny overhead for huge maintainability gain

---

## IMPLEMENTATION PLAN

### STEP 1: Create Palette Header Generator

**Location:** Add function immediately after PALETTE constant (after line 18, before line 20)

**Implementation:**
```rust
/// Generate sixel color palette definition from PALETTE constant
/// 
/// Returns a string containing all 16 color definitions in sixel format.
/// Format per VT340 spec: `#<index>;2;<r>;<g>;<b>` for each color.
/// RGB values are in 0-100 percent range per sixel specification.
///
/// # Returns
/// String like: `#0;2;0;0;0#1;2;20;20;80...#15;2;100;100;100`
fn palette_to_sixel_header() -> String {
    PALETTE
        .iter()
        .enumerate()
        .map(|(i, &(r, g, b))| format!("#{};2;{};{};{}", i, r, g, b))
        .collect::<Vec<_>>()
        .join("")
}
```

**What it does:**
- Iterates over PALETTE constant (defined at lines 1-18)
- Formats each color as sixel definition: `#<index>;2;<r>;<g>;<b>`
- Joins all definitions into single string (no separators between colors)
- Returns complete palette header for sixel sequence

**Why:** Creates single source of truth - palette changes only need to update PALETTE constant

---

### STEP 2: Update `regions_to_sixel()` Function

**File:** [`packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs)  
**Lines:** 171-178 (replace with single line)

**Current code (to replace):**
```rust
// Define 16-color palette (VT-340 compatible)
for (i, &(r, g, b)) in PALETTE.iter().enumerate() {
    result.push_str(&format!("#{};2;{};{};{}", i, r, g, b));
}
```

**Replace with:**
```rust
// Define palette (DRY - generated from PALETTE constant)
result.push_str(&palette_to_sixel_header());
```

**Change:** 7 lines → 2 lines (including comment)

---

### STEP 3: Update `encode_sixel_legacy()` Function

**File:** [`packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs)  
**Lines:** 273-287 (replace with single line)

**Current code (to remove):**
```rust
// Define a basic 16-color palette (8-bit RGB values)
// Using VT-340 compatible palette for better Rio compatibility
result.push_str("#0;2;0;0;0"); // 0: Black
result.push_str("#1;2;20;20;80"); // 1: Dark Blue
result.push_str("#2;2;20;80;20"); // 2: Dark Green
result.push_str("#3;2;20;80;80"); // 3: Dark Cyan
result.push_str("#4;2;80;20;20"); // 4: Dark Red
result.push_str("#5;2;80;20;80"); // 5: Dark Magenta
result.push_str("#6;2;80;80;20"); // 6: Brown
result.push_str("#7;2;80;80;80"); // 7: Light Gray
result.push_str("#8;2;40;40;40"); // 8: Dark Gray
result.push_str("#9;2;40;40;100"); // 9: Light Blue
result.push_str("#10;2;40;100;40"); // 10: Light Green
result.push_str("#11;2;40;100;100"); // 11: Light Cyan
result.push_str("#12;2;100;40;40"); // 12: Light Red
result.push_str("#13;2;100;40;100"); // 13: Light Magenta
result.push_str("#14;2;100;100;40"); // 14: Yellow
result.push_str("#15;2;100;100;100"); // 15: White
```

**Replace with:**
```rust
// Define palette (DRY - generated from PALETTE constant)
result.push_str(&palette_to_sixel_header());
```

**Change:** 17 lines → 2 lines (including comment)

---

## EXPECTED OUTPUT

The generated palette string MUST match the existing hardcoded output exactly:

```
#0;2;0;0;0#1;2;20;20;80#2;2;20;80;20#3;2;20;80;80#4;2;80;20;20#5;2;80;20;80#6;2;80;80;20#7;2;80;80;80#8;2;40;40;40#9;2;40;40;100#10;2;40;100;40#11;2;40;100;100#12;2;100;40;40#13;2;100;40;100#14;2;100;100;40#15;2;100;100;100
```

**Format verification:**
- 16 color definitions
- No spaces between colors
- Format: `#<index>;2;<r>;<g>;<b>` for each
- RGB values match PALETTE constant exactly

---

## DRY BENEFITS

**Before (duplication):**
- Change palette: Update PALETTE + two generation logic locations
- Risk: Iterator logic and hardcoded strings could diverge
- Maintenance: Error-prone, verbose, hard to update
- Code size: ~24 lines of duplication

**After (DRY):**
- Change palette: Update PALETTE constant only
- Risk: Eliminated (single source, single generation function)
- Maintenance: Safe, simple, one function to update
- Code size: ~8 lines (including helper function)

**Code reduction:** ~16 lines eliminated

---

## CONSTRAINTS

**CRITICAL:**
- DO NOT change palette values in PALETTE constant
- DO NOT modify sixel format specification
- Generated output MUST match original exactly
- Function must be private (not `pub`)
- No `unwrap()` or `expect()` in function
- Code must compile without errors or warnings

**Requirements:**
- Helper function returns `String` (ownership transferred to caller)
- Function placed immediately after PALETTE constant
- Both `regions_to_sixel()` and `encode_sixel_legacy()` must use the helper
- Preserve all comments explaining palette purpose

---

## DEFINITION OF DONE

**Implementation Complete When:**

1. ✅ `palette_to_sixel_header()` function exists after PALETTE constant
2. ✅ Function signature: `fn palette_to_sixel_header() -> String`
3. ✅ Function generates palette from PALETTE using iterator
4. ✅ Format string matches: `#{};2;{};{};{}`
5. ✅ `regions_to_sixel()` uses `palette_to_sixel_header()`
6. ✅ `encode_sixel_legacy()` uses `palette_to_sixel_header()`
7. ✅ All hardcoded palette strings removed
8. ✅ Generated output matches expected format exactly
9. ✅ No compilation errors or warnings
10. ✅ DRY principle satisfied (single source of truth)

**Verification:**
```bash
# Code compiles cleanly
cargo check -p rio-ext-test

# Helper function exists
rg "fn palette_to_sixel_header" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Hardcoded strings removed (should find NONE in active code)
rg 'push_str\("#0;2;0;0;0"\)' packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs

# Both functions use helper
rg "palette_to_sixel_header\(\)" packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs
```

---

## SOURCE FILE REFERENCES

**Related Files:**
- Implementation: [`packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs`](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs)
- PALETTE constant: [Lines 1-18](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs#L1-L18)
- regions_to_sixel: [Lines 165-230](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs#L165-L230)
- encode_sixel_legacy: [Lines 263-424](../packages/sweetmcp/packages/sixel6vt/src/renderer/mod.rs#L263-L424)

**Related Tasks:**
- SIXEL_2: Created PALETTE constant (prerequisite, completed)

**External References:**
- [VT340 Sixel Specification - Chapter 14](https://vt100.net/docs/vt3xx-gp/chapter14.html)
- VT340 Color Format: Section on RGB color coordinate system
# TURD: Language Revisions - Remaining "Legacy" References

## Status: INCOMPLETE (8/10)

Two comments still contain "legacy encoder" terminology, which will trigger false positives in future code audits. This defeats the core objective: "reducing false positives in future code audits."

---

## Outstanding Issue: Remaining "Legacy" Comments in renderer/mod.rs

**File:** `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/renderer/mod.rs`

### Comment 1 (Line 644)

**Current:**
```rust
// Use legacy encoder for complex images to avoid O(r²) merge performance issues
```

**Required Change:**
```rust
// Use column-based encoder for complex images to avoid O(r²) merge performance issues
```

**Implementation:**
Replace "legacy encoder" with "column-based encoder" in the comment at line 644.

---

### Comment 2 (Line 661)

**Current:**
```rust
// Fall back to legacy encoder for complex images with many regions
```

**Required Change:**
```rust
// Fall back to column-based encoder for complex images with many regions
```

**Implementation:**
Replace "legacy encoder" with "column-based encoder" in the comment at line 661.

---

## Why This Matters

The original task objective states:
> "Improve code clarity by replacing misleading terminology with precise technical language. This is NOT about fixing bugs or changing functionality - it's about making the codebase more maintainable and **reducing false positives in future code audits**."

With "legacy encoder" appearing in these two comments, a future search for "legacy" will STILL trigger false positives, defeating the entire purpose of this terminology cleanup task.

---

## Completed Items ✓

The following changes were successfully completed:

1. ✓ **Line 59 (image_generation/mod.rs):** "dummy tensor" → "zero tensor for streaming progress"
2. ✓ **Line 649 (renderer/mod.rs):** Log message "legacy encoder" → "column-based encoder"
3. ✓ **Line 654 (renderer/mod.rs):** Function call `encode_sixel_legacy` → `encode_sixel_column_based`
4. ✓ **Line 664 (renderer/mod.rs):** Log message "legacy encoder" → "column-based encoder"
5. ✓ **Line 667 (renderer/mod.rs):** Function call `encode_sixel_legacy` → `encode_sixel_column_based`
6. ✓ **Lines 677-683 (renderer/mod.rs):** Function definition and documentation updated
7. ✓ **Code compiles without errors**

---

## Definition of Done

- [ ] Line 644 comment updated to say "column-based encoder"
- [ ] Line 661 comment updated to say "column-based encoder"
- [ ] Search for "legacy" in renderer/mod.rs returns zero results
- [ ] File compiles without errors

---

## Verification Command

After completing changes, verify no "legacy" references remain:

```bash
grep -i "legacy" /Volumes/samsung_t9/paraphym/packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/renderer/mod.rs
```

Expected output: (empty - no matches)
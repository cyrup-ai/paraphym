# num-bigint-dig Patch Information

## Upstream Repository
- Original: https://github.com/dignifiedquire/num-bigint
- Version: 0.8.4
- Branch: Based on master with additional fixes

## Patches Applied

### 1. Fix Private vec! Macro Warnings
**Commit**: Local patch (based on upstream d94efb24354fe56dc799d9979c279a542d2ce195)

**Changes**:
1. `src/biguint.rs`: Fixed 6 instances (already fixed in upstream commit d94efb2)
2. `src/prime.rs`: Line 138 - Changed `vec![...]` to `alloc::vec![...]`
3. `src/bigrand.rs`: Line 319 - Changed `vec![...]` to `alloc::vec![...]`
4. `build.rs`: Added proper cfg check declaration

**Issue**: https://github.com/rust-lang/rust/issues/120192
Future Rust versions will treat private macro usage as a hard error.

**Status**: Complete - All private `vec!` macro usage has been replaced with `alloc::vec![]`

## Maintenance

This is a temporary fork used via `[patch.crates-io]` in the workspace Cargo.toml.
Once the upstream maintainer publishes a new version (>0.8.4) with these fixes,
this fork can be removed.

## Testing

Verified with:
- `cargo check --workspace` ✅
- `cargo clippy --workspace --all-targets` ✅
- `cargo build --workspace` ✅

All tests pass with 0 errors and 0 warnings.

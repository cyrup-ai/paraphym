# LEGACY_H: Final Comment Cleanup

## OBJECTIVE
Remove all "backward compatibility" and "legacy" marker comments after code removal is complete.

## SCOPE
Entire codebase - final cleanup pass to remove documentation of removed code.

## SUBTASK 1: Remove backward compat comment markers

Search and remove standalone comments:

```bash
# Find all backward compat comments
grep -rn "// Backward compatibility\|/// Type alias for backwards compatibility\|// Re-export for backward compatibility" packages/candle/src packages/sweetmcp/packages --include="*.rs"
```

Delete these comment lines (they were markers for code we've now removed)

## SUBTASK 2: Remove legacy comment markers

```bash
# Find all legacy comments
grep -rn "// Legacy\|/// Legacy\|// legacy marker" packages/candle/src packages/sweetmcp/packages --include="*.rs"
```

Delete these comment lines

## SUBTASK 3: Remove compat alias comments

```bash
# Find all compat comments
grep -rn "// Alias for backward compatibility\|/// Type alias for.*compat" packages/candle/src packages/sweetmcp/packages --include="*.rs"
```

Delete these comment lines

## SUBTASK 4: Remove fallback documentation

Search for and remove documentation about fallback env vars or deprecated APIs:
- "For legacy SurrealDB setups"
- "as fallbacks for"
- "backward compatibility"

## SUBTASK 5: Verify all legacy references removed

**Final validation:**
```bash
# Should return ZERO results (or only external API compat notes)
grep -ri "backward.*compat\|legacy.*alias\|for backward" packages/candle/src packages/sweetmcp/packages --include="*.rs"
```

**Acceptable exceptions:**
- Comments about compatibility with EXTERNAL APIs (e.g., "OpenAI compatibility")
- Comments about standard compliance (e.g., "HTTP3 API standardization")

**Must remove:**
- Internal backward compatibility comments
- Legacy code markers
- Deprecated API documentation

## VALIDATION COMMANDS
```bash
# Verify no legacy markers remain
grep -ri "backward.*compat\|legacy.*alias\|legacy marker\|for backward" packages/ --include="*.rs" | grep -v "OpenAI\|HTTP3\|external"

# Verify compilation
cargo check --all-features
cargo clippy --all-features
```

## DEFINITION OF DONE
- ✅ All "backward compatibility" comments removed
- ✅ All "legacy" marker comments removed
- ✅ All deprecated API documentation removed
- ✅ Only external API compatibility notes remain (if any)
- ✅ Code compiles without errors
- ✅ Clippy passes without warnings

## EXECUTION ORDER
**Task 8 of 8** - Execute LAST (after all code removal complete)

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on comment cleanup
- Be careful not to remove important external compatibility notes

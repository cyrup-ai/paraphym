# INPROD_27: Chat Config Persistence Sync Access

## SEVERITY: MEDIUM

## OBJECTIVE
Fix async/sync mismatch in chat configuration persistence instead of using hardcoded defaults.

## LOCATION
- `packages/candle/src/domain/chat/config.rs`

## CURRENT STATE
- Line 1197: `// For now, use defaults`
- Comment mentions persistence requires async but method is sync
- Hardcoded format = "json" and compression = false
- Configuration persistence is bypassed

## SUBTASK 1: Analyze Sync/Async Requirements
- Locate config.rs:1197
- Determine if method can be made async
- Or if persistence can provide sync access
- Evaluate architectural options

## SUBTASK 2: Implement Proper Config Access
- Either: Convert method to async and update callers, or
- Use sync wrapper around async persistence (blocking), or
- Cache config values for sync access, or
- Refactor persistence to support sync reads
- Choose approach that fits architecture

## SUBTASK 3: Load Actual Configuration
- Replace hardcoded defaults with real config values
- Access persistence layer properly
- Handle config load errors appropriately
- Return actual format and compression settings

## DEFINITION OF DONE
- [ ] Async/sync mismatch is resolved
- [ ] Actual configuration is loaded
- [ ] Hardcoded defaults are removed
- [ ] Config persistence is properly accessed
- [ ] Stub comment removed

## RESEARCH NOTES
- Review persistence layer interfaces
- Check for existing sync/async patterns
- Examine callers of this method
- Look for config caching mechanisms

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

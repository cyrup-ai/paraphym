# INPROD_18: Migration Checksum Calculation

## SEVERITY: MEDIUM

## OBJECTIVE
Implement proper migration checksum calculation instead of using simple version-based strings.

## LOCATION
- `packages/candle/src/memory/migration/mod.rs`

## CURRENT STATE
- Line 159: `// Calculate checksum (simple version-based for now)`
- Checksum is just format string with version number
- No actual content hashing for migration integrity
- Cannot detect if migration code has changed

## SUBTASK 1: Implement Content-Based Checksum
- Locate migration/mod.rs:159
- Hash the actual migration content/code
- Use cryptographic hash (SHA256 or similar)
- Generate checksum from migration implementation

## SUBTASK 2: Include Migration Metadata
- Hash migration version, name, and dependencies
- Include up/down migration content in hash
- Ensure reproducible checksum generation
- Handle migration content serialization

## SUBTASK 3: Verify Checksum on Migration
- Store checksums with applied migrations
- Verify checksum matches when re-running migrations
- Detect if migration code has changed since application
- Warn or error on checksum mismatch

## DEFINITION OF DONE
- [ ] Checksum is based on actual migration content
- [ ] Cryptographic hash is used
- [ ] Checksums are verified on migration runs
- [ ] Migration tampering can be detected
- [ ] Stub comment removed

## RESEARCH NOTES
- Review migration structure and available content
- Check for existing hash utilities in the codebase
- Examine how migrations are stored and tracked
- Look for checksum validation patterns

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

# INPROD_21: Dynamic Root Loading

## SEVERITY: MEDIUM

## OBJECTIVE
Implement dynamic root loading from configuration or filesystem instead of hardcoded roots.

## LOCATION
- `packages/sweetmcp/packages/axum/src/root/mod.rs`

## CURRENT STATE
- Line 11: `// In a real-world implementation, these would be dynamically loaded`
- Roots are hardcoded in the response
- No dynamic scanning or configuration loading
- Cannot add roots without code changes

## SUBTASK 1: Load Roots from Configuration
- Locate root/mod.rs:11
- Read root definitions from configuration file or database
- Parse root URIs and metadata
- Support multiple configuration sources

## SUBTASK 2: Scan Filesystem for Roots
- Implement filesystem scanning for root directories
- Discover roots based on markers or patterns
- Build root list dynamically at startup or runtime
- Cache discovered roots with refresh capability

## SUBTASK 3: Support Hot Reloading
- Watch configuration for changes
- Reload roots when configuration updates
- Invalidate caches when roots change
- Notify clients of root list changes if needed

## DEFINITION OF DONE
- [ ] Roots are loaded from configuration or scanned from filesystem
- [ ] No hardcoded roots remain
- [ ] Hot reloading is supported
- [ ] Configuration changes are detected
- [ ] Stub comment removed

## RESEARCH NOTES
- Review configuration structure and loading patterns
- Check for existing filesystem scanning utilities
- Examine ListRootsResult structure
- Look for configuration watching/reloading patterns

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

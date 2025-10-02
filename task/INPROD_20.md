# INPROD_20: File Watcher Event System

## SEVERITY: MEDIUM

## OBJECTIVE
Implement proper file watching with event-based monitoring instead of polling with metadata checks.

## LOCATION
- `packages/cylo/src/watcher.rs`

## CURRENT STATE
- Line 40: `// In a real implementation, we would use watchexec to get actual file events`
- Currently polls file metadata every 5 seconds
- No actual file system event monitoring
- Inefficient and has latency

## SUBTASK 1: Integrate watchexec or notify Crate
- Locate watcher.rs:40
- Replace polling loop with event-based file watching
- Use watchexec, notify crate, or similar for FS events
- Register watchers for specified paths

## SUBTASK 2: Handle File System Events
- Process create, modify, delete, and rename events
- Filter events based on file patterns if needed
- Debounce rapid successive events
- Handle platform-specific event details

## SUBTASK 3: Trigger Rebuild on Changes
- Invoke rebuild when relevant files change
- Pass changed file paths to rebuild handler
- Handle errors during rebuild
- Maintain previous_time or metadata for comparison

## DEFINITION OF DONE
- [ ] File watching uses event-based monitoring
- [ ] Polling is replaced with FS events
- [ ] Changes trigger rebuilds in real-time
- [ ] Platform-specific events are handled
- [ ] Stub comment removed

## RESEARCH NOTES
- Review notify or watchexec crate documentation
- Check existing dependencies for file watching
- Examine event types and filtering capabilities
- Look for debouncing patterns in the codebase

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

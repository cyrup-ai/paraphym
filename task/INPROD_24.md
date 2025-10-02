# INPROD_24: Round-Robin Load Balancing

## SEVERITY: MEDIUM

## OBJECTIVE
Implement actual round-robin algorithm instead of using lowest load as a simplified substitute.

## LOCATION
- `packages/sweetmcp/packages/pingora/src/metric_picker.rs`

## CURRENT STATE
- Line 108: `// Pick backend using round-robin (simplified to lowest load for now)`
- Method is named pick_round_robin but doesn't implement round-robin
- Uses lowest load algorithm instead
- Incorrect algorithm for stated purpose

## SUBTASK 1: Add Round-Robin State Tracking
- Locate metric_picker.rs:108
- Add counter or index to track last selected backend
- Use AtomicUsize for thread-safe counter
- Initialize counter in picker construction

## SUBTASK 2: Implement True Round-Robin Selection
- Increment counter on each pick
- Use modulo to wrap around backend list
- Select backend at current index
- Return backend in rotation order

## SUBTASK 3: Handle Backend List Changes
- Reset or adjust counter when backend list changes
- Handle empty backend list gracefully
- Ensure thread-safety during list updates
- Skip unhealthy backends in rotation

## DEFINITION OF DONE
- [ ] True round-robin algorithm is implemented
- [ ] Counter tracks rotation state
- [ ] Backends are selected in order
- [ ] Thread-safety is maintained
- [ ] Stub comment removed

## RESEARCH NOTES
- Review Backend structure and list management
- Check for existing atomic counter patterns
- Examine MetricPicker structure and state
- Look for backend health checking integration

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

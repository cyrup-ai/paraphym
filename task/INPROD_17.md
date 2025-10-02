# INPROD_17: Macro Execution Error Handling

## SEVERITY: MEDIUM

## OBJECTIVE
Implement proper error handling in macro execution instead of returning default results on errors.

## LOCATION
- `packages/candle/src/domain/chat/macros.rs`

## CURRENT STATE
- Line 1109: `// Error handling via on_chunk pattern - for now just return default`
- Errors result in default MacroExecutionResult with success: false
- Actual error information is lost
- No proper error propagation or handling

## SUBTASK 1: Capture Error Information
- Locate macros.rs:1109 in the async execution
- Capture actual error details when execution fails
- Include error message, type, and context
- Don't discard error information

## SUBTASK 2: Populate MacroExecutionResult with Error Details
- Add error information to MacroExecutionResult
- Set appropriate failure status
- Include actionable error messages
- Preserve error stack if available

## SUBTASK 3: Implement Error Recovery Strategies
- Add retry logic for transient failures if appropriate
- Provide fallback behavior where applicable
- Log errors for debugging and monitoring
- Ensure errors are propagated to error handlers

## DEFINITION OF DONE
- [ ] Actual errors are captured and included in results
- [ ] Error information is not lost
- [ ] MacroExecutionResult contains meaningful error details
- [ ] Error recovery strategies are implemented
- [ ] Stub comment removed

## RESEARCH NOTES
- Review MacroExecutionResult structure and error fields
- Check for existing error types in macro system
- Examine on_chunk callback pattern for error handling
- Look for retry or fallback patterns in the codebase

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

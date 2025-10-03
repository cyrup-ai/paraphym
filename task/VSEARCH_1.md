# VSEARCH_1: Implement Defer Queue Logic

## OBJECTIVE
Implement multi-stage cognitive filtering with defer queue and RequestInfo handling for vector search.

## LOCATION
`packages/candle/src/memory/vector/vector_search.rs:390, 396, 413`

## SUBTASK 1: Create CognitiveSearchState struct
```rust
struct CognitiveSearchState {
    deferred_results: Vec<(MemorySearchResult, f64)>, // (result, confidence)
    final_results: Vec<MemorySearchResult>,
}
```

## SUBTASK 2: Implement Defer outcome handling
- In filtering logic, handle DecisionOutcome::Defer
- Queue results for secondary evaluation with confidence scores
- Replace "for now" comment at line 390

## SUBTASK 3: Add process_deferred_results function
```rust
fn process_deferred_results(state: &mut CognitiveSearchState, threshold: f64) {
    state.deferred_results.retain(|(result, confidence)| {
        if *confidence > threshold {
            state.final_results.push(result.clone());
            false
        } else {
            true
        }
    });
}
```

## SUBTASK 4: Implement RequestInfo handling
- Add user interaction callback mechanism
- Handle RequestInfo outcome at lines 396, 413
- Remove "for now" comments

## DEFINITION OF DONE
- Defer queue fully implemented
- Multi-stage filtering operational
- RequestInfo outcome handled
- All "for now" comments removed
- Code compiles without warnings

## RESEARCH NOTES
- See VECCOG_4 task documentation (if exists)
- Cognitive filtering patterns
- Existing DecisionOutcome enum

## CONSTRAINTS
- Do NOT write unit tests
- Do NOT write integration tests
- Do NOT write benchmarks
- Focus solely on src modification

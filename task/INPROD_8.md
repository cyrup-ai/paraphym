# INPROD_8: Model Registry Type Dispatch and Downcasting

## SEVERITY: HIGH

## OBJECTIVE
Implement proper type dispatch and model downcasting in the model registry instead of using simplified conversions and returning errors.

## LOCATION
- `packages/candle/src/model/registry.rs`

## CURRENT STATE
- Line 31: `// For now, convert to GenericCandleModel`
- Line 353: `// For now, this method is not fully implemented due to Arc<T> conversion complexity`
- Model downcasting returns error instead of working
- Type dispatch is simplified

## SUBTASK 1: Implement Proper Type Dispatch in Model::new
- Locate registry.rs:31 in the ModelHandle::new method
- Implement sophisticated type dispatch instead of GenericCandleModel conversion
- Preserve actual model type information
- Store model in a way that allows later downcasting

## SUBTASK 2: Implement Model Downcasting
- Locate registry.rs:353 in the downcast method
- Solve the Arc<T> conversion complexity
- Actually return the downcasted model of type T
- Remove the error return for valid downcast requests

## SUBTASK 3: Handle Type Safety
- Ensure type safety in downcast operations
- Return proper errors only for invalid downcast attempts
- Maintain Arc reference counting correctly
- Support concurrent access to models

## DEFINITION OF DONE
- [ ] Type dispatch preserves actual model types
- [ ] Downcast method successfully returns models of type T
- [ ] Arc<T> conversion complexity is resolved
- [ ] Type safety is maintained
- [ ] Error is only returned for truly invalid downcasts
- [ ] Stub comments removed

## RESEARCH NOTES
- Review Rust patterns for type-erased storage with downcasting
- Examine the Any trait usage in ModelHandle
- Look at Arc::downcast patterns
- Review how GenericCandleModel wraps other models
- Check for existing type dispatch patterns in the codebase

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

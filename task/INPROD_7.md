# INPROD_7: Model Resolver Feature Detection

## SEVERITY: HIGH

## OBJECTIVE
Implement actual capability and feature detection in the model resolver instead of returning hardcoded false values.

## LOCATION
- `packages/candle/src/model/resolver.rs`

## CURRENT STATE
- Line 405: Default provider returns None with comment about checking configuration
- Line 414: HasCapability always returns false
- Line 418: HasFeature always returns false  
- Line 423: FeatureEnabled always returns false
- All feature detection is non-functional

## SUBTASK 1: Implement Default Provider Logic
- Locate resolver.rs:405 `get_default_provider` method
- Check configuration for default provider setting
- Return configured default provider if set
- Return first available provider as fallback

## SUBTASK 2: Implement Capability Detection
- Locate resolver.rs:414 in the HasCapability match arm
- Check if the model actually has the specified capability
- Query model metadata or info for capabilities
- Return true/false based on actual capability presence

## SUBTASK 3: Implement Feature Detection
- Locate resolver.rs:418 in the HasFeature match arm
- Check if the model has the specified feature
- Query model configuration or metadata
- Return actual feature availability

## SUBTASK 4: Implement Feature Enabled Check
- Locate resolver.rs:423 in the FeatureEnabled match arm
- Check if the feature is enabled in configuration
- Check feature flags or configuration settings
- Return actual enabled status

## DEFINITION OF DONE
- [ ] Default provider is determined from configuration
- [ ] Capability detection works correctly
- [ ] Feature detection works correctly
- [ ] Feature enabled check works correctly
- [ ] No hardcoded false returns remain
- [ ] Stub comments removed

## RESEARCH NOTES
- Review ModelInfo structure and available metadata
- Check for model capability definitions
- Examine configuration structures for default provider
- Look for feature flag patterns in the codebase

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation

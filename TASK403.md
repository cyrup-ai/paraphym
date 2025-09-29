# TASK403: Ensure Builder Always Provides Default Provider

## Objective
Guarantee that the agent builder always configures a default provider, ensuring the completion_provider field is never None and eliminating potential runtime errors.

## Current State Analysis

### Problem
**File**: [`/packages/candle/src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs)
**Issue**: Builder may not set a provider, leading to `completion_provider: None` in agent role

**File**: [`/packages/candle/src/domain/agent/role.rs`](../packages/candle/src/domain/agent/role.rs)
**Issue**: `get_completion_provider()` returns `Option<&CandleCompletionProviderType>`, allowing None values

### Current Unsafe Pattern
```rust
// In domain/agent/role.rs
pub fn get_completion_provider(&self) -> Option<&CandleCompletionProviderType> {
    self.completion_provider.as_ref()
}

// In domain/agent/chat.rs - leads to runtime errors
let provider = self.get_completion_provider()
    .ok_or_else(|| ChatError::System("No completion provider configured".to_string()))?;
```

## Target Architecture

Builder automatically provides KimiK2Provider as default if no provider is explicitly configured:

```rust
// Scenario 1: No provider specified - gets default KimiK2
let agent = CandleFluentAi::agent_role("assistant").into_agent();
// ✅ completion_provider = Some(KimiK2Provider)

// Scenario 2: Explicit provider - uses what's specified
let agent = CandleFluentAi::agent_role("assistant")
    .completion_provider(qwen_provider)
    .into_agent();
// ✅ completion_provider = Some(Qwen3CoderProvider)
```

## Implementation

### 1. Add Default Provider Creation Helper
**File**: [`/packages/candle/src/providers/kimi_k2.rs`](../packages/candle/src/providers/kimi_k2.rs)

Add a default implementation for fallback provider creation:

```rust
impl CandleKimiK2Provider {
    /// Create a default provider for builder fallback
    /// Uses sync initialization to avoid async complexity in builder
    pub fn default_for_builder() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = CandleKimiK2Config::default();
        let model_path = std::env::var("KIMI_MODEL_PATH")
            .unwrap_or_else(|_| {
                // Use standard model cache location
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{}/.candle/models/kimi-k2", home)
            });
        
        Self::with_config_sync(model_path, config)
    }
}
```

### 2. Update Builder to Always Provide Default
**File**: [`/packages/candle/src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs)

**Locate**: `CandleAgentRoleBuilderImpl` (the base builder without provider)
**Modify**: The `into_agent()` method to ensure provider is always set

```rust
impl CandleAgentRoleBuilder for CandleAgentRoleBuilderImpl {
    // ... existing methods ...
    
    fn into_agent(self) -> impl CandleAgentBuilder {
        // Always provide default provider if none set
        // Import at top: use crate::providers::CandleKimiK2Provider;
        // Import at top: use crate::domain::agent::role::CandleCompletionProviderType;
        
        let default_provider = CandleCompletionProviderType::KimiK2(
            CandleKimiK2Provider::default_for_builder()
                .unwrap_or_else(|e| {
                    log::warn!("Failed to create default KimiK2 provider: {}. Using minimal fallback.", e);
                    // Create absolute minimal fallback if default_for_builder fails
                    CandleKimiK2Provider::with_config_sync(
                        "./models/kimi-k2".to_string(),
                        CandleKimiK2Config::default()
                    ).expect("Critical: Could not create fallback provider")
                })
        );
        
        CandleAgentBuilderImpl {
            name: self.name,
            provider: default_provider, // Always Some via default
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            system_prompt: self.system_prompt,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            conversation_history: None,
        }
    }
}
```

### 3. Ensure Provider Builder Preserves Provider
**File**: [`/packages/candle/src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs)

**Locate**: `CandleAgentBuilderImpl<P>` implementation where P is the explicit provider
**Verify**: That it properly converts P to CandleCompletionProviderType

The `completion_provider()` method should create a new builder with the explicit provider:

```rust
impl CandleAgentRoleBuilder for CandleAgentRoleBuilderImpl {
    fn completion_provider<P>(self, provider: P) -> impl CandleAgentRoleBuilder
    where
        P: DomainCompletionModel + Clone + Send + 'static
    {
        // This creates CandleAgentBuilderImpl<P> which has the explicit provider
        CandleAgentBuilderImpl {
            name: self.name,
            provider, // Explicit provider set here
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            system_prompt: self.system_prompt,
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            conversation_history: None,
        }
    }
}
```

### 4. Update Domain Agent Role Contract
**File**: [`/packages/candle/src/domain/agent/role.rs`](../packages/candle/src/domain/agent/role.rs)

**Current lines ~200-250**: Update `get_completion_provider()` to guarantee non-None

```rust
impl CandleAgentRoleImpl {
    /// Get completion provider - guaranteed to exist by builder
    ///
    /// # Returns
    /// Reference to completion provider (never None after builder initialization)
    #[inline]
    pub fn get_completion_provider(&self) -> &CandleCompletionProviderType {
        self.completion_provider.as_ref()
            .expect("Provider guaranteed by builder - this should never panic")
    }
}
```

**Important**: Keep the `completion_provider` field as `Option<CandleCompletionProviderType>` for now to maintain backwards compatibility with manual construction. The builder guarantees it's always Some.

### 5. Update Chat System to Use Guaranteed Provider
**File**: [`/packages/candle/src/domain/agent/chat.rs`](../packages/candle/src/domain/agent/chat.rs)

**Current pattern (lines ~175, ~227)**:
```rust
// OLD: Returns Option, requires error handling
let provider = self.get_completion_provider()
    .ok_or_else(|| ChatError::System("No completion provider configured".to_string()))?;
```

**New pattern**:
```rust
// NEW: Returns &CandleCompletionProviderType directly
let provider = self.get_completion_provider();
// No error handling needed - builder guarantees provider exists
```

## Architecture Benefits

### Safety Guarantees
- **Builder always sets provider**: No more None values from builder path
- **Runtime errors eliminated**: Chat methods can't fail due to missing provider
- **Default behavior**: Sensible defaults (KimiK2) without explicit configuration

### Clean API
```rust
// Minimal API - works out of the box
CandleFluentAi::agent_role("assistant")
    .into_agent()
    .chat_with_message("Hello")

// Explicit configuration still supported  
CandleFluentAi::agent_role("assistant")
    .completion_provider(qwen_provider)
    .into_agent()
    .chat_with_message("Hello")
```

### Backwards Compatibility
- Manual `CandleAgentRoleImpl::new()` construction still possible (returns Option)
- Builder path guarantees non-None (new behavior)
- Existing code using `.ok_or_else()` continues to work

## Technical Implementation Details

### Provider Creation Pattern
**File**: [`/packages/candle/src/providers/kimi_k2.rs`](../packages/candle/src/providers/kimi_k2.rs) (lines ~175-250)

```rust
// Existing method used for default creation
pub fn with_config_sync(
    model_path: String,
    config: CandleKimiK2Config,
) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
```

**Key Points**:
- Sync initialization (no async in builder)
- Uses `TextGenerator` (from TASK401)
- Handles ProgressHub model download
- Returns Result for error handling

### Builder Type Flow
```
CandleAgentRoleBuilderImpl (no provider)
    ↓ .into_agent()
CandleAgentBuilderImpl<DefaultProvider> (has KimiK2)
    ↓ .chat_with_message()
AsyncStream<CandleMessageChunk>

OR

CandleAgentRoleBuilderImpl (no provider)
    ↓ .completion_provider(P)
CandleAgentRoleBuilderImpl<P> (has explicit P)
    ↓ .into_agent()
CandleAgentBuilderImpl<P> (has P)
    ↓ .chat_with_message()
AsyncStream<CandleMessageChunk>
```

### Error Handling Strategy
1. **Primary**: `CandleKimiK2Provider::default_for_builder()` - tries to create with env vars
2. **Fallback**: `with_config_sync()` with hardcoded path - last resort
3. **Critical failure**: `expect()` only if both fail (should never happen in practice)

## Files Modified
- [`/packages/candle/src/providers/kimi_k2.rs`](../packages/candle/src/providers/kimi_k2.rs) - Add `default_for_builder()` method
- [`/packages/candle/src/builders/agent_role.rs`](../packages/candle/src/builders/agent_role.rs) - Update `into_agent()` to provide default
- [`/packages/candle/src/domain/agent/role.rs`](../packages/candle/src/domain/agent/role.rs) - Update `get_completion_provider()` return type
- [`/packages/candle/src/domain/agent/chat.rs`](../packages/candle/src/domain/agent/chat.rs) - Simplify provider access (remove error handling)

## Dependencies
- **TASK401**: ✅ Complete - Providers must own TextGenerator before being used as defaults
- **TASK402**: ✅ Complete - Chat system uses direct provider calls

## Success Criteria
- Builder `into_agent()` always sets completion_provider to Some(provider)
- Default provider is KimiK2Provider with ProgressHub model download support
- `get_completion_provider()` returns `&CandleCompletionProviderType` (not Option)
- No runtime panics due to missing provider configuration in builder path
- Explicit provider configuration via `.completion_provider()` still works
- Chat methods no longer need `.ok_or_else()` error handling for provider access
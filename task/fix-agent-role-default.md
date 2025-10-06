# Fix Agent Role Builder Default to Phi-4-Reasoning

## Problem

The agent role builder correctly defaults to Phi4Reasoning in `agent_role.rs:510-590`, but the CLI in `main.rs` bypasses this by always explicitly setting a provider based on the `--model` argument (which defaults to "kimi-k2").

## Builder Defaults (Definitive)

All defaults are set in `CandleAgentRoleBuilderImpl::new()` and carried through to agent builder.

### Confirmed Defaults

1. **agent_role name**: `"cyrup.ai"`
2. **temperature**: `0` (deterministic)
3. **max_tokens**: From model's `TextToTextCapability` property (model-specific)
4. **memory_read_timeout**: `5000` ms (5 seconds)
5. **system_prompt**:
```
# Well-Informed Software Architect

You think out loud as you work through problems, sharing your process in addition to the solutions.
You track every task you do or needs doing in `TODO.md`, updating it religiously before and after a meaningful change to code.
You maintain `ARCHITECTURE.md` and carefully curate the vision for the modules we create.
You prototype exploratory code ideas, quickly putting together a prototype, so we talk about the "heart of the matter" and get on the same page.
If you don't know the answer, you ALWAYS RESEARCH on the web and talk it through with me. You know that planned takes less time in the end that hastily forged.You never pretend to have answers unless you are highly confident.
You really LOVE programming and the art of it. You craft applications that are fast, efficient, and blazing fast.
You produce clean, maintainable, *production quality* code all the time.
You are a master at debugging and fixing bugs.
You are a master at refactoring code, remembering to check for code that ALREADY EXISTS before writing new code that might duplicate existing functionality.
```

6. **tools**: Sequential Thinking (native) + Reasoner (always loaded)
7. **mcp_servers**: sweetmcp local at `https://sweetmcp.cyrup.dev:8443`
8. **memory**: MemoryManager (always initialized, guaranteed to exist)
9. **text_embedding**: Stella 1024 (default embedding model)
10. **completion_provider**: Phi-4-Reasoning (when `.into_agent()` called without explicit provider)

### Other Fields (Defaults TBD)
- additional_params
- metadata
- context (file, files, directory, github)
- on_chunk handler
- on_tool_result handler
- on_conversation_turn handler

## Current State

**agent_role.rs:510-590** ✅ CORRECT
```rust
fn into_agent(self) -> impl CandleAgentBuilder {
    // Default provider is now Phi-4-Reasoning - try multiple paths with graceful fallback
    let default_provider = CandlePhi4ReasoningProvider::default_for_builder()
        .or_else(|e| {
            log::warn!("Failed to create default Phi4Reasoning provider with ProgressHub: {}. Trying local path.", e);
            CandlePhi4ReasoningProvider::with_config_sync(
                "./models/phi-4-reasoning".to_string(),
                CandlePhi4ReasoningConfig::default()
            )
        })
        // ... multiple fallback attempts ...
```

**main.rs:30** ❌ WRONG
```rust
#[arg(short, long, default_value = "kimi-k2")]
model: String,
```

**main.rs:100-142** ❌ WRONG
```rust
match args.model.as_str() {
    "kimi-k2" => {
        let provider = CandleKimiK2Provider::new().await?;
        run_chat(provider, ...) // Explicit provider set, bypasses builder default
    }
    "qwen-coder" => {
        let provider = CandleQwen3CoderProvider::new().await?;
        run_chat_qwen(provider, ...) // Explicit provider set, bypasses builder default
    }
    _ => Err("Unknown model")
}
```

## Root Cause

The CLI **always** sets an explicit provider via `.completion_provider()`, which overrides the builder's `into_agent()` default. The builder default is never used.

## Solution

The `--model` CLI arg should be **optional**:
- When **omitted**: Don't call `.completion_provider()` → `into_agent()` uses its Phi4Reasoning default
- When **provided**: Resolve model_id via factory → call `.completion_provider()`

## Files Involved

- `packages/candle/src/main.rs` - CLI that currently bypasses builder default
- `packages/candle/src/builders/agent_role.rs` - Builder with correct default (lines 510-590)
- `packages/candle/src/domain/agent/role.rs` - Defines `CandleCompletionProviderType` enum

## Model IDs

Each provider has a `model_id` in their `CandleModelInfo`:
- `phi-4-reasoning` → `CandlePhi4ReasoningProvider`
- `kimi-k2` → `CandleKimiK2Provider`
- `qwen-coder` → `CandleQwen3CoderProvider`

## Implementation

1. Create factory function in `capability/text_to_text/mod.rs`
2. Make `--model` optional in CLI Args
3. Only call `.completion_provider()` when model is provided
4. Remove all hardcoded defaults from CLI (they belong in builder)

## See Also

- `task/fix-cli-options.md` - Broader CLI refactor to expose all builder options

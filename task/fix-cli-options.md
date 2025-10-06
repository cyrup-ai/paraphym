# Fix CLI to Expose ALL Builder Methods as Flags

## Problem

`main.rs` has hardcoded defaults that belong in the builder, and doesn't expose all builder configuration options as CLI flags. The CLI should be a thin layer that forwards user input to the builder, which owns all defaults.

## Builder Defaults (Definitive)

All defaults are set in `CandleAgentRoleBuilderImpl::new()` (agent_role.rs:344-355) and are NOT Option types.

### Confirmed Defaults

1. **agent_role name**: `"cyrup.ai"`
2. **temperature**: `0` (deterministic)
3. **max_tokens**: From model's `TextToTextCapability` property (model-specific, not a builder default)
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

6. **tools**: Sequential Thinking (native) + Reasoner (always loaded by default)
7. **mcp_servers**: sweetmcp local at `https://sweetmcp.cyrup.dev:8443`
8. **memory**: MemoryManager (always initialized, guaranteed to exist)
9. **text_embedding**: Stella 1024 (default embedding model)
10. **completion_provider**: Phi-4-Reasoning (set in `into_agent()` when not explicitly provided)

### Fields Needing Default Clarification
- **additional_params**: `ZeroOneOrMany::None` (empty)
- **metadata**: ❓ **NEEDS CLARIFICATION** - Sounds like duplicate of additional_params. What's the difference? What should the default be?
- **context** (file, files, directory, github) - default?
- **on_chunk handler** - default?
- **on_tool_result handler** - default?
- **on_conversation_turn handler** - default?

## Builder Methods That Need CLI Flags

Based on `src/builders/agent_role.rs:141-243`:

### Core Settings
- ✅ `completion_provider(P)` - via `--model <model_id>`
- ✅ `temperature(f64)` - via `--temperature <f64>`
- ❌ `max_tokens(u64)` - NOT needed (comes from model capability)
- ✅ `memory_read_timeout(u64)` - via `--memory-read-timeout <ms>`
- ✅ `system_prompt(String)` - via `--system-prompt <text>` or `--system-prompt-file <path>`

### Advanced Settings
- ✅ `additional_params([("k","v")])` - via `--additional-param key=value` (repeatable)
- ✅ `metadata([("k","v")])` - via `--metadata key=value` (repeatable)
- ✅ `context(...)` - via multiple flags:
  - `--context-file <path>` (repeatable)
  - `--context-glob <pattern>` (repeatable)
  - `--context-directory <path>` (repeatable)
  - `--context-github <owner/repo>` (repeatable)
- ✅ `tools(T)` - via `--tool <name>` (repeatable)
- ✅ `mcp_server<T>().bin(path).init(cmd)` - via `--mcp-server-bin <path>` + `--mcp-server-init <cmd>`

### Handlers (Cannot be CLI flags)
- ❌ `on_chunk(F)` - Implemented in code only
- ❌ `on_tool_result(F)` - Implemented in code only
- ❌ `on_conversation_turn(F)` - Implemented in code only

## CLI Args Struct Design

**IMPORTANT**: All fields are `Option<T>` because the builder owns the defaults, not the CLI.

```rust
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model to use (omit to use builder default: phi-4-reasoning)
    #[arg(short, long)]
    model: Option<String>,

    /// Agent role name (omit to use builder default: cyrup.ai)
    #[arg(short, long)]
    role: Option<String>,

    /// Temperature for sampling (omit to use builder default: 0)
    #[arg(short, long)]
    temperature: Option<f64>,

    /// Memory read timeout in milliseconds (omit to use builder default: 5000)
    #[arg(long)]
    memory_read_timeout: Option<u64>,

    /// System prompt inline text (overrides default)
    #[arg(long, conflicts_with = "system_prompt_file")]
    system_prompt: Option<String>,

    /// System prompt from file (overrides default)
    #[arg(long, conflicts_with = "system_prompt")]
    system_prompt_file: Option<PathBuf>,

    /// Additional parameters (key=value, repeatable)
    #[arg(long = "additional-param")]
    additional_params: Vec<String>,

    /// Metadata (key=value, repeatable)
    #[arg(long = "metadata")]
    metadata: Vec<String>,

    /// Context from single file (repeatable)
    #[arg(long = "context-file")]
    context_files: Vec<PathBuf>,

    /// Context from files matching glob (repeatable)
    #[arg(long = "context-glob")]
    context_globs: Vec<String>,

    /// Context from directory (repeatable)
    #[arg(long = "context-directory")]
    context_directories: Vec<PathBuf>,

    /// Context from GitHub repo (owner/repo, repeatable)
    #[arg(long = "context-github")]
    context_github: Vec<String>,

    /// Tools to load (repeatable, adds to defaults)
    #[arg(long = "tool")]
    tools: Vec<String>,

    /// MCP server binary path
    #[arg(long)]
    mcp_server_bin: Option<PathBuf>,

    /// MCP server init command
    #[arg(long)]
    mcp_server_init: Option<String>,
}
```

## Implementation Pattern

```rust
async fn run_chat(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Start builder with optional role override
    let mut builder = match args.role {
        Some(role) => CandleFluentAi::agent_role(role),
        None => CandleFluentAi::agent_role("cyrup.ai"), // Use default
    };

    // Apply optional overrides (only if provided)
    if let Some(temp) = args.temperature {
        builder = builder.temperature(temp);
    }
    if let Some(timeout) = args.memory_read_timeout {
        builder = builder.memory_read_timeout(timeout);
    }

    // System prompt: file takes precedence over inline
    if let Some(path) = args.system_prompt_file {
        let content = std::fs::read_to_string(path)?;
        builder = builder.system_prompt(content);
    } else if let Some(prompt) = args.system_prompt {
        builder = builder.system_prompt(prompt);
    }
    // Otherwise: uses builder default

    // Additional params
    if !args.additional_params.is_empty() {
        let params = parse_key_value_pairs(&args.additional_params)?;
        builder = builder.additional_params(params);
    }

    // Metadata
    if !args.metadata.is_empty() {
        let metadata = parse_key_value_pairs(&args.metadata)?;
        builder = builder.metadata(metadata);
    }

    // Context loading
    if has_any_context(&args) {
        let contexts = build_contexts(&args)?;
        builder = builder.context(contexts);
    }

    // Tools (adds to defaults)
    if !args.tools.is_empty() {
        let tools = load_tools(&args.tools)?;
        builder = builder.tools(tools);
    }

    // MCP server
    if let (Some(bin), Some(init)) = (args.mcp_server_bin, args.mcp_server_init) {
        builder = builder.mcp_server::<Stdio>()
            .bin(bin.to_string_lossy())
            .init(init);
    }

    // Provider (only if explicitly requested)
    if let Some(model_id) = args.model {
        let provider = create_provider_from_model_id(&model_id).await?;
        builder = builder.completion_provider(provider);
    }
    // Otherwise: into_agent() uses Phi4Reasoning default

    // Chunk handler (always in code)
    builder = builder.on_chunk(|chunk| {
        match &chunk {
            CandleMessageChunk::Text(text) => print!("{}", text),
            CandleMessageChunk::Complete { text, .. } => print!("{}", text),
            other => print!("{:?}", other),
        }
        io::stdout().flush().ok();
        chunk
    });

    builder.into_agent().chat(|conversation| {
        let user_input = conversation.latest_user_message();
        match user_input.to_lowercase().as_str() {
            "quit" | "exit" | "bye" => CandleChatLoop::Break,
            _ => CandleChatLoop::Reprompt("How can I help you?".to_string())
        }
    })?;

    Ok(())
}
```

## Helper Functions Needed

```rust
fn parse_key_value_pairs(pairs: &[String]) -> Result<Vec<(String, String)>> {
    pairs.iter()
        .map(|s| {
            let mut parts = s.splitn(2, '=');
            let key = parts.next().ok_or("Missing key")?;
            let value = parts.next().ok_or("Missing value")?;
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

fn has_any_context(args: &Args) -> bool {
    !args.context_files.is_empty() ||
    !args.context_globs.is_empty() ||
    !args.context_directories.is_empty() ||
    !args.context_github.is_empty()
}

fn build_contexts(args: &Args) -> Result<(...)> {
    // TODO: Need to research CandleContext construction
    todo!("Research domain/context/provider.rs")
}

fn load_tools(tool_names: &[String]) -> Result<ZeroOneOrMany<ToolInfo>> {
    // TODO: Need to research tool loading
    todo!("Research domain/tool/*.rs")
}
```

## Research Still Needed

Before implementation:
1. How to construct `CandleContext<CandleFile>`, `CandleContext<CandleFiles>`, etc.
2. How to load tools by name and create `ToolInfo`
3. Confirm remaining builder field defaults (additional_params, metadata, context, handlers)

## Related Tasks

- `task/fix-agent-role-default.md` - Default model issue

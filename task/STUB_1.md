# STUB_1: Shell Execution Security Fix

**Priority:** 🔴 CRITICAL  
**Severity:** Security Vulnerability  
**Estimated Effort:** 1 session

## OBJECTIVE

Replace the Python VM placeholder implementation in `eval-sh` plugin with production-quality shell execution that properly handles shell syntax (pipes, redirections, environment variables) with comprehensive security validation.

## BACKGROUND

Current implementation uses `rlua` Python VM to execute what should be shell commands. This is fundamentally broken:
- Shell syntax doesn't work in Python VM
- Security model mismatch
- User expectations violated (requesting shell, getting Python)
- Plugin description claims "shell execution" but delivers Python

## LOCATION

**File:** `packages/sweetmcp/plugins/eval-sh/src/lib.rs`  
**Lines:** 45-52 (and related VM initialization code)

## SUBTASK 1: Remove Python VM Dependencies

**What:** Remove all `rlua` dependencies and Python VM code  
**Where:** 
- `packages/sweetmcp/plugins/eval-sh/Cargo.toml` - Remove rlua dependency
- `packages/sweetmcp/plugins/eval-sh/src/lib.rs` - Remove VM initialization, storage, and execution code

**Why:** Python VM cannot execute shell commands and is the wrong technology for this plugin

**Actions:**
1. Delete `VM_INSTANCES` DashMap
2. Delete `get_or_create_vm()` function
3. Remove rlua imports
4. Remove rlua from Cargo.toml

## SUBTASK 2: Implement ShellExecutor

**What:** Create production-grade shell executor with security validation  
**Where:** `packages/sweetmcp/plugins/eval-sh/src/lib.rs`

**Why:** Need proper shell execution with timeout, security checks, and resource limits

**Implementation:**
```rust
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::timeout;
use regex::Regex;

pub struct ShellExecutor {
    timeout: Duration,
    allowed_commands: Vec<String>,
    blocked_patterns: Vec<Regex>,
}

impl ShellExecutor {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            allowed_commands: vec![],
            blocked_patterns: vec![
                Regex::new(r"rm\s+-rf\s+/").unwrap(),
                Regex::new(r":\(\)\{.*\}").unwrap(),
            ],
        }
    }
    
    fn validate_command(&self, cmd: &str) -> Result<(), ShellError> {
        // Check blocked patterns
        for pattern in &self.blocked_patterns {
            if pattern.is_match(cmd) {
                return Err(ShellError::BlockedCommand(cmd.to_string()));
            }
        }
        
        // Check whitelist if configured
        if !self.allowed_commands.is_empty() {
            let cmd_base = cmd.split_whitespace().next().unwrap_or("");
            if !self.allowed_commands.contains(&cmd_base.to_string()) {
                return Err(ShellError::NotWhitelisted(cmd_base.to_string()));
            }
        }
        
        Ok(())
    }
    
    pub async fn execute(&self, command: &str) -> Result<ExecutionResult, ShellError> {
        self.validate_command(command)?;
        
        let child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
            
        let output = timeout(self.timeout, child.wait_with_output())
            .await
            .map_err(|_| ShellError::Timeout)??;
            
        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }
}

pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("Command blocked by security policy: {0}")]
    BlockedCommand(String),
    #[error("Command not in whitelist: {0}")]
    NotWhitelisted(String),
    #[error("Command execution timeout")]
    Timeout,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## SUBTASK 3: Integrate with MCP Tool Interface

**What:** Update `ShellTool` implementation to use `ShellExecutor`  
**Where:** `packages/sweetmcp/plugins/eval-sh/src/lib.rs` - `impl McpTool for ShellTool`

**Why:** Connect new shell executor to existing MCP plugin interface

**Implementation:**
```rust
impl McpTool for ShellTool {
    async fn call(&self, params: Value) -> Result<CallToolResult> {
        let command = params.get("command")
            .and_then(|v| v.as_str())
            .ok_or(anyhow!("Missing command parameter"))?;
            
        let executor = ShellExecutor::new();
        let result = executor.execute(command).await
            .map_err(|e| anyhow!("Shell execution failed: {}", e))?;
        
        Ok(CallToolResult {
            is_error: result.exit_code.map(|c| c != 0),
            content: vec![
                Content {
                    type_: "text".to_string(),
                    text: Some(result.stdout),
                    ..Default::default()
                },
                Content {
                    type_: "text".to_string(),
                    text: Some(result.stderr),
                    ..Default::default()
                },
            ],
        })
    }
}
```

## SUBTASK 4: Update Dependencies

**What:** Add required dependencies to Cargo.toml  
**Where:** `packages/sweetmcp/plugins/eval-sh/Cargo.toml`

**Dependencies to add:**
```toml
[dependencies]
tokio = { version = "1", features = ["process", "time"] }
regex = "1"
thiserror = "1"
```

## DEFINITION OF DONE

- [ ] All rlua/Python VM code removed
- [ ] ShellExecutor struct implemented with security validation
- [ ] Command execution works with timeout
- [ ] Security checks block dangerous patterns (rm -rf /, fork bombs)
- [ ] Whitelist support implemented (configurable)
- [ ] MCP Tool integration complete
- [ ] Error handling comprehensive (ShellError enum)
- [ ] Plugin description comment updated to remove "placeholder" reference
- [ ] Cargo.toml dependencies updated
- [ ] Code compiles without warnings
- [ ] Manual verification: shell pipes and redirections work correctly

## REQUIREMENTS

- ❌ **NO TESTS** - Testing team handles test coverage
- ❌ **NO BENCHMARKS** - Performance team handles benchmarking
- ✅ **PRODUCTION CODE ONLY** - All implementations must be complete, no stubs

## RESEARCH NOTES

### Security Considerations
- Default timeout: 30 seconds (configurable)
- Blocked patterns should include:
  - `rm -rf /` and variants
  - Fork bombs: `:(){:|:&};:`
  - Command injection attempts with backticks or $()
- Consider environment variable sanitization

### Shell Selection
Using `sh -c` provides POSIX-compliant shell without requiring specific shell (bash, zsh, etc.)

### Alternative Approaches Considered
- Nix sandbox: Too heavy for this use case
- Docker: Requires Docker daemon
- Landlock (from cylo): Could be future enhancement for file access restrictions

### Related Systems
- Consider integration with `packages/cylo` for enhanced sandboxing in future
- Could reuse resource tracking from cylo backend

## VERIFICATION

After implementation, verify:
1. Basic command: `echo "hello world"` produces output
2. Pipes work: `echo test | grep test`
3. Redirections work: `ls > /tmp/test.txt`
4. Security blocks: `rm -rf /` is rejected
5. Timeout works: `sleep 60` is terminated after 30s
6. Error capture: Invalid command produces stderr output

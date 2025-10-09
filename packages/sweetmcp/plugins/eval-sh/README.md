# eval-sh

Shell command execution plugin with security validation and sandboxing.

## Features

- Execute shell commands via MCP protocol
- Security validation blocks dangerous patterns:
  - `rm -rf /` and recursive deletion attempts
  - Fork bombs (`:(){:|:&};:`)
  - Command injection (backticks, subshells)
- 30-second timeout enforcement
- Supports pipes, redirections, and environment variables

## Architecture

Uses Extism host functions to delegate shell execution from WASM plugin to daemon:
- Plugin runs in WASM sandbox (no direct system access)
- Daemon provides `shell_execute` host function with security validation
- Results returned to plugin for MCP formatting

## Usage

```json
{
  "plugins": [
    {
      "name": "eval-sh",
      "path": "path/to/sweetmcp-plugin-eval-sh.wasm"
    }
  ]
}
```

## Security

Commands are validated against security policies:
- Blocked: `rm -rf /`, fork bombs, command injection patterns
- Timeout: 30 seconds (exit code 124 on timeout)
- Executed in controlled environment with validation

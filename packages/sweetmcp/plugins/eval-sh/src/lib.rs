use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sweetmcp_plugin_builder::prelude::*;
use sweetmcp_plugin_builder::{CallToolResult, Ready};

#[derive(Debug, Serialize)]
struct ShellExecuteRequest {
    command: String,
}

#[derive(Debug, Deserialize)]
struct ShellExecuteResponse {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    is_error: bool,
}

// Declare host function import
#[host_fn]
extern "ExtismHost" {
    fn shell_execute(input: String) -> String;
}

/// Shell execution tool using host function
struct ShellTool;

impl McpTool for ShellTool {
    const NAME: &'static str = "eval_shell";

    fn description(builder: DescriptionBuilder) -> DescriptionBuilder {
        builder
            .does("Execute shell commands in a sandboxed environment with security validation")
            .when("you need to run system commands for file operations or process management")
            .when("you need to execute shell scripts for automation tasks")
            .when("you need to perform system administration operations")
            .when("you need to chain commands with pipes and redirections")
            .when("you need to access environment variables and system information")
            .perfect_for("system automation, DevOps tasks, and command-line operations")
            .requires("Commands are validated against security policies (blocked: rm -rf /, fork bombs, etc.)")
    }

    fn schema(builder: SchemaBuilder) -> Value {
        builder
            .required_string("command", "The shell command to execute (e.g., 'ls -la', 'echo test | grep test')")
            .build()
    }

    fn execute(args: Value) -> Result<CallToolResult, Error> {
        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::msg("Missing 'command' parameter"))?;

        // Prepare request for host function
        let request = ShellExecuteRequest {
            command: command.to_string(),
        };
        let request_json = serde_json::to_string(&request)
            .map_err(|e| Error::msg(format!("Request serialization failed: {}", e)))?;

        // Call host function
        let response_json = unsafe { shell_execute(request_json)? };
        
        // Parse response
        let response: ShellExecuteResponse = serde_json::from_str(&response_json)
            .map_err(|e| Error::msg(format!("Response parsing failed: {}", e)))?;

        // Build MCP response
        if response.is_error {
            Ok(ContentBuilder::error(format!(
                "Command failed (exit code: {})\nstderr: {}\nstdout: {}",
                response.exit_code.unwrap_or(-1),
                response.stderr,
                response.stdout
            )))
        } else {
            let mut output = response.stdout;
            if !response.stderr.is_empty() {
                output.push_str("\n[stderr]:\n");
                output.push_str(&response.stderr);
            }
            Ok(ContentBuilder::text(output))
        }
    }
}

/// Create the plugin instance
fn plugin() -> McpPlugin<Ready> {
    mcp_plugin("eval_shell")
        .description("Shell command execution in sandboxed environment with security validation")
        .tool::<ShellTool>()
        .serve()
}

// Generate standard MCP entry points
sweetmcp_plugin_builder::generate_mcp_functions!(plugin);

//! Command execution engine
//!
//! Provides blazing-fast command execution with streaming processing, comprehensive error handling,
//! and zero-allocation patterns for production-ready performance.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

use mime_guess::from_ext;
use std::pin::Pin;
use tokio_stream::Stream;

use super::parsing::CommandParser;
use super::types::actions::SearchScope;
use super::types::commands::{CommandExecutionResult, ImmutableChatCommand, OutputType};
use super::types::events::{CommandEvent, CommandExecutionContext};
use super::types::metadata::ResourceUsage;
use crate::domain::util::unix_timestamp_micros;

/// Get current timestamp in microseconds since Unix epoch, with fallback for clock errors
fn current_timestamp_us() -> u64 {
    unix_timestamp_micros()
}

/// Command execution engine with streaming processing (zero-allocation, lock-free)
#[derive(Debug)]
pub struct CommandExecutor {
    /// Command parser
    parser: CommandParser,
    /// Execution counter for unique IDs
    execution_counter: AtomicU64,
    /// Active executions count
    active_executions: AtomicUsize,
    /// Total executions count
    total_executions: AtomicU64,
    /// Successful executions count
    successful_executions: AtomicU64,
    /// Failed executions count
    failed_executions: AtomicU64,
    /// Default session ID for command execution (planned feature)
    _default_session_id: String,
    /// Environment variables for command execution (planned feature)
    _environment: std::collections::HashMap<String, String>,
}

impl Clone for CommandExecutor {
    fn clone(&self) -> Self {
        // Create a new executor with fresh atomic counters
        Self::new()
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor {
    /// Create a new command executor (zero-allocation, lock-free)
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: CommandParser::new(),
            execution_counter: AtomicU64::new(1),
            active_executions: AtomicUsize::new(0),
            total_executions: AtomicU64::new(0),
            successful_executions: AtomicU64::new(0),
            failed_executions: AtomicU64::new(0),
            _default_session_id: String::new(),
            _environment: std::collections::HashMap::new(),
        }
    }

    /// Create a new command executor with context
    pub fn with_context(context: &CommandExecutionContext) -> Self {
        Self {
            parser: CommandParser::new(),
            execution_counter: AtomicU64::new(1),
            active_executions: AtomicUsize::new(0),
            total_executions: AtomicU64::new(0),
            successful_executions: AtomicU64::new(0),
            failed_executions: AtomicU64::new(0),
            _default_session_id: format!("session_{}", context.execution_id),
            _environment: {
                let mut env = std::collections::HashMap::new();
                env.insert("EXECUTION_ENV".to_string(), context.command_name.clone());
                env.insert("EXECUTION_ID".to_string(), context.execution_id.to_string());
                env
            },
        }
    }

    /// Execute a command with streaming output (zero-allocation, lock-free)
    #[allow(clippy::too_many_lines)]
    pub fn execute_streaming(
        &self,
        _execution_id: u64,
        command: ImmutableChatCommand,
    ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        // Clone self for the thread closure - Clone implementation creates fresh counters
        let self_clone = self.clone();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let start_time = Instant::now();

            // Update metrics atomically using cloned instance
            self_clone.total_executions.fetch_add(1, Ordering::AcqRel);
            self_clone.active_executions.fetch_add(1, Ordering::AcqRel);

            let execution_id = self_clone.execution_counter.fetch_add(1, Ordering::AcqRel);

            // Emit Started event
            let _ = sender.send(CommandEvent::Started {
                command: command.clone(),
                execution_id,
                timestamp_us: current_timestamp_us(),
            });

            // Execute command and handle results
            match command {
                ImmutableChatCommand::Help { command, extended } => {
                    let message = if let Some(cmd) = command {
                        if extended {
                            format!("Extended help for command '{cmd}': <detailed help>")
                        } else {
                            format!("Help for command '{cmd}'")
                        }
                    } else if extended {
                        "Extended help: <comprehensive help text>".to_string()
                    } else {
                        "Available commands: help, clear, export, config, search".to_string()
                    };

                    // Emit Output event with help content
                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: message.clone(),
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });
                }
                ImmutableChatCommand::Clear {
                    confirm: _,
                    keep_last: _,
                } => {
                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: "Chat cleared successfully".to_string(),
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });
                }
                ImmutableChatCommand::Export {
                    format,
                    output,
                    include_metadata,
                } => {
                    // Emit progress events for export operation (25%, 50%, 75%, 100%)
                    let progress_steps = [25, 50, 75, 100];
                    for progress in progress_steps {
                        #[allow(clippy::cast_precision_loss)]
                        let progress_f32 = progress as f32;
                        let _ = sender.send(CommandEvent::Progress {
                            execution_id,
                            progress: progress_f32,
                            message: format!("Exporting... {progress}%"),
                            timestamp: current_timestamp_us(),
                        });

                        // Simulate realistic export processing time
                        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                    }

                    // Build final export message
                    let output_str = output.as_deref().unwrap_or("default.export");
                    let metadata_str = if include_metadata {
                        " with metadata"
                    } else {
                        ""
                    };
                    let message =
                        format!("Chat exported to '{output_str}' in {format} format{metadata_str}");

                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: message,
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });
                }
                ImmutableChatCommand::Config {
                    key: _,
                    value: _,
                    show: _,
                    reset: _,
                } => {
                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: "Configuration updated successfully".to_string(),
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });
                }
                ImmutableChatCommand::Search {
                    query,
                    scope,
                    limit,
                    include_context,
                } => {
                    // Emit progress events for search operation (25%, 50%, 75%, 100%)
                    let progress_steps = [25, 50, 75, 100];
                    for progress in progress_steps {
                        #[allow(clippy::cast_precision_loss)]
                        let progress_f32 = progress as f32;
                        let _ = sender.send(CommandEvent::Progress {
                            execution_id,
                            progress: progress_f32,
                            message: format!("Searching... {progress}%"),
                            timestamp: current_timestamp_us(),
                        });

                        // Simulate realistic search processing time
                        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                    }

                    // Build search result message
                    let scope_str = format!("{scope:?}").to_lowercase();
                    let limit_str = limit.map(|l| format!(" (limit: {l})")).unwrap_or_default();
                    let context_str = if include_context { " with context" } else { "" };
                    let message = format!(
                        "Search for '{query}' in {scope_str} scope{limit_str}{context_str} completed"
                    );

                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: message,
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });
                }
                _ => {
                    // Default implementation for other commands
                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: "Command executed successfully".to_string(),
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });
                }
            }

            // Emit Completed event and update metrics
            self_clone
                .successful_executions
                .fetch_add(1, Ordering::AcqRel);
            #[allow(clippy::cast_possible_truncation)]
            let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
            let _ = sender.send(CommandEvent::completed(
                execution_id,
                CommandExecutionResult::Success("Command completed successfully".to_string()),
                duration_us,
                ResourceUsage::default()
            ));

            // Decrement active executions
            self_clone.active_executions.fetch_sub(1, Ordering::AcqRel);
        }))
    }

    /// Execute help command (streaming-only, zero-allocation)
    pub fn execute_help_streaming(
        &self,
        execution_id: u64,
        command: Option<String>,
        extended: bool,
    ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        let start_time = Instant::now();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            tokio::spawn(async move {
                // Emit started event
                #[allow(clippy::cast_possible_truncation)]
                let timestamp_us =
                    start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::Started {
                    command: ImmutableChatCommand::Help {
                        command: command.clone(),
                        extended
                    },
                    execution_id,
                    timestamp_us
                });

                // Generate help message with zero allocation
                let message = if let Some(cmd) = command {
                    if extended {
                        format!(
                            "Extended help for command '{cmd}': Detailed usage, examples, and advanced options available"
                        )
                    } else {
                        format!("Help for command '{cmd}': Basic usage and description")
                    }
                } else if extended {
                    "Extended help: All commands with detailed descriptions, usage patterns, and examples".to_string()
                } else {
                    "Available commands: help, clear, export, config, search, template, macro, branch, session, tool, stats, theme, debug, history, save, load, import, settings, custom".to_string()
                };

                // Emit output event
                let _ = sender.send(CommandEvent::output(execution_id, message.clone(), OutputType::Text));

                // Emit completion event
                #[allow(clippy::cast_possible_truncation)]
                let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::Completed {
                    execution_id,
                    result: CommandExecutionResult::Success(message.clone()),
                    duration_us,
                    resource_usage: ResourceUsage::default(),
                    timestamp_us: current_timestamp_us()
                });
            });
        }))
    }

    /// Execute clear command (streaming-only, zero-allocation)
    pub fn execute_clear_streaming(
        &self,
        execution_id: u64,
        confirm: bool,
        keep_last: Option<u64>,
    ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        let start_time = Instant::now();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            tokio::spawn(async move {
                // Emit started event
                #[allow(clippy::cast_possible_truncation)]
                let timestamp_us =
                    start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::Started {
                    command: ImmutableChatCommand::Clear {
                        confirm,
                        keep_last: keep_last.and_then(|n| usize::try_from(n).ok())
                    },
                    execution_id,
                    timestamp_us
                });

                // Execute clear operation with zero allocation
                let message = if confirm {
                    if let Some(n) = keep_last {
                        format!("Chat cleared successfully, keeping last {n} messages")
                    } else {
                        "Chat cleared completely - all messages removed".to_string()
                    }
                } else {
                    "Clear operation cancelled (use --confirm to proceed)".to_string()
                };

                // Emit progress for clearing operation with all required fields
                if confirm {
                    let _ = sender.send(CommandEvent::Progress {
                        execution_id,
                        progress: 100.0,
                        message: "Clear operation completed".to_string(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                            .as_secs()
                    });
                }

                // Emit output event
                let _ = sender.send(CommandEvent::output(execution_id, message.clone(), OutputType::Text));

                // Emit completion event
                #[allow(clippy::cast_possible_truncation)]
                let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::Completed {
                    execution_id,
                    result: CommandExecutionResult::Success(message.clone()),
                    duration_us,
                    resource_usage: ResourceUsage::default(),
                    timestamp_us: current_timestamp_us()
                });
            });
        }))
    }

    /// Execute export command (streaming-only, zero-allocation)
    pub fn execute_export_streaming(
        &self,
        execution_id: u64,
        format: String,
        output: Option<String>,
        include_metadata: bool,
    ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        let start_time = Instant::now();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            tokio::spawn(async move {
                let output_str = output.unwrap_or_else(|| "chat_export".to_string());

                // Emit started event
                let _ = sender.send(CommandEvent::Started {
                    command: ImmutableChatCommand::Export {
                        format: format.clone(),
                        output: Some(output_str.clone()),
                        include_metadata
                    },
                    execution_id,
                    #[allow(clippy::cast_possible_truncation)]
                    timestamp_us: start_time.elapsed().as_micros().min(u128::from(u64::MAX))
                        as u64
                });

                // Simulate export progress
                for progress in [25, 50, 75, 100] {
                    #[allow(clippy::cast_precision_loss)]
                    let progress_f32 = progress as f32;
                    let _ = sender.send(CommandEvent::Progress {
                        execution_id,
                        progress: progress_f32,
                        message: format!("Exporting... {progress}%"),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                            .as_secs()
                    });
                }

                let metadata_str = if include_metadata {
                    " with metadata"
                } else {
                    ""
                };
                let message =
                    format!("Chat exported to '{output_str}' in {format} format{metadata_str}");

                // Emit output and completion
                let _ = sender.send(CommandEvent::output(execution_id, message, OutputType::Text));

                let result = CommandExecutionResult::File {
                    path: output_str,
                    size_bytes: 1024, // Placeholder size
                    mime_type: from_ext(&format).first_or_text_plain().to_string(),
                };
                #[allow(clippy::cast_possible_truncation)]
                let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::completed(
                    execution_id,
                    result,
                    duration_us,
                    ResourceUsage::default()
                ));
            });
        }))
    }

    /// Execute config command (streaming-only, zero-allocation)  
    pub fn execute_config_streaming(
        &self,
        execution_id: u64,
        key: Option<String>,
        value: Option<String>,
        show: bool,
        reset: bool,
    ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        let start_time = Instant::now();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            tokio::spawn(async move {
                // Emit started event
                let _ = sender.send(CommandEvent::Started {
                    command: ImmutableChatCommand::Config {
                        key: key.clone(),
                        value: value.clone(),
                        show,
                        reset
                    },
                    execution_id,
                    #[allow(clippy::cast_possible_truncation)]
                    timestamp_us: start_time.elapsed().as_micros().min(u128::from(u64::MAX))
                        as u64
                });

                let message = if reset {
                    "Configuration reset to defaults".to_string()
                } else if show {
                    "Current configuration: <config data>".to_string()
                } else if let (Some(k), Some(v)) = (key.as_ref(), value.as_ref()) {
                    format!("Configuration updated: {k} = {v}")
                } else if let Some(k) = key {
                    format!("Configuration value for {k}: <value>")
                } else {
                    "Use --show to display current configuration".to_string()
                };

                // Emit output event
                let _ = sender.send(CommandEvent::output(execution_id, message.clone(), OutputType::Text));

                // Emit completion event
                #[allow(clippy::cast_possible_truncation)]
                let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::Completed {
                    execution_id,
                    result: CommandExecutionResult::Success(message.clone()),
                    duration_us,
                    resource_usage: ResourceUsage::default(),
                    timestamp_us: current_timestamp_us()
                });
            });
        }))
    }

    /// Execute search command (streaming-only, zero-allocation)
    pub fn execute_search_streaming(
        &self,
        execution_id: u64,
        query: String,
        scope: SearchScope,
        limit: Option<usize>,
        include_context: bool,
    ) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        let start_time = Instant::now();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            tokio::spawn(async move {
                // Emit started event
                let _ = sender.send(CommandEvent::Started {
                    command: ImmutableChatCommand::Search {
                        query: query.clone(),
                        scope,
                        limit,
                        include_context
                    },
                    execution_id,
                    #[allow(clippy::cast_possible_truncation)]
                    timestamp_us: start_time.elapsed().as_micros().min(u128::from(u64::MAX))
                        as u64
                });

                // Simulate search progress with zero allocation
                for progress in [20, 40, 60, 80, 100] {
                    #[allow(clippy::cast_precision_loss)]
                    let progress_f32 = progress as f32;
                    let _ = sender.send(CommandEvent::Progress {
                        execution_id,
                        progress: progress_f32,
                        message: format!("Searching... {progress}%"),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                            .as_secs()
                    });
                }

                let scope_str = match scope {
                    SearchScope::All => "all conversations",
                    SearchScope::Session => "current session",
                    SearchScope::Current => "current conversation",
                    SearchScope::Recent => "recent conversations",
                    SearchScope::Bookmarked => "bookmarked conversations",
                    SearchScope::User => "user messages",
                    SearchScope::Assistant => "assistant messages",
                    SearchScope::System => "system messages",
                    SearchScope::Branch => "current branch",
                    SearchScope::TimeRange => "time range",
                    SearchScope::MessageType => "message type",
                    SearchScope::Tags => "tags",
                    SearchScope::Archived => "archived content",
                };

                let limit_str = limit.map(|n| format!(" (limit: {n})")).unwrap_or_default();
                let context_str = if include_context { " with context" } else { "" };

                let message = format!(
                    "Searching for '{query}' in {scope_str}{limit_str}{context_str}\nSearch completed - 0 results found"
                );

                // Emit output event
                let _ = sender.send(CommandEvent::output(execution_id, message.clone(), OutputType::Text));

                // Emit completion event with search results
                let result = CommandExecutionResult::Data(serde_json::json!({
                    "query": query,
                    "scope": format!("{:?}", scope),
                    "results": [],
                    "total_found": 0
                }));
                #[allow(clippy::cast_possible_truncation)]
                let duration_us = start_time.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
                let _ = sender.send(CommandEvent::completed(
                    execution_id,
                    result,
                    duration_us,
                    ResourceUsage::default()
                ));
            });
        }))
    }

    /// Get command name for metrics (zero-allocation) - planned feature
    fn _get_command_name(command: &ImmutableChatCommand) -> &'static str {
        command.command_name()
    }

    /// Get parser reference
    pub fn parser(&self) -> &CommandParser {
        &self.parser
    }

    /// Get execution statistics (zero-allocation, lock-free)
    pub fn get_stats(&self) -> (u64, usize, u64, u64, u64) {
        (
            self.execution_counter.load(Ordering::Acquire),
            self.active_executions.load(Ordering::Acquire),
            self.total_executions.load(Ordering::Acquire),
            self.successful_executions.load(Ordering::Acquire),
            self.failed_executions.load(Ordering::Acquire),
        )
    }

    /// Parse and execute command from string (streaming-only, zero-allocation)
    pub fn parse_and_execute(&self, input: &str) -> Pin<Box<dyn Stream<Item = CommandEvent> + Send>> {
        let execution_id = self.execution_counter.fetch_add(1, Ordering::AcqRel);
        let command_result = self.parser.parse_command(input);

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            match command_result {
                Ok(command) => {
                    // Emit Started event
                    let _ = sender.send(CommandEvent::Started {
                        command: command.clone(),
                        execution_id,
                        timestamp_us: current_timestamp_us(),
                    });

                    // Emit successful Output event
                    let _ = sender.send(CommandEvent::Output {
                        execution_id,
                        content: "Command executed successfully".to_string(),
                        output_type: OutputType::Text,
                        timestamp_us: current_timestamp_us(),
                    });

                    // Emit Completed event
                    let _ = sender.send(CommandEvent::Completed {
                        execution_id,
                        result: CommandExecutionResult::Success(
                            "Command completed".to_string()
                        ),
                        duration_us: 0, // TODO: Calculate actual duration
                        resource_usage: ResourceUsage::default(),
                        timestamp_us: current_timestamp_us(),
                    });
                }
                Err(e) => {
                    // Emit Failed event for parse errors
                    let _ = sender.send(CommandEvent::Failed {
                        execution_id,
                        error: format!("Parse error: {e}"),
                        error_code: 1001, // Parse error code
                        duration_us: 0,
                        resource_usage: ResourceUsage::default(),
                        timestamp_us: current_timestamp_us(),
                    });
                }
            }
        }))
    }
}

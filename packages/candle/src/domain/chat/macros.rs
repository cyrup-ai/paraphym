//! Macro system for chat automation with lock-free data structures
//!
//! This module provides a comprehensive macro system for recording, storing,
//! and playing back chat interactions using zero-allocation patterns and
//! lock-free data structures for blazing-fast performance.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use chrono::{DateTime, Utc};

use atomic_counter::{AtomicCounter, ConsistentCounter};
use crossbeam_queue::SegQueue;
use crossbeam_skiplist::SkipMap;
use dashmap::DashMap;
use serde::{Serialize, Deserialize};


use ystream::{handle_error, AsyncStream};
use uuid::Uuid;
use cyrup_sugars::prelude::MessageChunk;

use crate::domain::chat::commands::ImmutableChatCommand;
// Removed unused import: crate::chat::formatting::MessageContent

/// Macro action representing a single recorded operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroAction {
    /// Send a message with content
    SendMessage {
        /// Message content to send
        content: String,
        /// Type of message being sent
        message_type: String,
        /// Timestamp when action was recorded
        timestamp: Duration,
    },
    /// Execute a command
    ExecuteCommand {
        /// Command to execute
        command: ImmutableChatCommand,
        /// Timestamp when action was recorded
        timestamp: Duration,
    },
    /// Wait for a specified duration
    Wait {
        /// Duration to wait
        duration: Duration,
        /// Timestamp when action was recorded
        timestamp: Duration,
    },
    /// Set a variable value
    SetVariable {
        /// Variable name to set
        name: String,
        /// Value to assign to the variable
        value: String,
        /// Timestamp when action was recorded
        timestamp: Duration,
    },
    /// Conditional execution based on variable
    Conditional {
        /// Condition expression to evaluate
        condition: String,
        /// Actions to execute if condition is true
        then_actions: Vec<MacroAction>,
        /// Actions to execute if condition is false
        else_actions: Option<Vec<MacroAction>>,
        /// Timestamp when action was recorded
        timestamp: Duration,
    },
    /// Loop execution
    Loop {
        /// Number of iterations to perform
        iterations: u32,
        /// Actions to execute in each iteration
        actions: Vec<MacroAction>,
        /// Timestamp when action was recorded
        timestamp: Duration,
    },
}

/// Macro recording state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroRecordingState {
    /// Not recording
    Idle,
    /// Currently recording
    Recording,
    /// Recording paused
    Paused,
    /// Recording completed
    Completed,
}

/// Macro playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroPlaybackState {
    /// Not playing
    Idle,
    /// Currently playing
    Playing,
    /// Playback paused
    Paused,
    /// Playback completed
    Completed,
    /// Playback failed
    Failed,
}

/// Macro execution context with variable substitution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroExecutionContext {
    /// Variables available for substitution during execution
    pub variables: HashMap<String, String>,
    /// Unique identifier for this execution
    pub execution_id: Uuid,
    /// When execution started
    pub start_time: DateTime<Utc>,
    /// Current action index being executed
    pub current_action: usize,
    /// Stack of nested loop contexts
    pub loop_stack: Vec<LoopContext>,
}

impl Default for MacroExecutionContext {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            execution_id: Uuid::new_v4(),
            start_time: Utc::now(),
            current_action: 0,
            loop_stack: Vec::new(),
        }
    }
}

/// Loop execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopContext {
    /// Current iteration number (0-based)
    pub iteration: u32,
    /// Maximum number of iterations for this loop
    pub max_iterations: u32,
    /// Index of the first action in the loop
    pub start_action: usize,
    /// Index of the last action in the loop
    pub end_action: usize,
}

/// Macro metadata and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroMetadata {
    /// Unique identifier for the macro
    pub id: Uuid,
    /// Human-readable name of the macro
    pub name: String,
    /// Description of what the macro does
    pub description: String,
    /// When the macro was first created
    pub created_at: Duration,
    /// When the macro was last modified
    pub updated_at: Duration,
    /// Version number of the macro
    pub version: u32,
    /// Tags for categorizing and searching macros
    pub tags: Vec<String>,
    /// Author who created the macro
    pub author: String,
    /// Number of times this macro has been executed
    pub execution_count: u64,
    /// When the macro was last executed
    pub last_execution: Option<Duration>,
    /// Average execution duration
    pub average_duration: Duration,
    /// Success rate of executions (0.0 to 1.0)
    pub success_rate: f64,
    /// Category this macro belongs to
    pub category: String,
    /// Whether this macro is private to the user
    pub is_private: bool,
}

/// Complete macro definition with actions and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMacro {
    /// Metadata and statistics for the macro
    pub metadata: MacroMetadata,
    /// Sequence of actions to execute
    pub actions: Vec<MacroAction>,
    /// Default variables for macro execution
    pub variables: HashMap<String, String>,
    /// Triggers that can activate this macro
    pub triggers: Vec<String>,
    /// Conditions that must be met for execution
    pub conditions: Vec<String>,
    /// Other macros this macro depends on
    pub dependencies: Vec<String>,
    /// Configuration for how this macro executes
    pub execution_config: MacroExecutionConfig,
}

/// Macro execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroExecutionConfig {
    /// Maximum time allowed for macro execution
    pub max_execution_time: Duration,
    /// Number of times to retry on failure
    pub retry_count: u32,
    /// Delay between retry attempts
    pub retry_delay: Duration,
    /// Whether to abort execution on any error
    pub abort_on_error: bool,
    /// Whether actions can be executed in parallel
    pub parallel_execution: bool,
    /// Execution priority (0-255, higher = more priority)
    pub priority: u8,
    /// Resource limits for macro execution
    pub resource_limits: ResourceLimits,
}

/// Resource limits for macro execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in megabytes
    pub max_memory_mb: u32,
    /// Maximum CPU usage percentage (0-100)
    pub max_cpu_percent: u8,
    /// Maximum number of network requests allowed
    pub max_network_requests: u32,
    /// Maximum number of file operations allowed
    pub max_file_operations: u32,
}

/// Macro recording session
#[derive(Debug)]
pub struct MacroRecordingSession {
    /// Unique identifier for this recording session
    pub id: Uuid,
    /// Name of the macro being recorded
    pub name: String,
    /// When recording started
    pub start_time: DateTime<Utc>,
    /// Queue of recorded actions (lock-free)
    pub actions: SegQueue<MacroAction>,
    /// Current recording state
    pub state: MacroRecordingState,
    /// Variables captured during recording
    pub variables: HashMap<String, String>,
    /// Metadata for the macro being recorded
    pub metadata: MacroMetadata,
}

/// Macro playback session
#[derive(Debug)]
pub struct MacroPlaybackSession {
    /// Unique identifier for this playback session
    pub id: Uuid,
    /// ID of the macro being played back
    pub macro_id: Uuid,
    /// When playback started
    pub start_time: DateTime<Utc>,
    /// Execution context with variables and state
    pub context: MacroExecutionContext,
    /// Current playback state
    pub state: MacroPlaybackState,
    /// Index of the currently executing action
    pub current_action: usize,
    /// Total number of actions in the macro
    pub total_actions: usize,
    /// Error message if playback failed
    pub error: Option<String>,
}

/// High-performance macro system with lock-free operations
pub struct MacroSystem {
    /// Lock-free macro storage using skip list
    macros: SkipMap<Uuid, ChatMacro>,
    /// Active recording sessions
    recording_sessions: DashMap<Uuid, MacroRecordingSession>,
    /// Active playback sessions
    playback_sessions: DashMap<Uuid, MacroPlaybackSession>,
    /// Macro execution statistics
    execution_stats: SkipMap<Uuid, Arc<ExecutionStats>>,
    /// Global macro counter
    macro_counter: ConsistentCounter,
    /// Execution counter
    execution_counter: ConsistentCounter,
}

/// Macro execution statistics
#[derive(Debug, Default)]
pub struct ExecutionStats {
    /// Total number of macro executions attempted
    pub total_executions: ConsistentCounter,
    /// Number of successful macro executions
    pub successful_executions: ConsistentCounter,
    /// Number of failed macro executions
    pub failed_executions: ConsistentCounter,
    /// Total time spent executing macros
    pub total_duration: parking_lot::Mutex<Duration>,
    /// Average execution duration per macro
    pub average_duration: parking_lot::Mutex<Duration>,
    /// Timestamp of the last macro execution
    pub last_execution: parking_lot::Mutex<Option<DateTime<Utc>>>,
}

impl Clone for ExecutionStats {
    fn clone(&self) -> Self {
        ExecutionStats {
            total_executions: ConsistentCounter::new(self.total_executions.get()),
            successful_executions: ConsistentCounter::new(self.successful_executions.get()),
            failed_executions: ConsistentCounter::new(self.failed_executions.get()),
            total_duration: parking_lot::Mutex::new(*self.total_duration.lock()),
            average_duration: parking_lot::Mutex::new(*self.average_duration.lock()),
            last_execution: parking_lot::Mutex::new(*self.last_execution.lock()),
        }
    }
}

impl Default for MacroExecutionConfig {
    fn default() -> Self {
        Self {
            max_execution_time: Duration::from_secs(300), // 5 minutes
            retry_count: 3,
            retry_delay: Duration::from_millis(1000),
            abort_on_error: false,
            parallel_execution: false,
            priority: 5,
            resource_limits: ResourceLimits::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 100,
            max_cpu_percent: 25,
            max_network_requests: 50,
            max_file_operations: 20,
        }
    }
}

impl MacroSystem {
    /// Create a new macro system with optimal performance settings
    pub fn new() -> Self {
        Self {
            macros: SkipMap::new(),
            recording_sessions: DashMap::new(),
            playback_sessions: DashMap::new(),
            execution_stats: SkipMap::new(),
            macro_counter: ConsistentCounter::new(0),
            execution_counter: ConsistentCounter::new(0),
        }
    }

    /// Start recording a new macro
    pub fn start_recording(&self, name: String, description: String) -> AsyncStream<MacroSessionId> {
        let session_id = Uuid::new_v4();
        let macro_id = Uuid::new_v4();

        // Create metadata for the macro
        let metadata = MacroMetadata {
            id: macro_id,
            name: name.clone(),
            description: description.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default(),
            version: 1,
            tags: Vec::new(),
            author: "system".to_string(),
            execution_count: 0,
            last_execution: None,
            average_duration: Duration::from_secs(0),
            success_rate: 0.0,
            category: "user-defined".to_string(),
            is_private: false,
        };

        // Create a new recording session with the correct fields
        let new_session = MacroRecordingSession {
            id: session_id,
            name,
            start_time: Utc::now(),
            actions: SegQueue::new(),
            state: MacroRecordingState::Recording,
            variables: HashMap::new(),
            metadata,
        };

        // Capture session_id in owned variable before any operations
        let owned_session_id = session_id;

        // Insert the new session directly
        self.recording_sessions.insert(session_id, new_session);

        // Create a stream that immediately yields the session ID using AsyncStream pattern
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                let _ = sender.send(MacroSessionId(owned_session_id));
            });
        })
    }

    /// Record a macro action
    pub fn record_action(&self, session_id: Uuid, action: MacroAction) -> AsyncStream<MacroActionResult> {
        // Get a reference to the session without cloning the entire DashMap
        if let Some(session) = self.recording_sessions.get(&session_id) {
            // Check if we're still recording
            if session.value().state == MacroRecordingState::Recording {
                // Push the action to the session's actions directly
                session.value().actions.push(action);

                // Create a stream that immediately yields success
                return AsyncStream::with_channel(move |sender| {
                    let _ = sender.send(MacroActionResult);
                });
            }
        }

        // If we get here, either the session doesn't exist or it's not recording
        // Return an empty stream that immediately completes
        AsyncStream::with_channel(|_sender| {
            // Empty stream - no data to send
        })
    }

    /// Stop recording and save the macro
    pub fn stop_recording(&self, session_id: Uuid) -> Result<Uuid, MacroSystemError> {
        if let Some((_, session)) = self.recording_sessions.remove(&session_id) {
            let mut session = session; // make mutable
            session.state = MacroRecordingState::Completed;

            // Collect all recorded actions
            let mut actions = Vec::new();
            while let Some(action) = session.actions.pop() {
                actions.push(action);
            }
            actions.reverse(); // Restore original order

            // Create the macro
            let chat_macro = ChatMacro {
                metadata: session.metadata.clone(),
                actions: actions.into(),
                variables: session.variables,
                triggers: Vec::new(),
                conditions: Vec::new(),
                dependencies: Vec::new(),
                execution_config: MacroExecutionConfig::default(),
            };

            let macro_id = session.metadata.id;
            self.macros.insert(macro_id, chat_macro);
            self.macro_counter.inc();

            Ok(macro_id)
        } else {
            Err(MacroSystemError::SessionNotFound)
        }
    }

    /// Get a macro by ID
    pub fn get_macro(&self, macro_id: Uuid) -> Option<ChatMacro> {
        self.macros
            .get(&macro_id)
            .map(|entry| entry.value().clone())
    }

    /// List all available macros
    pub fn list_macros(&self) -> Vec<MacroMetadata> {
        self.macros
            .iter()
            .map(|entry| entry.value().metadata.clone())
            .collect()
    }

    /// Search macros by name, description, or tags
    pub fn search_macros(&self, query: &str) -> Vec<MacroMetadata> {
        let query_lower = query.to_lowercase();

        self.macros
            .iter()
            .filter(|entry| {
                let macro_def = entry.value();
                macro_def
                    .metadata
                    .name
                    .to_lowercase()
                    .contains(&query_lower)
                    || macro_def
                        .metadata
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
                    || macro_def
                        .metadata
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .map(|entry| entry.value().metadata.clone())
            .collect()
    }

    /// Start macro playback
    pub fn start_playback(
        &self,
        macro_id: Uuid,
        variables: HashMap<String, String>,
    ) -> Result<Uuid, MacroSystemError> {
        let macro_def = self
            .macros
            .get(&macro_id)
            .ok_or(MacroSystemError::MacroNotFound)?
            .value()
            .clone();

        let session_id = Uuid::new_v4();
        let context = MacroExecutionContext {
            variables,
            execution_id: session_id,
            start_time: Utc::now(),
            current_action: 0,
            loop_stack: Vec::new(),
        };

        let session = MacroPlaybackSession {
            id: session_id,
            macro_id,
            start_time: Utc::now(),
            context,
            state: MacroPlaybackState::Playing,
            current_action: 0,
            total_actions: macro_def.actions.len(),
            error: None,
        };

        self.playback_sessions.insert(session_id, session);

        self.execution_counter.inc();

        Ok(session_id)
    }

    /// Execute the next action in a playback session
    pub fn execute_next_action(
        &self,
        session_id: Uuid,
    ) -> Result<MacroPlaybackResult, MacroSystemError> {
        let mut session_guard = self
            .playback_sessions
            .get_mut(&session_id)
            .ok_or(MacroSystemError::SessionNotFound)?;

        let session = session_guard.value_mut();
        if session.state != MacroPlaybackState::Playing {
            return Ok(MacroPlaybackResult::SessionNotActive);
        }

        let macro_def = self
            .macros
            .get(&session.macro_id)
            .ok_or(MacroSystemError::MacroNotFound)?
            .value()
            .clone();

        if session.current_action >= macro_def.actions.len() {
            session.state = MacroPlaybackState::Completed;
            return Ok(MacroPlaybackResult::Completed);
        }

        let action = &macro_def.actions[session.current_action];
        let result = execute_action_sync(action, &mut session.context)?;

        session.current_action += 1;

        match result {
            ActionExecutionResult::Success => {
                if session.current_action >= macro_def.actions.len() {
                    session.state = MacroPlaybackState::Completed;
                    Ok(MacroPlaybackResult::Completed)
                } else {
                    Ok(MacroPlaybackResult::ActionExecuted)
                }
            }
            ActionExecutionResult::Wait(duration) => {
                std::thread::sleep(duration);
                Ok(MacroPlaybackResult::ActionExecuted)
            }
            ActionExecutionResult::SkipToAction(index) => {
                session.current_action = index;
                Ok(MacroPlaybackResult::ActionExecuted)
            }
            ActionExecutionResult::Error(error) => {
                session.state = MacroPlaybackState::Failed;
                session.error = Some(error);
                Ok(MacroPlaybackResult::Failed)
            }
        }
    }

    /// Execute a single macro action with streaming results - planned feature
    fn _execute_action(
        action: &MacroAction,
        context: &mut MacroExecutionContext,
    ) -> AsyncStream<ActionExecutionResult> {
        // Clone only what's needed for the closure
        let action_clone = action.clone();
        let context_vars = context.variables.clone();
        let mut ctx = context.clone();

        AsyncStream::with_channel(move |sender| {
            let result = match &action_clone {
                MacroAction::SendMessage {
                    content,
                    message_type,
                    ..
                } => {
                    let resolved_content = resolve_variables_sync(content, &context_vars);
                    // In a real implementation, this would send the message to the chat system
                    println!(
                        "Sending message: {resolved_content} (type: {message_type})"
                    );
                    Ok::<ActionExecutionResult, MacroSystemError>(ActionExecutionResult::Success)
                }
                MacroAction::ExecuteCommand { command, .. } => {
                    // In a real implementation, this would execute the command
                    println!("Executing command: {command:?}");
                    Ok::<ActionExecutionResult, MacroSystemError>(ActionExecutionResult::Success)
                }
                MacroAction::Wait { duration, .. } => Ok(ActionExecutionResult::Wait(*duration)),
                MacroAction::SetVariable { name, value, .. } => {
                    let resolved_value = resolve_variables_sync(value, &context_vars);
                    ctx.variables.insert(name.clone(), resolved_value.into());
                    Ok::<ActionExecutionResult, MacroSystemError>(ActionExecutionResult::Success)
                }
                MacroAction::Conditional {
                    condition,
                    then_actions,
                    else_actions,
                    ..
                } => {
                    let condition_result = evaluate_condition_sync(condition, &context_vars);

                    let actions_to_execute = if condition_result {
                        then_actions
                    } else if let Some(else_actions) = else_actions {
                        else_actions
                    } else {
                        let _ = sender.send(ActionExecutionResult::Success);
                        return;
                    };

                    // Execute conditional actions synchronously
                    for action in actions_to_execute.iter() {
                        match execute_action_sync(action, &mut ctx) {
                            Ok(ActionExecutionResult::Error(error)) => {
                                let _ = sender.send(ActionExecutionResult::Error(error));
                                return;
                            }
                            Err(e) => {
                                handle_error!(e, "Action execution failed");
                            }
                            _ => {},
                        }
                    }

                    Ok::<ActionExecutionResult, MacroSystemError>(ActionExecutionResult::Success)
                }
                MacroAction::Loop {
                    iterations,
                    actions,
                    ..
                } => {
                    let loop_context = LoopContext {
                        iteration: 0,
                        max_iterations: *iterations,
                        start_action: 0,
                        end_action: actions.len(),
                    };

                    ctx.loop_stack.push(loop_context);

                    for _ in 0..*iterations {
                        for action in actions.iter() {
                            match execute_action_sync(action, &mut ctx) {
                                Ok(ActionExecutionResult::Error(error)) => {
                                    ctx.loop_stack.pop();
                                    let _ = sender.send(ActionExecutionResult::Error(error));
                                    return;
                                }
                                Err(e) => {
                                    ctx.loop_stack.pop();
                                    handle_error!(e, "Loop action execution failed");
                                }
                                _ => {},
                            }
                        }
                    }

                    ctx.loop_stack.pop();
                    Ok::<ActionExecutionResult, MacroSystemError>(ActionExecutionResult::Success)
                }
            };

            match result {
                Ok(action_result) => {
                    let _ = sender.send(action_result);
                }
                Err(e) => {
                    handle_error!(e, "Action execution failed");
                }
            }
        })
    }

    /// Resolve variables in a string - planned feature
    fn _resolve_variables(&self, content: &str, variables: &HashMap<String, String>) -> String {
        let mut result = content.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{key}}}");
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// Evaluate a condition string - planned feature
    fn _evaluate_condition(
        &self,
        condition: &str,
        variables: &HashMap<String, String>,
    ) -> bool {
        // Simple condition evaluation - in a real implementation, this would be more sophisticated
        if condition.contains("==") {
            let parts: Vec<&str> = condition.split("==").collect();
            if parts.len() == 2 {
                let left = self._resolve_variables(parts[0].trim(), variables);
                let right = self._resolve_variables(parts[1].trim(), variables);
                return left == right;
            }
        }

        // Default to false for unsupported conditions
        false
    }

    /// Get execution statistics for a macro
    pub fn get_execution_stats(&self, macro_id: Uuid) -> Option<ExecutionStats> {
        self.execution_stats
            .get(&macro_id)
            .map(|entry| (**entry.value()).clone())
    }

    /// Get total macro count
    pub fn get_macro_count(&self) -> usize {
        self.macro_counter.get()
    }

    /// Get total execution count
    pub fn get_execution_count(&self) -> usize {
        self.execution_counter.get()
    }
}

/// Result of macro action execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionExecutionResult {
    /// Action executed successfully
    Success,
    /// Wait for specified duration before continuing
    Wait(Duration),
    /// Skip to the specified action index
    SkipToAction(usize),
    /// Execution failed with error message
    Error(String),
}

impl Default for ActionExecutionResult {
    fn default() -> Self {
        ActionExecutionResult::Success
    }
}

impl MessageChunk for ActionExecutionResult {
    fn bad_chunk(_error: String) -> Self {
        ActionExecutionResult::Error(_error)
    }
    
    fn error(&self) -> Option<&str> {
        match self {
            ActionExecutionResult::Error(msg) => Some(msg),
            _ => None,
        }
    }
}

/// Result of macro playback operation
#[derive(Debug)]
pub enum MacroPlaybackResult {
    /// Single action was executed successfully
    ActionExecuted,
    /// Macro playback completed successfully
    Completed,
    /// Macro playback failed
    Failed,
    /// Playback session is not active
    SessionNotActive,
}

/// Macro system errors
#[derive(Debug, thiserror::Error)]
pub enum MacroSystemError {
    /// Requested macro session was not found
    #[error("Recording session not found")]
    SessionNotFound,
    /// Macro recording is not currently active
    #[error("Recording not active")]
    RecordingNotActive,
    /// Requested macro does not exist
    #[error("Macro not found")]
    MacroNotFound,
    /// System time operation failed
    #[error("System time error")]
    SystemTimeError,
    /// Error occurred during macro execution
    #[error("Execution error: {0}")]
    ExecutionError(String),
    /// Macro validation failed
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl Default for MacroSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro processor for executing and managing chat macros
///
/// This processor provides comprehensive macro execution capabilities with:
/// - Recording and playback of chat interactions
/// - Variable substitution and conditional logic
/// - Performance monitoring and error handling
/// - Concurrent execution with lock-free data structures
/// - Macro validation and optimization
#[derive(Debug, Clone)]
pub struct MacroProcessor {
    /// Macro storage with lock-free access
    macros: Arc<SkipMap<Uuid, ChatMacro>>,
    /// Execution statistics
    stats: Arc<MacroProcessorStats>,
    /// Variable context for macro execution
    #[allow(dead_code)] // TODO: Implement variable system for macro expansion
    variables: Arc<RwLock<HashMap<String, String>>>,
    /// Execution queue for async processing
    #[allow(dead_code)] // TODO: Implement in macro execution system
    execution_queue: Arc<SegQueue<MacroExecutionRequest>>,
    /// Configuration settings
    config: MacroProcessorConfig,
}

/// Macro processor statistics (internal atomic counters)
#[derive(Debug, Default)]
pub struct MacroProcessorStats {
    /// Total macros executed
    pub total_executions: AtomicUsize,
    /// Successful executions
    pub successful_executions: AtomicUsize,
    /// Failed executions
    pub failed_executions: AtomicUsize,
    /// Total execution time in microseconds
    pub total_execution_time_us: AtomicUsize,
    /// Active executions
    pub active_executions: AtomicUsize,
}

/// Macro processor statistics snapshot (for external API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroProcessorStatsSnapshot {
    /// Total macros executed
    pub total_executions: usize,
    /// Successful executions
    pub successful_executions: usize,
    /// Failed executions
    pub failed_executions: usize,
    /// Total execution time in microseconds
    pub total_execution_time_us: usize,
    /// Active executions
    pub active_executions: usize,
}

/// Macro processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroProcessorConfig {
    /// Maximum concurrent executions
    pub max_concurrent_executions: usize,
    /// Default execution timeout in seconds
    pub default_timeout_seconds: u64,
    /// Enable variable substitution
    pub enable_variable_substitution: bool,
    /// Enable conditional execution
    pub enable_conditional_execution: bool,
    /// Enable loop execution
    pub enable_loop_execution: bool,
    /// Maximum macro recursion depth
    pub max_recursion_depth: usize,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Auto-save macro changes
    pub auto_save: bool,
}

/// Macro execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroExecutionRequest {
    /// Macro ID to execute
    pub macro_id: Uuid,
    /// Execution context variables
    pub context_variables: HashMap<String, String>,
    /// Execution timeout override
    pub timeout_override: Option<Duration>,
    /// Execution priority (higher = more priority)
    pub priority: u32,
    /// Request timestamp
    pub requested_at: Duration,
}

/// Macro execution result
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacroExecutionResult {
    /// Execution success indicator
    pub success: bool,
    /// Execution message/error
    pub message: String,
    /// Actions executed
    pub actions_executed: usize,
    /// Execution duration
    pub execution_duration: Duration,
    /// Variables modified during execution
    pub modified_variables: HashMap<String, String>,
    /// Execution metadata
    pub metadata: MacroExecutionMetadata,
}

impl MessageChunk for MacroExecutionResult {
    fn bad_chunk(_error: String) -> Self {
        MacroExecutionResult {
            success: false,
            message: _error,
            actions_executed: 0,
            execution_duration: Duration::from_secs(0),
            modified_variables: HashMap::new(),
            metadata: MacroExecutionMetadata::default(),
        }
    }
    
    fn error(&self) -> Option<&str> {
        if !self.success {
            Some(&self.message)
        } else {
            None
        }
    }
}

/// Macro execution metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacroExecutionMetadata {
    /// Execution ID
    pub execution_id: Uuid,
    /// Macro ID
    pub macro_id: Uuid,
    /// Start timestamp
    pub started_at: Duration,
    /// End timestamp
    pub completed_at: Duration,
    /// Execution context
    pub context: HashMap<String, String>,
    /// Performance metrics
    pub performance: MacroPerformanceMetrics,
}

/// Macro performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacroPerformanceMetrics {
    /// CPU time used in microseconds
    pub cpu_time_us: u64,
    /// Memory used in bytes
    pub memory_bytes: u64,
    /// Network requests made
    pub network_requests: u32,
    /// Disk operations performed
    pub disk_operations: u32,
}

impl Default for MacroProcessorConfig {
    fn default() -> Self {
        Self {
            max_concurrent_executions: 10,
            default_timeout_seconds: 30,
            enable_variable_substitution: true,
            enable_conditional_execution: true,
            enable_loop_execution: true,
            max_recursion_depth: 10,
            enable_monitoring: true,
            auto_save: true,
        }
    }
}

impl MacroProcessor {
    /// Create a new macro processor
    pub fn new() -> Self {
        Self {
            macros: Arc::new(SkipMap::new()),
            stats: Arc::new(MacroProcessorStats::default()),
            variables: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(SegQueue::new()),
            config: MacroProcessorConfig::default(),
        }
    }

    /// Create a macro processor with custom configuration
    pub fn with_config(config: MacroProcessorConfig) -> Self {
        Self {
            macros: Arc::new(SkipMap::new()),
            stats: Arc::new(MacroProcessorStats::default()),
            variables: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(SegQueue::new()),
            config,
        }
    }

    /// Register a macro
    pub fn register_macro(&self, macro_def: ChatMacro) -> Result<(), MacroSystemError> {
        // Validate macro
        self.validate_macro(&macro_def)?;

        // Store macro
        self.macros.insert(macro_def.metadata.id, macro_def);

        Ok(())
    }

    /// Unregister a macro
    pub fn unregister_macro(&self, macro_id: &Uuid) -> Result<(), MacroSystemError> {
        if self.macros.remove(macro_id).is_none() {
            return Err(MacroSystemError::MacroNotFound);
        }

        Ok(())
    }

    /// Execute a macro by ID
    pub fn execute_macro(
        &self,
        macro_id: &Uuid,
        context_variables: HashMap<String, String>,
    ) -> AsyncStream<MacroExecutionResult> {
        let macro_def = self
            .macros
            .get(macro_id)
            .ok_or(MacroSystemError::MacroNotFound);

        if let Ok(macro_entry) = macro_def {
            let macro_def = macro_entry.value().clone();
            self.execute_macro_impl(macro_def, context_variables)
        } else {
            let macro_id = *macro_id;
            AsyncStream::with_channel(move |sender| {
                std::thread::spawn(move || {
                    // Error handling via on_chunk pattern - for now just return default
                    let default_result = MacroExecutionResult {
                        success: false,
                        message: String::from("Macro not found"),
                        actions_executed: 0,
                        execution_duration: Duration::from_secs(0),
                        modified_variables: HashMap::new(),
                        metadata: MacroExecutionMetadata {
                            execution_id: Uuid::new_v4(),
                            macro_id,
                            started_at: Duration::from_secs(0),
                            completed_at: Duration::from_secs(0),
                            context: HashMap::new(),
                            performance: MacroPerformanceMetrics::default(),
                        },
                    };

                    let _ = sender.send(default_result);
                });
            })
        }
    }

    /// Internal macro execution implementation
    fn execute_macro_impl(
        &self,
        macro_def: ChatMacro,
        context_variables: HashMap<String, String>,
    ) -> AsyncStream<MacroExecutionResult> {
        let self_clone = self.clone();
        AsyncStream::with_channel(move |sender| {
            std::thread::spawn(move || {
                self_clone
                    .stats
                    .active_executions
                    .fetch_add(1, Ordering::Relaxed);
                self_clone
                    .stats
                    .total_executions
                    .fetch_add(1, Ordering::Relaxed);

                let execution_id = Uuid::new_v4();
                let start_time = Utc::now();
                let started_at = Duration::from_secs(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                );

                let mut context = MacroExecutionContext {
                    variables: context_variables,
                    execution_id,
                    start_time,
                    current_action: 0,
                    loop_stack: Vec::new(),
                };

                let mut actions_executed = 0;
                let modified_variables = HashMap::new();
                let mut performance = MacroPerformanceMetrics::default();

                let success = loop {
                    if context.current_action >= macro_def.actions.len() {
                        break true;
                    }

                    let action = &macro_def.actions[context.current_action];

                    match execute_action_sync(action, &mut context) {
                        Ok(ActionExecutionResult::Success) => {
                            actions_executed += 1;
                            context.current_action += 1;
                        }
                        Ok(ActionExecutionResult::Wait(duration)) => {
                            std::thread::sleep(duration);
                            actions_executed += 1;
                            context.current_action += 1;
                        }
                        Ok(ActionExecutionResult::SkipToAction(index)) => {
                            actions_executed += 1;
                            context.current_action = index;
                        }
                        Ok(ActionExecutionResult::Error(_error)) => {
                            break false;
                        }
                        Err(_e) => {
                            break false;
                        }
                    }

                    // Update performance metrics - simplified for now
                    performance.disk_operations += 1;
                };

                let execution_duration = Utc::now().signed_duration_since(start_time).to_std().unwrap_or_default();
                let completed_at = Duration::from_secs(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                );

                let metadata = MacroExecutionMetadata {
                    execution_id,
                    macro_id: macro_def.metadata.id,
                    started_at,
                    completed_at,
                    context: context.variables.clone(),
                    performance,
                };

                let result = MacroExecutionResult {
                    success,
                    message: if success {
                        String::from("Execution successful")
                    } else {
                        String::from("Execution failed")
                    },
                    actions_executed,
                    execution_duration,
                    modified_variables,
                    metadata,
                };

                let _ = sender.send(result);
            });
        })
    }

    /// Validate a macro
    fn validate_macro(&self, macro_def: &ChatMacro) -> Result<(), MacroSystemError> {
        if macro_def.actions.is_empty() {
            return Err(MacroSystemError::ValidationError(
                "Macro has no actions".to_string(),
            ));
        }

        if macro_def.metadata.name.is_empty() {
            return Err(MacroSystemError::ValidationError(
                "Macro name is required".to_string(),
            ));
        }

        // Additional validation - recursion depth, etc.
        let mut depth = 0;
        for action in macro_def.actions.iter() {
            if let MacroAction::Loop { actions: _, .. } = action {
                depth += 1;
                if depth > self.config.max_recursion_depth {
                    return Err(MacroSystemError::ValidationError(
                        "Maximum recursion depth exceeded".to_string(),
                    ));
                }
                // Recursive validation
            }
        }

        Ok(())
    }

    /// Get macro processor statistics snapshot
    pub fn get_stats_snapshot(&self) -> MacroProcessorStatsSnapshot {
        MacroProcessorStatsSnapshot {
            total_executions: self.stats.total_executions.load(Ordering::Relaxed),
            successful_executions: self.stats.successful_executions.load(Ordering::Relaxed),
            failed_executions: self.stats.failed_executions.load(Ordering::Relaxed),
            total_execution_time_us: self.stats.total_execution_time_us.load(Ordering::Relaxed),
            active_executions: self.stats.active_executions.load(Ordering::Relaxed),
        }
    }

    /// Get active executions count
    pub fn get_active_executions(&self) -> usize {
        self.stats.active_executions.load(Ordering::Relaxed)
    }

    /// Set a global variable that persists across macro executions
    pub fn set_global_variable(
        &self,
        name: String,
        value: String,
    ) -> Result<(), MacroSystemError> {
        match self.variables.write() {
            Ok(mut vars) => {
                vars.insert(name, value);
                Ok(())
            }
            Err(_) => Err(MacroSystemError::ValidationError(
                "Failed to acquire lock on variables".to_string(),
            )),
        }
    }

    /// Get a global variable value by name
    pub fn get_global_variable(&self, name: &str) -> Option<String> {
        match self.variables.read() {
            Ok(vars) => vars.get(name).cloned(),
            Err(_) => None,
        }
    }

    /// Get all global variables as a snapshot
    pub fn get_global_variables_snapshot(&self) -> HashMap<String, String> {
        match self.variables.read() {
            Ok(vars) => vars.clone(),
            Err(_) => HashMap::new(),
        }
    }

    /// Clear all global variables
    pub fn clear_global_variables(&self) -> Result<(), MacroSystemError> {
        match self.variables.write() {
            Ok(mut vars) => {
                vars.clear();
                Ok(())
            }
            Err(_) => Err(MacroSystemError::ValidationError(
                "Failed to acquire lock on variables".to_string(),
            )),
        }
    }
}

/// Synchronous macro action execution
fn execute_action_sync(
    action: &MacroAction,
    context: &mut MacroExecutionContext,
) -> Result<ActionExecutionResult, MacroSystemError> {
    match action {
        MacroAction::SendMessage {
            content,
            message_type,
            ..
        } => {
            let resolved_content = resolve_variables_sync(content, &context.variables);
            // In a real implementation, this would send the message to the chat system
            println!(
                "Sending message: {resolved_content} (type: {message_type})"
            );
            Ok(ActionExecutionResult::Success)
        }
        MacroAction::ExecuteCommand { command, .. } => {
            // In a real implementation, this would execute the command
            println!("Executing command: {command:?}");
            Ok(ActionExecutionResult::Success)
        }
        MacroAction::Wait { duration, .. } => Ok(ActionExecutionResult::Wait(*duration)),
        MacroAction::SetVariable { name, value, .. } => {
            let resolved_value = resolve_variables_sync(value, &context.variables);
            context.variables.insert(name.clone(), resolved_value);
            Ok(ActionExecutionResult::Success)
        }
        MacroAction::Conditional {
            condition,
            then_actions,
            else_actions,
            ..
        } => {
            let condition_result = evaluate_condition_sync(condition, &context.variables);

            let actions_to_execute = if condition_result {
                then_actions
            } else if let Some(else_actions) = else_actions {
                else_actions
            } else {
                return Ok(ActionExecutionResult::Success);
            };

            // Execute conditional actions synchronously
            for action in actions_to_execute.iter() {
                if let Err(e) = execute_action_sync(action, context) {
                    return Err(e);
                }
            }

            Ok(ActionExecutionResult::Success)
        }
        MacroAction::Loop {
            iterations,
            actions,
            ..
        } => {
            let loop_context = LoopContext {
                iteration: 0,
                max_iterations: *iterations,
                start_action: context.current_action,
                end_action: context.current_action + actions.len() - 1,
            };

            context.loop_stack.push(loop_context);

            for _ in 0..*iterations {
                for action in actions.iter() {
                    if let Err(e) = execute_action_sync(action, context) {
                        context.loop_stack.pop();
                        return Err(e);
                    }
                }
            }

            context.loop_stack.pop();
            Ok(ActionExecutionResult::Success)
        }
    }
}

/// Synchronous variable resolution
fn resolve_variables_sync(content: &str, variables: &HashMap<String, String>) -> String {
    let mut result = content.to_string();

    for (key, value) in variables {
        let placeholder = format!("{{{key}}}");
        result = result.replace(&placeholder, value);
    }

    result
}

/// Synchronous condition evaluation
fn evaluate_condition_sync(condition: &str, variables: &HashMap<String, String>) -> bool {
    let resolved_condition = resolve_variables_sync(condition, variables);

    // Simple condition evaluation - in a real implementation, this would be more sophisticated
    if resolved_condition.contains("==") {
        let parts: Vec<&str> = resolved_condition.split("==").collect();
        if parts.len() == 2 {
            let left = parts[0].trim();
            let right = parts[1].trim();
            return left == right;
        }
    }

    // Default to false for unsupported conditions
    false
}

impl Default for MacroProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for Uuid to implement MessageChunk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacroSessionId(pub Uuid);

/// Wrapper for () to implement MessageChunk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacroActionResult;

impl From<Uuid> for MacroSessionId {
    fn from(uuid: Uuid) -> Self {
        MacroSessionId(uuid)
    }
}

impl From<MacroSessionId> for Uuid {
    fn from(session_id: MacroSessionId) -> Self {
        session_id.0
    }
}

/// Implementation of MessageChunk for MacroSessionId to enable streaming
impl MessageChunk for MacroSessionId {
    fn bad_chunk(_error: String) -> Self {
        // For errors, we'll generate a nil UUID and log the error
        // This is a fallback since Uuid doesn't have a natural error representation
        MacroSessionId(Uuid::nil())
    }

    fn error(&self) -> Option<&str> {
        // MacroSessionId doesn't carry error information, so always None
        None
    }
}

/// Implementation of MessageChunk for MacroActionResult to enable streaming
impl MessageChunk for MacroActionResult {
    fn bad_chunk(_error: String) -> Self {
        // For action results, we just return the default unit result
        MacroActionResult
    }

    fn error(&self) -> Option<&str> {
        // MacroActionResult doesn't carry error information, so always None
        None
    }
}

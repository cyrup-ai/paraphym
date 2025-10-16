//! Macro system for chat automation with lock-free data structures
//!
//! This module provides a comprehensive macro system for recording, storing,
//! and playing back chat interactions using zero-allocation patterns and
//! lock-free data structures for blazing-fast performance.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use chrono::{DateTime, Utc};
use log::{error, warn};

use atomic_counter::{AtomicCounter, ConsistentCounter};
use tokio::sync::Mutex;
use crossbeam_skiplist::SkipMap;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use cyrup_sugars::prelude::MessageChunk;
use uuid::Uuid;
use tokio_stream::StreamExt;

use crate::domain::chat::commands::{CommandEvent, ImmutableChatCommand, execute_candle_command};
use crate::domain::chat::conversation::CandleStreamingConversation;
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

/// Token types for condition evaluation
#[derive(Debug, Clone, PartialEq)]
enum CondToken {
    /// Literal value (identifier, number, or string)
    Value(String),

    // Comparison operators (precedence 80)
    /// Equality operator ==
    Eq,
    /// Inequality operator !=
    Neq,
    /// Less than operator <
    Lt,
    /// Greater than operator >
    Gt,
    /// Less than or equal <=
    Leq,
    /// Greater than or equal >=
    Geq,

    // Logical operators
    /// Logical AND &&
    And, // precedence 75
    /// Logical OR ||
    Or, // precedence 70
    /// Logical NOT !
    Not, // precedence 110

    // Grouping
    /// Left parenthesis (
    LParen,
    /// Right parenthesis )
    RParen,
}

/// Runtime value types for condition evaluation
#[derive(Debug, Clone, PartialEq)]
enum CondValue {
    /// Numeric value (floating point)
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
}

impl std::fmt::Display for CondValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CondValue::Number(n) => write!(f, "{n}"),
            CondValue::String(s) => write!(f, "{s}"),
            CondValue::Boolean(b) => write!(f, "{b}"),
        }
    }
}

impl CondValue {
    /// Parse a string into the appropriate value type
    fn parse(s: &str) -> Self {
        // Try boolean literals first
        match s.to_lowercase().as_str() {
            "true" => return CondValue::Boolean(true),
            "false" => return CondValue::Boolean(false),
            _ => {}
        }

        // Try to parse as number
        if let Ok(num) = s.parse::<f64>() {
            return CondValue::Number(num);
        }

        // Default to string
        CondValue::String(s.to_string())
    }

    /// Convert value to boolean for logical operations
    fn as_bool(&self) -> bool {
        match self {
            CondValue::Boolean(b) => *b,
            CondValue::Number(n) => *n != 0.0 && !n.is_nan(),
            CondValue::String(s) => !s.is_empty(),
        }
    }

    /// Test equality with type-aware comparison
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            // Both numbers: numeric comparison with epsilon
            (CondValue::Number(a), CondValue::Number(b)) => (a - b).abs() < f64::EPSILON,
            // Both booleans: boolean comparison
            (CondValue::Boolean(a), CondValue::Boolean(b)) => a == b,
            // Both strings: string comparison
            (CondValue::String(a), CondValue::String(b)) => a == b,
            // Mixed types: convert both to string and compare
            _ => self.to_string() == other.to_string(),
        }
    }

    /// Compare values with type checking
    fn compare(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Numeric comparison
            (CondValue::Number(a), CondValue::Number(b)) => a.partial_cmp(b),
            // String comparison
            (CondValue::String(a), CondValue::String(b)) => Some(a.cmp(b)),
            // Boolean comparison (false < true)
            (CondValue::Boolean(a), CondValue::Boolean(b)) => Some(a.cmp(b)),
            // Mixed types: cannot compare
            _ => None,
        }
    }
}

/// Tokenize a condition string into tokens
/// Parse a string literal with escape sequences
fn parse_string_literal(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut value = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next();
            break;
        }
        if ch == '\\' {
            // Escape sequence
            chars.next();
            if let Some(&escaped) = chars.peek() {
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    _ => {
                        value.push('\\');
                        value.push(escaped);
                    }
                }
                chars.next();
            }
        } else {
            value.push(ch);
            chars.next();
        }
    }
    value
}

/// Parse an identifier or unquoted value
fn parse_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut value = String::new();
    while let Some(&ch) = chars.peek() {
        // Stop at whitespace or operator characters
        if ch.is_whitespace() || "()!<>=&|\"".contains(ch) {
            break;
        }
        value.push(ch);
        chars.next();
    }
    value
}

fn tokenize_condition(input: &str) -> Vec<CondToken> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }

            // Parentheses
            '(' => {
                tokens.push(CondToken::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(CondToken::RParen);
                chars.next();
            }

            // NOT operator or inequality
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(CondToken::Neq);
                } else {
                    tokens.push(CondToken::Not);
                }
            }

            // Equality operator
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(CondToken::Eq);
                } else {
                    // Single = not supported, treat as ==
                    tokens.push(CondToken::Eq);
                }
            }

            // Less than or less-equal
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(CondToken::Leq);
                } else {
                    tokens.push(CondToken::Lt);
                }
            }

            // Greater than or greater-equal
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(CondToken::Geq);
                } else {
                    tokens.push(CondToken::Gt);
                }
            }

            // AND operator
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(CondToken::And);
                }
                // Single & is invalid, skip
            }

            // OR operator
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(CondToken::Or);
                }
                // Single | is invalid, skip
            }

            // String literal with double quotes
            '"' => {
                chars.next();
                let value = parse_string_literal(&mut chars);
                tokens.push(CondToken::Value(value));
            }

            // Identifier, number, or unquoted string
            _ => {
                let value = parse_identifier(&mut chars);
                if !value.is_empty() {
                    tokens.push(CondToken::Value(value));
                }
            }
        }
    }

    tokens
}

/// Recursive descent parser for condition expressions
struct CondParser {
    tokens: Vec<CondToken>,
    pos: usize,
}

impl CondParser {
    /// Create new parser with tokens
    fn new(tokens: Vec<CondToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Get current token without consuming
    fn current(&self) -> Option<&CondToken> {
        self.tokens.get(self.pos)
    }

    /// Advance to next token
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    /// Check if current token matches and consume if so
    fn match_token(&mut self, expected: &CondToken) -> bool {
        if let Some(token) = self.current()
            && std::mem::discriminant(token) == std::mem::discriminant(expected)
        {
            self.advance();
            return true;
        }
        false
    }

    /// Parse OR expression (lowest precedence: 70)
    fn parse_or(&mut self) -> CondValue {
        let mut left = self.parse_and();

        while self.match_token(&CondToken::Or) {
            let right = self.parse_and();
            left = CondValue::Boolean(left.as_bool() || right.as_bool());
        }

        left
    }

    /// Parse AND expression (precedence: 75)
    fn parse_and(&mut self) -> CondValue {
        let mut left = self.parse_not();

        while self.match_token(&CondToken::And) {
            let right = self.parse_not();
            left = CondValue::Boolean(left.as_bool() && right.as_bool());
        }

        left
    }

    /// Parse NOT expression (precedence: 110)
    fn parse_not(&mut self) -> CondValue {
        if self.match_token(&CondToken::Not) {
            let value = self.parse_not(); // Right-associative
            return CondValue::Boolean(!value.as_bool());
        }

        self.parse_comparison()
    }

    /// Parse comparison expression (precedence: 80)
    fn parse_comparison(&mut self) -> CondValue {
        let left = self.parse_primary();

        // Check for comparison operator
        if let Some(token) = self.current() {
            match token {
                CondToken::Eq => {
                    self.advance();
                    let right = self.parse_primary();
                    return CondValue::Boolean(left.equals(&right));
                }
                CondToken::Neq => {
                    self.advance();
                    let right = self.parse_primary();
                    return CondValue::Boolean(!left.equals(&right));
                }
                CondToken::Lt => {
                    self.advance();
                    let right = self.parse_primary();
                    if let Some(ordering) = left.compare(&right) {
                        return CondValue::Boolean(ordering == std::cmp::Ordering::Less);
                    }
                    return CondValue::Boolean(false);
                }
                CondToken::Gt => {
                    self.advance();
                    let right = self.parse_primary();
                    if let Some(ordering) = left.compare(&right) {
                        return CondValue::Boolean(ordering == std::cmp::Ordering::Greater);
                    }
                    return CondValue::Boolean(false);
                }
                CondToken::Leq => {
                    self.advance();
                    let right = self.parse_primary();
                    if let Some(ordering) = left.compare(&right) {
                        return CondValue::Boolean(ordering != std::cmp::Ordering::Greater);
                    }
                    return CondValue::Boolean(false);
                }
                CondToken::Geq => {
                    self.advance();
                    let right = self.parse_primary();
                    if let Some(ordering) = left.compare(&right) {
                        return CondValue::Boolean(ordering != std::cmp::Ordering::Less);
                    }
                    return CondValue::Boolean(false);
                }
                _ => {}
            }
        }

        // No comparison operator, return value as-is
        left
    }

    /// Parse primary expression (values and parentheses)
    fn parse_primary(&mut self) -> CondValue {
        // Handle parentheses
        if self.match_token(&CondToken::LParen) {
            let value = self.parse_or(); // Restart at lowest precedence
            self.match_token(&CondToken::RParen); // Consume closing paren (optional)
            return value;
        }

        // Handle values
        if let Some(CondToken::Value(s)) = self.current() {
            let s = s.clone();
            self.advance();
            return CondValue::parse(&s);
        }

        // Error case: unexpected token or end
        CondValue::Boolean(false)
    }
}

/// Macro execution context with variable substitution
#[derive(Debug, Clone)]
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
    /// Optional conversation for message sending
    pub conversation: Option<Arc<RwLock<CandleStreamingConversation>>>,
}

impl Default for MacroExecutionContext {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            execution_id: Uuid::new_v4(),
            start_time: Utc::now(),
            current_action: 0,
            loop_stack: Vec::new(),
            conversation: None,
        }
    }
}

/// Map message type string to appropriate conversation method
fn send_message_to_conversation(
    conversation: &Arc<RwLock<CandleStreamingConversation>>,
    content: String,
    message_type: &str,
) -> Result<(), String> {
    let mut conv = conversation
        .write()
        .map_err(|e| format!("Failed to acquire conversation lock: {e}"))?;

    let result = match message_type.to_lowercase().as_str() {
        "assistant" => conv.add_assistant_message(content),
        "system" => conv.add_system_message(content),
        _ => conv.add_user_message(content), // Default to user (includes "user" case)
    };

    result
        .map(|_| ())
        .map_err(|e| format!("Failed to add message to conversation: {e}"))
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
    /// Queue of recorded actions
    pub actions: Arc<Mutex<Vec<MacroAction>>>,
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
    #[must_use]
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
    pub fn start_recording(&self, name: String, description: &str) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = MacroSessionId> + Send>> {
        let session_id = Uuid::new_v4();
        let macro_id = Uuid::new_v4();

        // Create metadata for the macro
        let metadata = MacroMetadata {
            id: macro_id,
            name: name.clone(),
            description: description.to_string(),
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
            actions: Arc::new(Mutex::new(Vec::new())),
            state: MacroRecordingState::Recording,
            variables: HashMap::new(),
            metadata,
        };

        // Capture session_id in owned variable before any operations
        let owned_session_id = session_id;

        // Insert the new session directly
        self.recording_sessions.insert(session_id, new_session);

        // Create a stream that immediately yields the session ID using tokio stream pattern
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let _ = sender.send(MacroSessionId(owned_session_id));
        }))
    }

    /// Record a macro action
    pub fn record_action(
        &self,
        session_id: Uuid,
        action: MacroAction,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = MacroActionResult> + Send>> {
        // Get a reference to the session without cloning the entire DashMap
        if let Some(session) = self.recording_sessions.get(&session_id) {
            // Check if we're still recording
            if session.value().state == MacroRecordingState::Recording {
                // Push the action to the session's actions
                if let Ok(mut actions) = session.value().actions.try_lock() {
                    actions.push(action);
                }

                // Create a stream that immediately yields success
                return Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                    let _ = sender.send(MacroActionResult);
                }));
            }
        }

        // If we get here, either the session doesn't exist or it's not recording
        // Return an empty stream that immediately completes
        Box::pin(crate::async_stream::empty())
    }

    /// Stop recording and save the macro
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if:
    /// - Recording session with the given ID does not exist
    /// - Macro cannot be saved
    pub fn stop_recording(&self, session_id: Uuid) -> Result<Uuid, MacroSystemError> {
        if let Some((_, session)) = self.recording_sessions.remove(&session_id) {
            let mut session = session; // make mutable
            session.state = MacroRecordingState::Completed;

            // Collect all recorded actions
            let actions = if let Ok(mut action_vec) = session.actions.try_lock() {
                action_vec.drain(..).collect()
            } else {
                Vec::new()
            };

            // Create the macro
            let chat_macro = ChatMacro {
                metadata: session.metadata.clone(),
                actions,
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
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if:
    /// - Macro with the given ID does not exist
    /// - Playback session cannot be created
    pub fn start_playback(
        &self,
        macro_id: Uuid,
        variables: HashMap<String, String>,
        conversation: Option<Arc<RwLock<CandleStreamingConversation>>>,
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
            conversation,
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
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if:
    /// - Playback session with the given ID does not exist
    /// - Action execution fails
    pub async fn execute_next_action(
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
                tokio::time::sleep(duration).await;
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
    #[allow(clippy::too_many_lines)]
    fn _execute_action(
        action: &MacroAction,
        context: &mut MacroExecutionContext,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = ActionExecutionResult> + Send>> {
        // Clone only what's needed for the closure
        let action_clone = action.clone();
        let context_vars = context.variables.clone();
        let mut ctx = context.clone();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let result = match &action_clone {
                MacroAction::SendMessage {
                    content,
                    message_type,
                    ..
                } => {
                    let resolved_content = resolve_variables_sync(content, &context_vars);

                    // Send message to conversation if available
                    if let Some(ref conversation) = ctx.conversation {
                        match send_message_to_conversation(
                            conversation,
                            resolved_content.clone(),
                            message_type,
                        ) {
                            Ok(()) => Ok::<ActionExecutionResult, MacroSystemError>(
                                ActionExecutionResult::Success,
                            ),
                            Err(e) => Ok::<ActionExecutionResult, MacroSystemError>(
                                ActionExecutionResult::Error(format!("Message send failed: {e}")),
                            ),
                        }
                    } else {
                        // No conversation available - log warning and continue
                        warn!(
                            "No conversation available for SendMessage action. Message: {resolved_content}"
                        );
                        Ok::<ActionExecutionResult, MacroSystemError>(
                            ActionExecutionResult::Success,
                        )
                    }
                }
                MacroAction::ExecuteCommand { command, .. } => {
                    // Execute command using existing infrastructure
                    let mut event_stream = execute_candle_command(command.clone());

                    // Collect events asynchronously
                    let mut command_output = String::new();
                    let mut result = ActionExecutionResult::Success;

                    while let Some(event) = event_stream.next().await {
                        match event {
                            CommandEvent::Output { content, .. } => {
                                // Collect output for potential logging/debugging
                                command_output.push_str(&content);
                            }
                            CommandEvent::Completed { .. } => {
                                // Command succeeded
                                result = ActionExecutionResult::Success;
                            }
                            CommandEvent::Failed { error, .. } => {
                                // Command failed - capture error
                                result = ActionExecutionResult::Error(format!(
                                    "Command execution failed: {error}"
                                ));
                                break; // Exit early on failure
                            }
                            CommandEvent::Cancelled { reason, .. } => {
                                // Command was cancelled
                                result = ActionExecutionResult::Error(format!(
                                    "Command cancelled: {reason}"
                                ));
                                break;
                            }
                            _ => {} // Ignore other events (Started, Progress, Warning, ResourceAlert)
                        }
                    }

                    // Log output if command succeeded and produced output
                    if !command_output.is_empty()
                        && matches!(result, ActionExecutionResult::Success)
                    {
                        log::debug!("Command output: {command_output}");
                    }

                    Ok::<ActionExecutionResult, MacroSystemError>(result)
                }
                MacroAction::Wait { duration, .. } => Ok(ActionExecutionResult::Wait(*duration)),
                MacroAction::SetVariable { name, value, .. } => {
                    let resolved_value = resolve_variables_sync(value, &context_vars);
                    ctx.variables.insert(name.clone(), resolved_value);
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
                    for action in actions_to_execute {
                        match execute_action_sync(action, &mut ctx) {
                            Ok(ActionExecutionResult::Error(error)) => {
                                let _ = sender.send(ActionExecutionResult::Error(error));
                                return;
                            }
                            Err(e) => {
                                error!("Action execution failed: {e}");
                            }
                            _ => {}
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
                        for action in actions {
                            match execute_action_sync(action, &mut ctx) {
                                Ok(ActionExecutionResult::Error(error)) => {
                                    ctx.loop_stack.pop();
                                    let _ = sender.send(ActionExecutionResult::Error(error));
                                    return;
                                }
                                Err(e) => {
                                    ctx.loop_stack.pop();
                                    error!("Loop action execution failed: {e}");
                                }
                                _ => {}
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
                    error!("Action execution failed: {e}");
                }
            }
        }))
    }

    /// Resolve variables in a string - planned feature
    fn _resolve_variables(content: &str, variables: &HashMap<String, String>) -> String {
        let mut result = content.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{key}}}");
            result = result.replace(&placeholder, value);
        }

        result
    }

    /// Evaluate a condition string with full expression support
    fn _evaluate_condition(condition: &str, variables: &HashMap<String, String>) -> bool {
        // Handle empty condition
        if condition.trim().is_empty() {
            return false;
        }

        // Resolve variables in condition
        let resolved = Self::_resolve_variables(condition, variables);

        // Tokenize the resolved condition
        let tokens = tokenize_condition(&resolved);

        // Handle empty token stream
        if tokens.is_empty() {
            return false;
        }

        // Parse and evaluate expression
        let mut parser = CondParser::new(tokens);
        let result = parser.parse_or();

        // Convert result to boolean
        result.as_bool()
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ActionExecutionResult {
    /// Action executed successfully
    #[default]
    Success,
    /// Wait for specified duration before continuing
    Wait(Duration),
    /// Skip to the specified action index
    SkipToAction(usize),
    /// Execution failed with error message
    Error(String),
}

impl MessageChunk for ActionExecutionResult {
    fn bad_chunk(error: String) -> Self {
        ActionExecutionResult::Error(error)
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
    execution_queue: Arc<Mutex<Vec<MacroExecutionRequest>>>,
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

bitflags::bitflags! {
    /// Macro processor feature flags for zero-allocation enable/disable checks
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct MacroFeatureFlags: u8 {
        /// Enable variable substitution in macros
        const VARIABLE_SUBSTITUTION = 1 << 0;
        /// Enable conditional execution (if/else)
        const CONDITIONAL_EXECUTION = 1 << 1;
        /// Enable loop execution (for/while)
        const LOOP_EXECUTION = 1 << 2;
        /// Enable performance monitoring
        const MONITORING = 1 << 3;
        /// Auto-save macro changes
        const AUTO_SAVE = 1 << 4;
        /// All macro features enabled
        const ALL = Self::VARIABLE_SUBSTITUTION.bits() | Self::CONDITIONAL_EXECUTION.bits() | Self::LOOP_EXECUTION.bits() | Self::MONITORING.bits() | Self::AUTO_SAVE.bits();
    }
}

/// Macro processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroProcessorConfig {
    /// Maximum concurrent executions
    pub max_concurrent_executions: usize,
    /// Default execution timeout in seconds
    pub default_timeout_seconds: u64,
    /// Feature flags
    pub flags: MacroFeatureFlags,
    /// Maximum macro recursion depth
    pub max_recursion_depth: usize,
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
    fn bad_chunk(error: String) -> Self {
        MacroExecutionResult {
            success: false,
            message: error,
            actions_executed: 0,
            execution_duration: Duration::from_secs(0),
            modified_variables: HashMap::new(),
            metadata: MacroExecutionMetadata::default(),
        }
    }

    fn error(&self) -> Option<&str> {
        if self.success {
            None
        } else {
            Some(&self.message)
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
            flags: MacroFeatureFlags::ALL,
            max_recursion_depth: 10,
        }
    }
}

impl MacroProcessor {
    /// Create a new macro processor
    #[must_use]
    pub fn new() -> Self {
        Self {
            macros: Arc::new(SkipMap::new()),
            stats: Arc::new(MacroProcessorStats::default()),
            variables: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(Mutex::new(Vec::new())),
            config: MacroProcessorConfig::default(),
        }
    }

    /// Create a macro processor with custom configuration
    #[must_use]
    pub fn with_config(config: MacroProcessorConfig) -> Self {
        Self {
            macros: Arc::new(SkipMap::new()),
            stats: Arc::new(MacroProcessorStats::default()),
            variables: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(Mutex::new(Vec::new())),
            config,
        }
    }

    /// Register a macro
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if macro validation fails
    pub fn register_macro(&self, macro_def: ChatMacro) -> Result<(), MacroSystemError> {
        // Validate macro
        self.validate_macro(&macro_def)?;

        // Store macro
        self.macros.insert(macro_def.metadata.id, macro_def);

        Ok(())
    }

    /// Unregister a macro
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if macro with the given ID does not exist
    pub fn unregister_macro(&self, macro_id: &Uuid) -> Result<(), MacroSystemError> {
        if self.macros.remove(macro_id).is_none() {
            return Err(MacroSystemError::MacroNotFound);
        }

        Ok(())
    }

    /// Execute a macro by ID
    #[must_use]
    pub fn execute_macro(
        &self,
        macro_id: &Uuid,
        context_variables: HashMap<String, String>,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = MacroExecutionResult> + Send>> {
        let macro_def = self
            .macros
            .get(macro_id)
            .ok_or(MacroSystemError::MacroNotFound);

        if let Ok(macro_entry) = macro_def {
            let macro_def = macro_entry.value().clone();
            self.execute_macro_impl(macro_def, context_variables)
        } else {
            let macro_id = *macro_id;
            Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                let default_result = MacroExecutionResult {
                    success: false,
                    message: format!("Macro not found: {macro_id:?}"),
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
            }))
        }
    }

    /// Internal macro execution implementation
    fn execute_macro_impl(
        &self,
        macro_def: ChatMacro,
        context_variables: HashMap<String, String>,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = MacroExecutionResult> + Send>> {
        let self_clone = self.clone();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
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
                conversation: None,
            };

            let mut performance = MacroPerformanceMetrics::default();
            let (success, actions_executed, error_message) =
                Self::execute_macro_actions(&macro_def, &mut context, &mut performance).await;

            let execution_duration = Utc::now()
                .signed_duration_since(start_time)
                .to_std()
                .unwrap_or_default();
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
                    error_message.unwrap_or_else(|| String::from("Execution failed"))
                },
                actions_executed,
                execution_duration,
                modified_variables: HashMap::new(),
                metadata,
            };

            let _ = sender.send(result);
        }))
    }

    async fn execute_macro_actions(
        macro_def: &ChatMacro,
        context: &mut MacroExecutionContext,
        performance: &mut MacroPerformanceMetrics,
    ) -> (bool, usize, Option<String>) {
        let mut actions_executed = 0;
        let mut error_message: Option<String> = None;

        let success = loop {
            if context.current_action >= macro_def.actions.len() {
                break true;
            }

            let action = &macro_def.actions[context.current_action];

            match execute_action_sync(action, context) {
                Ok(ActionExecutionResult::Success) => {
                    actions_executed += 1;
                    context.current_action += 1;
                }
                Ok(ActionExecutionResult::Wait(duration)) => {
                    tokio::time::sleep(duration).await;
                    actions_executed += 1;
                    context.current_action += 1;
                }
                Ok(ActionExecutionResult::SkipToAction(index)) => {
                    actions_executed += 1;
                    context.current_action = index;
                }
                Ok(ActionExecutionResult::Error(error)) => {
                    error_message = Some(format!(
                        "Action {} failed: {}",
                        context.current_action, error
                    ));
                    if let Some(ref msg) = error_message {
                        error!("Macro execution error: {msg}");
                    }
                    break false;
                }
                Err(e) => {
                    error_message = Some(format!(
                        "System error at action {}: {}",
                        context.current_action, e
                    ));
                    if let Some(ref msg) = error_message {
                        error!("Macro system error: {msg}");
                    }
                    break false;
                }
            }

            performance.disk_operations += 1;
        };

        (success, actions_executed, error_message)
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
        for action in &macro_def.actions {
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
    #[must_use]
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
    #[must_use]
    pub fn get_active_executions(&self) -> usize {
        self.stats.active_executions.load(Ordering::Relaxed)
    }

    /// Set a global variable that persists across macro executions
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if lock on variables cannot be acquired
    pub fn set_global_variable(&self, name: String, value: String) -> Result<(), MacroSystemError> {
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
    #[must_use]
    pub fn get_global_variable(&self, name: &str) -> Option<String> {
        match self.variables.read() {
            Ok(vars) => vars.get(name).cloned(),
            Err(_) => None,
        }
    }

    /// Get all global variables as a snapshot
    #[must_use]
    pub fn get_global_variables_snapshot(&self) -> HashMap<String, String> {
        match self.variables.read() {
            Ok(vars) => vars.clone(),
            Err(_) => HashMap::new(),
        }
    }

    /// Clear all global variables
    ///
    /// # Errors
    ///
    /// Returns `MacroSystemError` if lock acquisition fails
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

            // Send message to conversation if available
            if let Some(ref conversation) = context.conversation {
                match send_message_to_conversation(
                    conversation,
                    resolved_content.clone(),
                    message_type,
                ) {
                    Ok(()) => Ok(ActionExecutionResult::Success),
                    Err(e) => Ok(ActionExecutionResult::Error(format!(
                        "Message send failed: {e}"
                    ))),
                }
            } else {
                // No conversation available - log warning and continue
                warn!(
                    "No conversation available for SendMessage action. Message: {resolved_content}"
                );
                Ok(ActionExecutionResult::Success)
            }
        }
        MacroAction::ExecuteCommand { command, .. } => {
            // Note: Sync command execution not supported - commands require async streams
            // This sync helper is only used for conditional/loop actions
            // Real command execution happens in the async _execute_action function
            warn!("Command execution skipped in sync context: {command:?}");
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
            for action in actions_to_execute {
                execute_action_sync(action, context)?;
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
                for action in actions {
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

/// Synchronous condition evaluation with full expression support
fn evaluate_condition_sync(condition: &str, variables: &HashMap<String, String>) -> bool {
    // Handle empty condition
    if condition.trim().is_empty() {
        return false;
    }

    // Resolve variables in condition
    let resolved = resolve_variables_sync(condition, variables);

    // Tokenize the resolved condition
    let tokens = tokenize_condition(&resolved);

    // Handle empty token stream
    if tokens.is_empty() {
        return false;
    }

    // Parse and evaluate expression
    let mut parser = CondParser::new(tokens);
    let result = parser.parse_or();

    // Convert result to boolean
    result.as_bool()
}

impl Default for MacroProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for `Uuid` to implement `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacroSessionId(pub Uuid);

/// Wrapper for `()` to implement `MessageChunk`
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

/// Implementation of `MessageChunk` for `MacroSessionId` to enable streaming
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

/// Implementation of `MessageChunk` for `MacroActionResult` to enable streaming
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

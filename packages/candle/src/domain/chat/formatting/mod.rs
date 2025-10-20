//! Immutable message formatting with streaming operations
//!
//! Provides zero-allocation, lock-free message formatting with streaming operations.
//! All Arc usage eliminated in favor of owned strings and borrowed data patterns
//! for blazing-fast performance with immutable formatting structures.

// Declare submodules
pub mod content;
pub mod error;
pub mod events;
pub mod formatter;
pub mod options;
pub mod compat;

// Re-export all public types to maintain API compatibility
pub use content::{
    ImmutableMessageContent,
    FormatStyle,
    StyleType,
};

pub use error::{
    FormatError,
    FormatResult,
};

pub use events::{
    FormattingEvent,
};

pub use formatter::{
    StreamingMessageFormatter,
    FormatterStats,
    FormatterWithStream,
};

pub use options::{
    FormatFlags,
    ImmutableFormatOptions,
    SyntaxTheme,
    ImmutableColorScheme,
    OutputFormat,
    ImmutableCustomFormatRule,
};

// Re-export deprecated aliases for backward compatibility
pub use compat::{
    MessageContent,
    FormatOptions,
    ColorScheme,
    CustomFormatRule,
    MessageFormatter,
};

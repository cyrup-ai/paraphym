//! High-level memory management functionality
//!
//! This module was decomposed from a 1,331-line monolithic file
//! into 9 focused modules for better maintainability.

mod types;
mod lifecycle;
mod temporal;
mod workers;
mod operations;
mod search;
mod relationships;
mod conversions;
mod trait_impl;

// Re-export public types
pub use types::*;

// Re-export the main coordinator struct
pub use lifecycle::MemoryCoordinator;

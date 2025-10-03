//! Cryptographic utilities module for secure token handling
//!
//! This module provides comprehensive cryptographic utilities including NaCl box
//! encryption for discovery tokens, token rotation, revocation list support,
//! and secure token wrappers with zero allocation patterns and blazing-fast performance.

pub mod core;
pub mod operations;

// Re-export core types for ergonomic use

// Re-export operations types
// Re-export wrapper types

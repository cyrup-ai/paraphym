//! Shared Tokio runtime for the Candle package
//!
//! This module provides a single shared runtime to avoid creating
//! multiple runtimes for every database operation.
//!
//! The runtime is created in a separate thread to avoid nested runtime
//! issues when called from within #[tokio::main] or other async contexts.

use std::sync::OnceLock;
use tokio::runtime::Runtime;

/// Global shared Tokio runtime (None if initialization failed)
///
/// Created in a separate thread to ensure it's never initialized from
/// within an existing async runtime context, preventing nested runtime errors.
static SHARED_RUNTIME: OnceLock<Option<Runtime>> = OnceLock::new();

/// Get reference to the shared runtime
///
/// Returns None if runtime initialization failed.
/// The runtime is lazily initialized on first access in a separate thread
/// to avoid nested runtime issues.
pub fn shared_runtime() -> Option<&'static Runtime> {
    SHARED_RUNTIME
        .get_or_init(|| {
            // Create runtime in separate thread to avoid nested runtime issues
            std::thread::spawn(|| Runtime::new().ok())
                .join()
                .ok()
                .flatten()
        })
        .as_ref()
}
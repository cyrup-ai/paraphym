//! Shared Tokio runtime for the Candle package
//!
//! This module provides a single shared runtime to avoid creating
//! multiple runtimes for every database operation.

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

/// Global shared Tokio runtime (None if initialization failed)
static SHARED_RUNTIME: Lazy<Option<Runtime>> = Lazy::new(|| {
    Runtime::new().ok()
});

/// Get reference to the shared runtime
///
/// Returns None if runtime initialization failed
pub fn shared_runtime() -> Option<&'static Runtime> {
    SHARED_RUNTIME.as_ref()
}
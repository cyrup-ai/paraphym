//! Shared Tokio runtime for the Candle package
//! 
//! This module provides a single shared runtime to avoid creating
//! multiple runtimes for every database operation.

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

/// Global shared Tokio runtime
static SHARED_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    // APPROVED BY DAVID MAPLE 09/30/2025: Panic is appropriate for runtime initialization failure
    Runtime::new().expect("Failed to create shared Tokio runtime")
});

/// Get reference to the shared runtime
pub fn shared_runtime() -> &'static Runtime {
    &SHARED_RUNTIME
}
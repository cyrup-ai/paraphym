//! Core context module
//!
//! This module provides the core context functionality including types,
//! subscriptions, and global context management with zero allocation
//! patterns and blazing-fast performance.

pub mod globals;
pub mod subscriptions;
pub mod types;

// Re-export core types and functions for ergonomic usage
pub use globals::{
    APPLICATION_CONTEXT, GlobalContextManager, GlobalContextStats, SAMPLING_CONTEXT,
    context_access, initialize_global_context,
};
pub use subscriptions::{
    CONTEXT_SUBSCRIPTIONS, ContextSubscription, ContextSubscriptionManager, SubscriptionStats,
};
pub use types::{ApplicationContext, ContextStats, SamplingContext, SamplingStats};

/// Create a new context subscription
pub fn subscription(id: String, scopes: Vec<String>) -> ContextSubscription {
    ContextSubscription::new(id, scopes)
}

/// Get global application context (convenience function)
///
/// **Note:** Now async and returns owned value
pub async fn app_context() -> ApplicationContext {
    context_access::app_context().await
}

/// Get global sampling context (convenience function)
///
/// **Note:** Now async and returns owned value
pub async fn sampling_context() -> SamplingContext {
    context_access::sampling_context().await
}

/// Try to get global application context (convenience function)
///
/// **Note:** Now async and returns owned value
pub async fn try_app_context() -> Option<ApplicationContext> {
    context_access::try_app_context().await
}

/// Try to get global sampling context (convenience function)
///
/// **Note:** Now async and returns owned value
pub async fn try_sampling_context() -> Option<SamplingContext> {
    context_access::try_sampling_context().await
}

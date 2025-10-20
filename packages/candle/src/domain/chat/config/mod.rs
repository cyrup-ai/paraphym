//! Configuration management system for chat features
//!
//! This module provides a comprehensive configuration management system with atomic updates,
//! validation, persistence, and change notifications using zero-allocation patterns and
//! lock-free operations for blazing-fast performance.

mod model;
mod types;
mod validation;
mod manager;
mod persistence;
mod streaming;

// Re-export public API - maintain exact same public interface
pub use model::{
    CandleModelConfig,
    CandleModelRetryConfig,
    CandleModelPerformanceConfig,
};

pub use types::{
    CandleChatConfig,
    CandlePersonalityConfig,
    CandleBehaviorConfig,
    CandleUIConfig,
};

pub use validation::{
    CandleConfigurationValidator,
    CandleConfigurationValidationError,
    CandleConfigurationValidationResult,
    CandlePersonalityValidator,
    CandleBehaviorValidator,
    CandleUIValidator,
};

pub use manager::{
    CandleConfigurationManager,
    CandleConfigurationStatistics,
    CandleConfigurationChangeEvent,
    CandleConfigurationChangeType,
};

pub use persistence::{
    CandleConfigurationPersistence,
    CandlePersistenceEvent,
    CandlePersistenceType,
};

pub use streaming::{
    CandleModelConfigChunk,
    CandleModelConfigData,
    CandleConfigUpdate,
    CandleConfigUpdateType,
};

//! Committee Evaluation System Using Existing Providers
//!
//! Provides committee evaluation using existing CandleKimiK2Provider and CandleQwen3CoderProvider.

pub mod committee_types;
pub mod committee_evaluators;

// Re-export key types
pub use committee_types::{Committee, CommitteeConfig};
pub use committee_evaluators::ProviderCommitteeEvaluator;
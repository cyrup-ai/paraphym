//! Committee Evaluation System Using Existing Models
//!
//! Provides committee evaluation using existing CandleKimiK2Model and CandleQwen3CoderModel.

pub mod committee_types;
pub mod committee_evaluators;

// Re-export key types
pub use committee_types::{Committee, CommitteeConfig};
pub use committee_evaluators::ModelCommitteeEvaluator;
//! Temporal context and causal dependency tracking for time-aware memory operations

use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Temporal context for time-aware memory operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    /// Start time of temporal window
    pub window_start: SystemTime,
    /// End time of temporal window
    pub window_end: SystemTime,
    /// Duration of temporal window in seconds
    pub window_duration_secs: u64,
    /// Causal links between memories
    pub causal_links: Vec<CausalLink>,
    /// Last consolidation timestamp
    pub last_consolidation: SystemTime,
}

impl TemporalContext {
    /// Create new temporal context with specified window duration
    #[must_use]
    pub fn new(window_duration_secs: u64) -> Self {
        let window_start = SystemTime::now();
        let window_end = window_start
            + std::time::Duration::from_secs(window_duration_secs);
        Self {
            window_start,
            window_end,
            window_duration_secs,
            causal_links: Vec::new(),
            last_consolidation: SystemTime::now(),
        }
    }

    /// Add causal link between memories
    pub fn add_causal_link(&mut self, link: CausalLink) {
        self.causal_links.push(link);
    }

    /// Check if a timestamp is within the temporal window
    #[must_use]
    pub fn is_within_window(&self, timestamp: SystemTime) -> bool {
        timestamp >= self.window_start && timestamp <= self.window_end
    }

    /// Shift temporal window forward
    pub fn shift_window_forward(&mut self) {
        let duration = std::time::Duration::from_secs(self.window_duration_secs);
        self.window_start = self.window_end;
        self.window_end = self.window_start + duration;
    }

    /// Get causal predecessors of a memory
    #[must_use]
    pub fn get_causal_predecessors(&self, memory_id: Uuid) -> Vec<Uuid> {
        self.causal_links
            .iter()
            .filter(|link| link.consequent_id == memory_id)
            .map(|link| link.antecedent_id)
            .collect()
    }

    /// Get causal successors of a memory
    #[must_use]
    pub fn get_causal_successors(&self, memory_id: Uuid) -> Vec<Uuid> {
        self.causal_links
            .iter()
            .filter(|link| link.antecedent_id == memory_id)
            .map(|link| link.consequent_id)
            .collect()
    }
}

impl Default for TemporalContext {
    fn default() -> Self {
        Self::new(3600) // Default 1-hour window
    }
}

/// Causal link between memories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    /// Antecedent memory (cause)
    pub antecedent_id: Uuid,
    /// Consequent memory (effect)
    pub consequent_id: Uuid,
    /// Causal strength (0.0 to 1.0)
    pub strength: f32,
    /// Confidence in causal relationship (0.0 to 1.0)
    pub confidence: f32,
    /// Timestamp when link was established
    pub created_at: SystemTime,
}

impl CausalLink {
    /// Create new causal link
    #[must_use]
    pub fn new(antecedent_id: Uuid, consequent_id: Uuid, strength: f32, confidence: f32) -> Self {
        Self {
            antecedent_id,
            consequent_id,
            strength: strength.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
            created_at: SystemTime::now(),
        }
    }
}

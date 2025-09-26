//! Token management and special token handling
//!
//! This module provides structures and utilities for managing tokens,
//! special tokens (EOS, BOS, PAD), token probabilities, and token history
//! for repetition penalty calculations.

use std::cmp::Ordering;
use std::collections::VecDeque;

/// Special tokens used during text generation
///
/// Manages end-of-sequence, beginning-of-sequence, and padding tokens
/// that control generation behavior and termination conditions.
#[derive(Debug, Clone)]
pub struct SpecialTokens {
    /// End of sequence token ID
    pub eos_token_id: Option<u32>,
    /// Beginning of sequence token ID  
    pub bos_token_id: Option<u32>,
    /// Padding token ID
    pub pad_token_id: Option<u32>,
}

impl SpecialTokens {
    /// Create new SpecialTokens with all tokens set to None
    pub fn new() -> Self {
        Self {
            eos_token_id: None,
            bos_token_id: None,
            pad_token_id: None,
        }
    }

    /// Create SpecialTokens with EOS token specified
    pub fn with_eos(eos_token_id: u32) -> Self {
        Self {
            eos_token_id: Some(eos_token_id),
            bos_token_id: None,
            pad_token_id: None,
        }
    }

    /// Check if a token is the EOS token
    pub fn is_eos(&self, token_id: u32) -> bool {
        self.eos_token_id
            .map(|eos| eos == token_id)
            .unwrap_or(false)
    }

    /// Check if a token is the BOS token
    pub fn is_bos(&self, token_id: u32) -> bool {
        self.bos_token_id
            .map(|bos| bos == token_id)
            .unwrap_or(false)
    }

    /// Check if a token is the PAD token
    pub fn is_pad(&self, token_id: u32) -> bool {
        self.pad_token_id
            .map(|pad| pad == token_id)
            .unwrap_or(false)
    }
}
impl Default for SpecialTokens {
    fn default() -> Self {
        Self::new()
    }
}

/// Token with associated probability for sampling operations
///
/// Used in top-k and nucleus sampling to track token candidates
/// with their probabilities. Implements ordering for sorting.
#[derive(Debug, Clone)]
pub struct TokenProb {
    /// Token identifier
    pub token_id: u32,
    /// Probability/logit value
    pub prob: f32,
}

impl TokenProb {
    /// Create new TokenProb
    pub fn new(token_id: u32, prob: f32) -> Self {
        Self { token_id, prob }
    }
}

impl PartialEq for TokenProb {
    fn eq(&self, other: &Self) -> bool {
        self.prob == other.prob
    }
}

impl Eq for TokenProb {}

impl PartialOrd for TokenProb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse order for max-heap behavior (highest prob first)
        other.prob.partial_cmp(&self.prob)
    }
}

impl Ord for TokenProb {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
/// Token history management for repetition penalty calculation
///
/// Maintains a sliding window of recent tokens to apply repetition
/// penalties effectively while managing memory usage.
#[derive(Debug, Clone)]
pub struct TokenHistory {
    /// Token storage with fixed capacity
    tokens: VecDeque<u32>,
    /// Maximum number of tokens to retain
    max_length: usize,
}

impl TokenHistory {
    /// Create new TokenHistory with specified capacity
    pub fn new(max_length: usize) -> Self {
        Self {
            tokens: VecDeque::with_capacity(max_length),
            max_length,
        }
    }

    /// Add a token to the history, removing oldest if at capacity
    pub fn push(&mut self, token_id: u32) {
        if self.tokens.len() >= self.max_length {
            self.tokens.pop_front();
        }
        self.tokens.push_back(token_id);
    }

    /// Get all tokens in history
    pub fn tokens(&self) -> &VecDeque<u32> {
        &self.tokens
    }

    /// Get tokens as a slice for repetition penalty calculation
    pub fn as_slice(&self) -> Vec<u32> {
        self.tokens.iter().copied().collect()
    }

    /// Clear all tokens from history
    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    /// Get the number of tokens in history
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_tokens() {
        let tokens = SpecialTokens::with_eos(2);
        assert!(tokens.is_eos(2));
        assert!(!tokens.is_eos(1));
        assert!(!tokens.is_bos(2));
    }

    #[test]
    fn test_token_prob_ordering() {
        let token1 = TokenProb::new(1, 0.8);
        let token2 = TokenProb::new(2, 0.6);

        // Higher probability should sort first (reverse order)
        assert!(token1 < token2);

        let mut tokens = vec![token2, token1];
        tokens.sort();
        assert_eq!(tokens[0].prob, 0.8);
    }

    #[test]
    fn test_token_history() {
        let mut history = TokenHistory::new(3);

        history.push(1);
        history.push(2);
        history.push(3);
        assert_eq!(history.len(), 3);

        history.push(4); // Should evict token 1
        assert_eq!(history.len(), 3);
        assert_eq!(history.as_slice(), vec![2, 3, 4]);
    }
}

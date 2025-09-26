//! Query processing and expansion functionality

use std::collections::HashMap;

use super::types::{ProcessedQuery, QueryMetadata, QueryOperator, SearchOptions};

/// Query processor for advanced query handling
pub struct QueryProcessor {
    /// Enable query expansion
    expansion_enabled: bool,
    /// Expansion dictionary for synonyms
    expansion_dict: HashMap<String, Vec<String>>,
}

impl QueryProcessor {
    /// Create a new query processor
    pub fn new() -> Self {
        Self {
            expansion_enabled: false,
            expansion_dict: HashMap::new(),
        }
    }

    /// Process a query string
    pub fn process_query(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<ProcessedQuery, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = std::time::Instant::now();

        let terms: Vec<String> = query
            .split_whitespace()
            .map(|term| term.to_lowercase())
            .collect();

        let expanded_terms = if options.enable_query_expansion {
            match self.expand_terms_sync(&terms, &options.expansion_dictionary) {
                Ok(terms) => terms,
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        };

        let result = ProcessedQuery {
            original: query.to_string(),
            terms,
            expanded_terms,
            operator: QueryOperator::And,
            metadata: QueryMetadata {
                processed_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                processing_time_us: start_time.elapsed().as_micros() as u64,
                expansion_applied: options.enable_query_expansion,
                normalization_applied: true,
            },
        };

        Ok(result)
    }

    /// Expand query terms using synonyms
    fn expand_terms_sync(
        &self,
        terms: &[String],
        dictionary: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut expanded = Vec::new();

        for term in terms {
            if let Some(synonyms) = dictionary.get(term) {
                expanded.extend(synonyms.clone());
            }
        }

        Ok(expanded)
    }
}

impl Default for QueryProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for QueryProcessor {
    fn clone(&self) -> Self {
        Self {
            expansion_enabled: self.expansion_enabled,
            expansion_dict: self.expansion_dict.clone(),
        }
    }
}

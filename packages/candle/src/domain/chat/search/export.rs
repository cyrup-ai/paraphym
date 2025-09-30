//! Search result export functionality

use ystream::AsyncStream;
use serde_json;

use super::types::{ExportFormat, ExportOptions, SearchError, SearchResult};
use crate::domain::context::chunk::CandleJsonChunk;

/// Search result exporter with streaming capabilities
pub struct SearchExporter {
    /// Default export options
    default_options: ExportOptions,
}

/// History exporter for chat conversation history (domain version)
#[derive(Debug)]
pub struct HistoryExporter {
    /// Default export options
    default_options: ExportOptions,
}

impl SearchExporter {
    /// Create a new search exporter
    pub fn new() -> Self {
        Self {
            default_options: ExportOptions::default(),
        }
    }

    /// Export search results as a stream
    pub fn export_stream(
        &self,
        results: Vec<SearchResult>,
        options: Option<ExportOptions>,
    ) -> AsyncStream<CandleJsonChunk> {
        let export_options = options.unwrap_or_else(|| self.default_options.clone());
        let limited_results = if let Some(max) = export_options.max_results {
            results.into_iter().take(max).collect()
        } else {
            results
        };

        // Clone self to avoid borrowing issues
        let self_clone = self.clone();

        AsyncStream::with_channel(move |sender| {
            if let ExportFormat::Json = export_options.format {
                if let Ok(json) = self_clone.export_json_sync(&limited_results, &export_options)
                    && let Ok(value) = serde_json::from_str(&json) {
                        let _ = sender.send(CandleJsonChunk(value));
                    }
            } else {
                // Other formats not implemented in simplified version
                let error_value = serde_json::json!({"error": "Export format not supported"});
                let _ = sender.send(CandleJsonChunk(error_value));
            }
        })
    }

    /// Export to JSON format
    fn export_json_sync(
        &self,
        results: &[SearchResult],
        _options: &ExportOptions,
    ) -> Result<String, SearchError> {
        serde_json::to_string_pretty(&results).map_err(|e| SearchError::ExportError {
            reason: format!("JSON serialization failed: {e}"),
        })
    }
}

impl Default for SearchExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SearchExporter {
    fn clone(&self) -> Self {
        Self {
            default_options: self.default_options.clone(),
        }
    }
}

impl HistoryExporter {
    /// Create a new history exporter
    pub fn new() -> Self {
        Self {
            default_options: ExportOptions::default(),
        }
    }

    /// Create exporter with custom default options
    pub fn with_options(options: ExportOptions) -> Self {
        Self {
            default_options: options,
        }
    }
}

impl Default for HistoryExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HistoryExporter {
    fn clone(&self) -> Self {
        Self {
            default_options: self.default_options.clone(),
        }
    }
}

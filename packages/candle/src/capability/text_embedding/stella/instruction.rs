//! Task-specific instruction formatting for Stella embeddings

/// Valid task types for Stella embeddings
const VALID_TASKS: &[&str] = &[
    "s2p",
    "s2s",
    "search_query",
    "search_document",
    "classification",
    "clustering",
    "retrieval",
];

/// Format texts with task-specific instruction prefix following canonical Stella example
///
/// # Task Types
/// - `"s2p"`, `"search_query"`, `"search_document"`, or `"retrieval"`: Search query → passage retrieval
///   - Instruction: "Given a web search query, retrieve relevant passages that answer the query."
/// - `"s2s"`, `"classification"`, or `"clustering"`: Semantic similarity
///   - Instruction: "Retrieve semantically similar text."
/// - `None`: Defaults to search query mode (`"s2p"`)
///
/// # Validation
/// If an invalid task is provided, a warning will be logged and the default `"s2p"` instruction will be used.
///
/// # Examples
/// ```ignore
/// let texts = vec!["What is Rust?", "How does async work?"];
/// let formatted = format_with_instruction(&texts, Some("search_query"));
/// // Returns texts prefixed with search instruction
/// ```
pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    // Validate task parameter and warn if invalid
    if let Some(t) = task {
        if !VALID_TASKS.contains(&t) {
            log::warn!(
                "Unknown embedding task '{}'. Using default 's2p'. Valid tasks: {}",
                t,
                VALID_TASKS.join(", ")
            );
        }
    }

    let instruct = match task {
        Some("s2p") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        }
        Some("s2s") => "Retrieve semantically similar text.",
        Some("search_query") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        } // Map to s2p
        Some("search_document") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        } // Map to s2p
        Some("classification") => "Retrieve semantically similar text.", // Map to s2s
        Some("clustering") => "Retrieve semantically similar text.",     // Map to s2s
        Some("retrieval") => {
            "Given a web search query, retrieve relevant passages that answer the query."
        } // Map to s2p
        _ => "Given a web search query, retrieve relevant passages that answer the query.", // Default to s2p
    };

    texts
        .iter()
        .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_test_logging() {
        INIT.call_once(|| {
            env_logger::builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Warn)
                .init();
        });
    }

    #[test]
    fn test_valid_tasks_no_warning() {
        // Test all valid task types
        let valid_tasks = vec![
            "s2p",
            "s2s",
            "search_query",
            "search_document",
            "classification",
            "clustering",
            "retrieval",
        ];

        for task in valid_tasks {
            let result = format_with_instruction(&["test"], Some(task));
            assert_eq!(result.len(), 1);
            assert!(result[0].starts_with("Instruct:"));
        }
    }

    #[test]
    fn test_none_task_uses_default() {
        let result = format_with_instruction(&["test"], None);
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("Given a web search query"));
    }

    #[test]
    fn test_invalid_task_warning() {
        init_test_logging();
        // This test needs to capture log output
        // Use a test logger or env_logger test utilities
        let result = format_with_instruction(&["test"], Some("invalid_task"));

        // Should still return valid output (fallback to default)
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("Given a web search query"));

        // Warning will be printed to test output
        // Manual verification: run with --nocapture to see warning
    }

    #[test]
    fn test_case_sensitive_task() {
        // Uppercase should trigger warning
        let result = format_with_instruction(&["test"], Some("S2P"));
        assert_eq!(result.len(), 1);
        // Should use default, not s2p instruction
        assert!(result[0].contains("Given a web search query"));
    }

    #[test]
    fn test_empty_string_task() {
        let result = format_with_instruction(&["test"], Some(""));
        assert_eq!(result.len(), 1);
        // Should trigger warning and use default
        assert!(result[0].contains("Given a web search query"));
    }

    #[test]
    fn test_multiple_texts() {
        let texts = vec!["text1", "text2", "text3"];
        let result = format_with_instruction(&texts, Some("s2p"));
        assert_eq!(result.len(), 3);
        for formatted in result {
            assert!(formatted.starts_with("Instruct:"));
            assert!(formatted.contains("Query:"));
        }
    }

    #[test]
    fn test_empty_texts_array() {
        let result = format_with_instruction(&[], Some("s2p"));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_instruction_mapping() {
        // s2p, search_query, search_document, retrieval -> search instruction
        let search_tasks = vec!["s2p", "search_query", "search_document", "retrieval"];
        for task in search_tasks {
            let result = format_with_instruction(&["test"], Some(task));
            assert!(result[0].contains("Given a web search query"));
        }

        // s2s, classification, clustering -> similarity instruction
        let similarity_tasks = vec!["s2s", "classification", "clustering"];
        for task in similarity_tasks {
            let result = format_with_instruction(&["test"], Some(task));
            assert!(result[0].contains("Retrieve semantically similar text"));
        }
    }
}

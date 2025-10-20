//! Task-specific instruction formatting for Stella embeddings

/// Format texts with task-specific instruction prefix following canonical Stella example
pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
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

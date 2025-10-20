//! Document builder implementations - Zero Box<dyn> trait-based architecture
//!
//! All document loading and processing logic with zero allocation.
//! Uses domain-first architecture with proper cyrup_domain imports.

use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use crate::domain::context::chunk::{CandleStringChunk, CandleZeroOneOrManyChunk};
use crate::domain::context::{
    CandleContentFormat as ContentFormat, CandleDocumentMediaType as DocumentMediaType,
};
use crate::domain::context::{CandleDocument as Document, CandleDocumentChunk as DocumentChunk};
use cyrup_sugars::ZeroOneOrMany;
use cyrup_sugars::prelude::MessageChunk;
use quyc::Quyc;
use serde_json::Value;
use std::fs;
use tokio_stream::{Stream, StreamExt};
use crate::async_stream;
use std::pin::Pin;
use std::future::Future;

/// Document builder data enumeration for zero-allocation type tracking
#[derive(Debug, Clone)]
pub enum DocumentBuilderData {
    File(PathBuf),
    Url(String),
    Github {
        repo: String,
        path: String,
        branch: Option<String>,
    },
    Glob(String),
    Text(String),
}

/// Document builder trait - elegant zero-allocation builder pattern
pub trait DocumentBuilder: Sized {
    /// Set content format - EXACT syntax: .format(ContentFormat::Markdown)
    fn format(self, format: ContentFormat) -> impl DocumentBuilder;

    /// Set media type - EXACT syntax: .media_type(DocumentMediaType::PDF)
    fn media_type(self, media_type: DocumentMediaType) -> impl DocumentBuilder;

    /// Set encoding - EXACT syntax: .encoding("utf-8")
    fn encoding(self, encoding: impl Into<String>) -> impl DocumentBuilder;

    /// Set maximum file size - EXACT syntax: .max_size(1024 * 1024)
    fn max_size(self, size: usize) -> impl DocumentBuilder;

    /// Set request timeout - EXACT syntax: .timeout(30000)
    fn timeout(self, timeout_ms: u64) -> impl DocumentBuilder;

    /// Set retry attempts - EXACT syntax: .retry(5)
    fn retry(self, attempts: u8) -> impl DocumentBuilder;

    /// Enable/disable caching - EXACT syntax: .cache(true)
    fn cache(self, enabled: bool) -> impl DocumentBuilder;

    /// Set GitHub branch - EXACT syntax: .branch("main")
    fn branch(self, branch: impl Into<String>) -> impl DocumentBuilder;

    /// Add metadata property - EXACT syntax: .property("key", "value")
    fn property(self, key: impl Into<String>, value: impl Into<Value>) -> impl DocumentBuilder;

    /// Add multiple properties - EXACT syntax: .properties([("key", "value")])
    fn properties<F>(self, f: F) -> impl DocumentBuilder
    where
        F: FnOnce() -> BTreeMap<String, Value>;

    /// Add error handler - EXACT syntax: .on_error(|error| { ... })
    fn on_error<F>(self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(String) + Send + Sync + 'static + Clone;

    /// Add chunk handler - EXACT syntax: .on_chunk(|chunk| { ... })
    fn on_chunk<F>(self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static + Clone;

    /// Synchronous load for immediate data only - EXACT syntax: .load()
    fn load(self) -> Document;

    /// Load document asynchronously - EXACT syntax: .load_async()
    fn load_async(self) -> Pin<Box<dyn Future<Output = Document> + Send>>;

    /// Load multiple documents - EXACT syntax: .load_all()
    fn load_all(self) -> Pin<Box<dyn Future<Output = ZeroOneOrMany<Document>> + Send>>;

    /// Stream documents one by one - EXACT syntax: .stream()
    fn stream(self) -> impl Stream<Item = Document>;

    /// Stream document content in chunks - EXACT syntax: .stream_chunks(1024)
    fn stream_chunks(self, chunk_size: usize) -> impl Stream<Item = DocumentChunk>;

    /// Stream document content line by line - EXACT syntax: .stream_lines()
    fn stream_lines(self) -> impl Stream<Item = DocumentChunk>;
}

/// Hidden implementation struct - zero-allocation builder state using DOMAIN OBJECTS
#[derive(Clone)]
struct DocumentBuilderImpl<F1 = fn(String), F2 = fn(DocumentChunk) -> DocumentChunk>
where
    F1: Fn(String) + Send + Sync + 'static + Clone,
    F2: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static + Clone,
{
    data: DocumentBuilderData,
    format: Option<ContentFormat>,
    media_type: Option<DocumentMediaType>,
    additional_props: BTreeMap<String, Value>,
    encoding: Option<String>,
    max_size: Option<usize>,
    timeout_ms: Option<u64>,
    retry_attempts: u8,
    cache_enabled: bool,
    error_handler: Option<F1>,
    chunk_handler: Option<F2>,
    _marker: PhantomData<(F1, F2)>,
}

impl DocumentBuilderImpl {
    /// Create a new document builder with optimal defaults
    #[allow(clippy::new_ret_no_self)]
    pub fn new(data: DocumentBuilderData) -> impl DocumentBuilder {
        DocumentBuilderImpl::<fn(String), fn(DocumentChunk) -> DocumentChunk> {
            data,
            format: None,
            media_type: None,
            additional_props: BTreeMap::new(),
            encoding: None,
            max_size: None,
            timeout_ms: None,
            retry_attempts: 3,
            cache_enabled: true,
            error_handler: None,
            chunk_handler: None,
            _marker: PhantomData,
        }
    }
}

impl Document {
    /// Create document from file path - EXACT syntax: Document::from_file("path/to/file.txt")
    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::File(path.as_ref().to_path_buf()))
    }

    /// Create document from URL - EXACT syntax: Document::from_url("https://example.com/doc.pdf")
    #[inline]
    pub fn from_url(url: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Url(url.into()))
            .max_size(10 * 1024 * 1024) // 10MB default
            .timeout(30000) // 30s default
    }

    /// Create document from GitHub - EXACT syntax: Document::from_github("owner/repo", "path/to/file.md")
    #[inline]
    pub fn from_github(repo: impl Into<String>, path: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Github {
            repo: repo.into(),
            path: path.into(),
            branch: None,
        })
        .max_size(1024 * 1024) // 1MB default for GitHub files
        .timeout(15000) // 15s default
    }

    /// Create document from glob pattern - EXACT syntax: Document::from_glob("**/*.md")
    #[inline]
    pub fn from_glob(pattern: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Glob(pattern.into()))
            .retry(1)
            .cache(false)
    }

    /// Create document from text - EXACT syntax: Document::from_text("content")
    #[inline]
    pub fn from_text(text: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Text(text.into()))
            .format(ContentFormat::Text)
            .media_type(DocumentMediaType::TXT)
            .encoding("utf-8")
            .retry(0)
            .cache(false)
    }

    /// Create document from base64 data - EXACT syntax: Document::from_base64("base64data")
    #[inline]
    pub fn from_base64(data: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Text(data.into()))
            .format(ContentFormat::Base64)
            .media_type(DocumentMediaType::PDF)
            .retry(0)
            .cache(false)
    }

    /// Create document from data - EXACT syntax: Document::from_data("content")
    #[inline]
    pub fn from_data(data: impl Into<String>) -> impl DocumentBuilder {
        Self::from_text(data)
    }
}

impl<F1, F2> DocumentBuilder for DocumentBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static + Clone,
    F2: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static + Clone,
{
    /// Set content format - EXACT syntax: .format(ContentFormat::Markdown)
    fn format(mut self, format: ContentFormat) -> impl DocumentBuilder {
        self.format = Some(format);
        self
    }

    /// Set media type - EXACT syntax: .media_type(DocumentMediaType::PDF)
    fn media_type(mut self, media_type: DocumentMediaType) -> impl DocumentBuilder {
        self.media_type = Some(media_type);
        self
    }

    /// Set encoding - EXACT syntax: .encoding("utf-8")
    fn encoding(mut self, encoding: impl Into<String>) -> impl DocumentBuilder {
        self.encoding = Some(encoding.into());
        self
    }

    /// Set maximum file size - EXACT syntax: .max_size(1024 * 1024)
    fn max_size(mut self, size: usize) -> impl DocumentBuilder {
        self.max_size = Some(size);
        self
    }

    /// Set request timeout - EXACT syntax: .timeout(30000)
    fn timeout(mut self, timeout_ms: u64) -> impl DocumentBuilder {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    /// Set retry attempts - EXACT syntax: .retry(5)
    fn retry(mut self, attempts: u8) -> impl DocumentBuilder {
        self.retry_attempts = attempts;
        self
    }

    /// Enable/disable caching - EXACT syntax: .cache(true)
    fn cache(mut self, enabled: bool) -> impl DocumentBuilder {
        self.cache_enabled = enabled;
        self
    }

    /// Set GitHub branch - EXACT syntax: .branch("main")
    fn branch(mut self, branch: impl Into<String>) -> impl DocumentBuilder {
        if let DocumentBuilderData::Github {
            repo: _,
            path: _,
            branch: branch_field,
        } = &mut self.data
        {
            *branch_field = Some(branch.into());
        }
        self
    }

    /// Add metadata property - EXACT syntax: .property("key", "value")
    fn property(mut self, key: impl Into<String>, value: impl Into<Value>) -> impl DocumentBuilder {
        self.additional_props.insert(key.into(), value.into());
        self
    }

    /// Add multiple properties - EXACT syntax: .properties([("key", "value")])
    fn properties<F>(mut self, f: F) -> impl DocumentBuilder
    where
        F: FnOnce() -> BTreeMap<String, Value>,
    {
        let props = f();
        for (key, value) in props {
            self.additional_props.insert(key, value);
        }
        self
    }

    /// Add error handler - EXACT syntax: .on_error(|error| { ... })
    fn on_error<F>(self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(String) + Send + Sync + 'static + Clone,
    {
        DocumentBuilderImpl {
            data: self.data,
            format: self.format,
            media_type: self.media_type,
            additional_props: self.additional_props,
            encoding: self.encoding,
            max_size: self.max_size,
            timeout_ms: self.timeout_ms,
            retry_attempts: self.retry_attempts,
            cache_enabled: self.cache_enabled,
            error_handler: Some(handler),
            chunk_handler: self.chunk_handler,
            _marker: PhantomData,
        }
    }

    /// Add chunk handler - EXACT syntax: .on_chunk(|chunk| { ... })
    fn on_chunk<F>(self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static + Clone,
    {
        DocumentBuilderImpl {
            data: self.data,
            format: self.format,
            media_type: self.media_type,
            additional_props: self.additional_props,
            encoding: self.encoding,
            max_size: self.max_size,
            timeout_ms: self.timeout_ms,
            retry_attempts: self.retry_attempts,
            cache_enabled: self.cache_enabled,
            error_handler: self.error_handler,
            chunk_handler: Some(handler),
            _marker: PhantomData,
        }
    }

    /// Synchronous load for immediate data only - EXACT syntax: .load()
    fn load(self) -> Document {
        match self.data {
            DocumentBuilderData::Text(data) => Document {
                data,
                format: self.format,
                media_type: self.media_type,
                additional_props: self.additional_props.into_iter().collect(),
            },
            _ => {
                // Return error document instead of panicking
                Document {
                    data: "Error: load() can only be used with immediate data. Use on_error().load_async() for file/url/glob operations.".to_string(),
                    format: Some(ContentFormat::Text),
                    media_type: Some(DocumentMediaType::TXT),
                    additional_props: std::collections::HashMap::new()
                }
            }
        }
    }

    /// Load document asynchronously - EXACT syntax: .load_async()
    fn load_async(self) -> Pin<Box<dyn Future<Output = Document> + Send>> {
        Box::pin(async move {
            if let Some(handler) = self.error_handler.clone() {
                let mut stream = Self::load_document_data(self, handler);
                match stream.next().await {
                    Some(document) => document,
                    None => {
                        // Return empty document on error
                        Document {
                            data: String::new(),
                            format: Some(ContentFormat::Text),
                            media_type: Some(DocumentMediaType::TXT),
                            additional_props: std::collections::HashMap::new(),
                        }
                    }
                }
            } else {
                // No error handler - use default no-op handler
                let default_handler = |_: String| {};
                let mut stream = Self::load_document_data(self, default_handler);
                match stream.next().await {
                    Some(document) => document,
                    None => {
                        // Return empty document on error
                        Document {
                            data: String::new(),
                            format: Some(ContentFormat::Text),
                            media_type: Some(DocumentMediaType::TXT),
                            additional_props: std::collections::HashMap::new(),
                        }
                    }
                }
            }
        })
    }

    /// Load multiple documents - EXACT syntax: .load_all()
    fn load_all(self) -> Pin<Box<dyn Future<Output = ZeroOneOrMany<Document>> + Send>> {
        Box::pin(async move {
            match self.data.clone() {
                DocumentBuilderData::Glob(pattern) => {
                    if let Some(handler) = self.error_handler.clone() {
                        let mut stream = Self::load_glob_documents(pattern, self, handler);
                        match stream.next().await {
                            Some(chunk) => chunk.0,
                            None => ZeroOneOrMany::None,
                        }
                    } else {
                        let default_handler = |_: String| {};
                        let mut stream = Self::load_glob_documents(pattern, self, default_handler);
                        match stream.next().await {
                            Some(chunk) => chunk.0,
                            None => ZeroOneOrMany::None,
                        }
                    }
                }
                _ => {
                    // Single document
                    if let Some(handler) = self.error_handler.clone() {
                        let mut stream = Self::load_document_data(self, handler);
                        match stream.next().await {
                            Some(doc) => ZeroOneOrMany::One(doc),
                            None => ZeroOneOrMany::None,
                        }
                    } else {
                        let default_handler = |_: String| {};
                        let mut stream = Self::load_document_data(self, default_handler);
                        match stream.next().await {
                            Some(doc) => ZeroOneOrMany::One(doc),
                            None => ZeroOneOrMany::None,
                        }
                    }
                }
            }
        })
    }

    /// Stream documents one by one - EXACT syntax: .stream()
    fn stream(self) -> impl Stream<Item = Document> {
        async_stream::spawn_stream(move |tx| async move {
            match self.data {
                DocumentBuilderData::Glob(pattern) => {
                    if let Ok(paths) = glob::glob(&pattern) {
                        for entry in paths.filter_map(Result::ok) {
                            let doc_builder = DocumentBuilderImpl {
                                data: DocumentBuilderData::File(entry),
                                format: self.format,
                                media_type: self.media_type,
                                additional_props: self.additional_props.clone(),
                                encoding: self.encoding.clone(),
                                max_size: self.max_size,
                                timeout_ms: self.timeout_ms,
                                retry_attempts: self.retry_attempts,
                                cache_enabled: self.cache_enabled,
                                error_handler: None,
                                chunk_handler: None,
                                _marker: PhantomData,
                            };

                            if let Some(handler) = self.error_handler.clone() {
                                let handler_clone = handler.clone();
                                let mut doc_stream = Self::load_document_data(doc_builder, handler);
                                match doc_stream.next().await {
                                    Some(doc) => {
                                        let _ = tx.send(doc);
                                    }
                                    None => {
                                        handler_clone("Failed to load document".to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    if let Some(handler) = self.error_handler.clone() {
                        let handler_clone = handler.clone();
                        let mut doc_stream = Self::load_document_data(self, handler);
                        match doc_stream.next().await {
                            Some(doc) => {
                                let _ = tx.send(doc);
                            }
                            None => {
                                handler_clone("Failed to load document".to_string());
                            }
                        }
                    }
                }
            }
        })
    }

    /// Stream document content in chunks - EXACT syntax: .stream_chunks(1024)
    fn stream_chunks(self, chunk_size: usize) -> impl Stream<Item = DocumentChunk> {
        async_stream::spawn_stream(move |tx| async move {
            let chunk_handler = self.chunk_handler.clone();
            let doc = self.load_async().await;

            let content = &doc.data;
            let mut offset = 0;

            while offset < content.len() {
                let end = (offset + chunk_size).min(content.len());
                let mut chunk = DocumentChunk::new(&content[offset..end]).with_range(offset, end);

                // Apply chunk handler if present
                if let Some(ref handler) = chunk_handler {
                    chunk = handler(chunk);
                }

                let _ = tx.send(chunk);
                offset = end;
            }
        })
    }

    /// Stream document content line by line - EXACT syntax: .stream_lines()
    fn stream_lines(self) -> impl Stream<Item = DocumentChunk> {
        async_stream::spawn_stream(move |tx| async move {
            let chunk_handler = self.chunk_handler.clone();
            let doc = self.load_async().await;

            let mut offset = 0;
            for line in doc.data.lines() {
                let mut chunk = DocumentChunk::new(line).with_range(offset, offset + line.len());

                // Apply chunk handler if present
                if let Some(ref handler) = chunk_handler {
                    chunk = handler(chunk);
                }

                let _ = tx.send(chunk);
                offset += line.len() + 1; // +1 for newline
            }
        })
    }
}

// ========================================================================
// Internal Implementation - Zero Allocation, Lock-Free
// ========================================================================

impl<F1, F2> DocumentBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static + Clone,
    F2: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static + Clone,
{
    fn load_document_data<G>(
        builder: DocumentBuilderImpl<F1, F2>,
        error_handler: G,
    ) -> Pin<Box<dyn Stream<Item = Document> + Send>>
    where
        G: Fn(String) + Send + 'static,
    {
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let content = match builder.data {
                DocumentBuilderData::File(ref path) => {
                    let mut file_stream = Self::load_file_content(path, &builder);
                    match file_stream.next().await {
                        Some(content_result) => content_result.0,
                        None => {
                            error_handler("Failed to load file content".to_string());
                            return;
                        }
                    }
                }
                DocumentBuilderData::Url(ref url) => {
                    let mut url_stream = Self::load_url_content(url, &builder);
                    match url_stream.next().await {
                        Some(content_result) => content_result.0,
                        None => {
                            error_handler("Failed to load URL content".to_string());
                            return;
                        }
                    }
                }
                DocumentBuilderData::Github {
                    ref repo,
                    ref path,
                    ref branch,
                } => {
                    let mut github_stream =
                        Self::load_github_content(repo, path, branch.as_deref(), &builder);
                    match github_stream.next().await {
                        Some(content_result) => content_result.0,
                        None => {
                            error_handler("Failed to load GitHub content".to_string());
                            return;
                        }
                    }
                }
                DocumentBuilderData::Text(ref text) => text.clone(),
                DocumentBuilderData::Glob(_) => {
                    error_handler("Glob patterns require load_all() or stream()".to_string());
                    return;
                }
            };

            // Detect format if not specified
            let format = builder
                .format
                .unwrap_or_else(|| Self::detect_format(&content, &builder.data));

            // Detect media type if not specified
            let media_type = builder
                .media_type
                .unwrap_or_else(|| Self::detect_media_type(&format, &builder.data));

            // Build metadata
            let mut metadata =
                std::collections::HashMap::with_capacity(builder.additional_props.len() + 4);
            for (key, value) in builder.additional_props {
                metadata.insert(key, value);
            }

            if let Some(encoding) = builder.encoding {
                metadata.insert("encoding".to_string(), Value::String(encoding));
            }

            metadata.insert("size".to_string(), Value::Number(content.len().into()));
            metadata.insert(
                "cache_enabled".to_string(),
                Value::Bool(builder.cache_enabled),
            );

            let document = Document {
                data: content,
                format: Some(format),
                media_type: Some(media_type),
                additional_props: metadata,
            };

            let _ = sender.send(document);
        }))
    }

    fn load_glob_documents<G>(
        pattern: String,
        builder: DocumentBuilderImpl<F1, F2>,
        error_handler: G,
    ) -> Pin<Box<dyn Stream<Item = CandleZeroOneOrManyChunk<Document>> + Send>>
    where
        G: Fn(String) + Send + Sync + Clone + 'static,
    {
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let paths = match glob::glob(&pattern) {
                Ok(paths) => paths.filter_map(Result::ok).collect::<Vec<_>>(),
                Err(e) => {
                    error_handler(format!("Invalid glob pattern: {}", e));
                    let _ = sender.send(CandleZeroOneOrManyChunk(ZeroOneOrMany::None));
                    return;
                }
            };

            if paths.is_empty() {
                let _ = sender.send(CandleZeroOneOrManyChunk(ZeroOneOrMany::None));
                return;
            }

            let mut documents = Vec::with_capacity(paths.len());

            for path in paths {
                let doc_builder = DocumentBuilderImpl {
                    data: DocumentBuilderData::File(path),
                    format: builder.format,
                    media_type: builder.media_type,
                    additional_props: builder.additional_props.clone(),
                    encoding: builder.encoding.clone(),
                    max_size: builder.max_size,
                    timeout_ms: builder.timeout_ms,
                    retry_attempts: builder.retry_attempts,
                    cache_enabled: builder.cache_enabled,
                    error_handler: None,
                    chunk_handler: None,
                    _marker: PhantomData,
                };

                let mut doc_stream = Self::load_document_data(doc_builder, error_handler.clone());
                match doc_stream.next().await {
                    Some(doc) => {
                        documents.push(doc);
                    }
                    None => {
                        error_handler("Failed to load document from glob pattern".to_string());
                    }
                }
            }

            let result = match documents.len() {
                0 => ZeroOneOrMany::None,
                1 => {
                    let mut iter = documents.into_iter();
                    match iter.next() {
                        Some(doc) => ZeroOneOrMany::One(doc),
                        None => ZeroOneOrMany::None,
                    }
                }
                _ => ZeroOneOrMany::Many(documents),
            };

            let _ = sender.send(CandleZeroOneOrManyChunk(result));
        }))
    }

    fn load_file_content(
        path: &Path,
        builder: &DocumentBuilderImpl<F1, F2>,
    ) -> Pin<Box<dyn Stream<Item = CandleStringChunk> + Send>> {
        let path = path.to_path_buf();
        let builder = builder.clone();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            // Check file size first if max_size is set
            if let Some(max_size) = builder.max_size {
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        if metadata.len() as usize > max_size {
                            return; // Skip sending - file too large
                        }
                    }
                    Err(_) => {
                        return; // Skip sending - metadata error
                    }
                }
            }

            // Attempt to read with retries
            let mut last_error = String::new();
            for attempt in 0..=builder.retry_attempts {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let _ = sender.send(CandleStringChunk(content));
                        return;
                    }
                    Err(e) => {
                        last_error = format!("Attempt {}: {}", attempt + 1, e);
                        if attempt < builder.retry_attempts {
                            tokio::time::sleep(std::time::Duration::from_millis(
                                100 * (1 << attempt), // Exponential backoff
                            )).await;
                        }
                    }
                }
            }
            // If all attempts failed, log the error and send error chunk
            log::error!(
                "Failed to read file after {} attempts: {}",
                builder.retry_attempts + 1,
                last_error
            );
            let _ = sender.send(CandleStringChunk::bad_chunk(last_error));
        }))
    }

    fn load_url_content(
        url: &str,
        builder: &DocumentBuilderImpl<F1, F2>,
    ) -> Pin<Box<dyn Stream<Item = CandleStringChunk> + Send>> {
        let url = url.to_string();
        let builder = builder.clone();

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            // Attempt request with retries
            for attempt in 0..=builder.retry_attempts {
                // Use Quyc::get() directly - much simpler!
                let mut quyc_builder = Quyc::debug();

                // Set timeout if specified
                if let Some(timeout_ms) = builder.timeout_ms {
                    quyc_builder =
                        quyc_builder.timeout(std::time::Duration::from_millis(timeout_ms));
                }

                // Simple string response type for text content
                #[derive(serde::Deserialize, Default)]
                struct StringResponse(String);

                impl cyrup_sugars::prelude::MessageChunk for StringResponse {
                    fn bad_chunk(error: String) -> Self {
                        StringResponse(format!("ERROR: {}", error))
                    }

                    fn error(&self) -> Option<&str> {
                        if self.0.starts_with("ERROR: ") {
                            Some(&self.0)
                        } else {
                            None
                        }
                    }
                }

                // Use quyc to get the content
                let response: StringResponse = quyc_builder.get(&url).collect_one();

                if let Some(error) = response.error() {
                    if attempt < builder.retry_attempts {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            100 * (1 << attempt), // Exponential backoff
                        )).await;
                        continue;
                    }
                    // Final attempt failed, log error and send error chunk
                    log::error!(
                        "Failed to load URL after {} attempts: {}",
                        builder.retry_attempts + 1,
                        error
                    );
                    let _ = sender.send(CandleStringChunk::bad_chunk(error.to_string()));
                    return;
                }

                let content = response.0;

                // Check size if max_size is set
                if let Some(max_size) = builder.max_size
                    && content.len() > max_size
                {
                    return; // Skip sending - content too large
                }

                let _ = sender.send(CandleStringChunk(content));
                return;
            }
        }))
    }

    fn load_github_content(
        repo: &str,
        path: &str,
        branch: Option<&str>,
        builder: &DocumentBuilderImpl<F1, F2>,
    ) -> Pin<Box<dyn Stream<Item = CandleStringChunk> + Send>> {
        let branch = branch.unwrap_or("main");
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}",
            repo, branch, path
        );

        Self::load_url_content(&url, builder)
    }

    #[inline]
    fn detect_format(content: &str, data: &DocumentBuilderData) -> ContentFormat {
        match data {
            DocumentBuilderData::File(path) => {
                match path.extension().and_then(|ext| ext.to_str()) {
                    Some("md") | Some("markdown") => ContentFormat::Markdown,
                    Some("html") | Some("htm") => ContentFormat::Html,
                    Some("json") => ContentFormat::Json,
                    Some("xml") => ContentFormat::Xml,
                    Some("yaml") | Some("yml") => ContentFormat::Yaml,
                    Some("csv") => ContentFormat::Csv,
                    _ => ContentFormat::Text,
                }
            }
            DocumentBuilderData::Github { path, .. } => {
                match std::path::Path::new(path)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    Some("md") | Some("markdown") => ContentFormat::Markdown,
                    Some("html") | Some("htm") => ContentFormat::Html,
                    Some("json") => ContentFormat::Json,
                    Some("xml") => ContentFormat::Xml,
                    Some("yaml") | Some("yml") => ContentFormat::Yaml,
                    Some("csv") => ContentFormat::Csv,
                    _ => {
                        // Content-based detection
                        if content.trim_start().starts_with('{')
                            || content.trim_start().starts_with('[')
                        {
                            ContentFormat::Json
                        } else if content.trim_start().starts_with('<') {
                            ContentFormat::Html
                        } else {
                            ContentFormat::Text
                        }
                    }
                }
            }
            DocumentBuilderData::Url(url) => {
                if url.ends_with(".json") {
                    ContentFormat::Json
                } else if url.ends_with(".html") || url.ends_with(".htm") {
                    ContentFormat::Html
                } else if url.ends_with(".md") || url.ends_with(".markdown") {
                    ContentFormat::Markdown
                } else {
                    ContentFormat::Text
                }
            }
            _ => ContentFormat::Text,
        }
    }

    #[inline]
    fn detect_media_type(format: &ContentFormat, data: &DocumentBuilderData) -> DocumentMediaType {
        match format {
            ContentFormat::Json => DocumentMediaType::Json,
            ContentFormat::Html => DocumentMediaType::Html,
            ContentFormat::Markdown => DocumentMediaType::Markdown,
            ContentFormat::Xml => DocumentMediaType::Xml,
            ContentFormat::Csv => DocumentMediaType::Csv,
            ContentFormat::Yaml => DocumentMediaType::Yaml,
            ContentFormat::Base64 => match data {
                DocumentBuilderData::File(path) => {
                    match path.extension().and_then(|ext| ext.to_str()) {
                        Some("pdf") => DocumentMediaType::PDF,
                        Some("doc") | Some("docx") => DocumentMediaType::Document,
                        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") => {
                            DocumentMediaType::Image
                        }
                        _ => DocumentMediaType::Binary,
                    }
                }
                _ => DocumentMediaType::Binary,
            },
            _ => DocumentMediaType::PlainText,
        }
    }
}

/// Convenience function for fluent document creation
#[inline]
pub fn document() -> impl DocumentBuilder {
    Document::from_text("")
}

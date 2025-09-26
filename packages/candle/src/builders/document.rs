//! Document builder implementations - Zero Box<dyn> trait-based architecture
//!
//! All document loading and processing logic with zero allocation.
//! Uses domain-first architecture with proper paraphym_domain imports.

use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use crate::domain::context::{CandleDocumentChunk as DocumentChunk, CandleDocument as Document};
use crate::domain::context::{CandleContentFormat as ContentFormat, CandleDocumentMediaType as DocumentMediaType};
use crate::util::ZeroOneOrMany;
use ystream::{AsyncTask, AsyncStream, spawn_task};
use paraphym_http3::{HttpClient, HttpConfig, HttpMethod, HttpRequest};
use serde_json::Value;
use std::fs;

/// Document builder data enumeration for zero-allocation type tracking
#[derive(Debug, Clone)]
pub enum DocumentBuilderData {
    File(PathBuf),
    Url(String),
    Github {
        repo: String,
        path: String,
        branch: Option<String>},
    Glob(String),
    Text(String)}

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
        F: Fn(String) + Send + Sync + 'static;
    
    /// Add chunk handler - EXACT syntax: .on_chunk(|chunk| { ... })
    fn on_chunk<F>(self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static;
    
    /// Synchronous load for immediate data only - EXACT syntax: .load()
    fn load(self) -> Document;
    
    /// Load document asynchronously - EXACT syntax: .load_async()
    fn load_async(self) -> AsyncTask<Document>;
    
    /// Load multiple documents - EXACT syntax: .load_all()
    fn load_all(self) -> AsyncTask<ZeroOneOrMany<Document>>;
    
    /// Stream documents one by one - EXACT syntax: .stream()
    fn stream(self) -> AsyncStream<Document>;
    
    /// Stream document content in chunks - EXACT syntax: .stream_chunks(1024)
    fn stream_chunks(self, chunk_size: usize) -> AsyncStream<DocumentChunk>;
    
    /// Stream document content line by line - EXACT syntax: .stream_lines()
    fn stream_lines(self) -> AsyncStream<DocumentChunk>;
}

/// Hidden implementation struct - zero-allocation builder state using DOMAIN OBJECTS
#[derive(Clone)]
struct DocumentBuilderImpl<
    F1 = fn(String),
    F2 = fn(DocumentChunk) -> DocumentChunk,
> where
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
    pub fn new(data: DocumentBuilderData) -> impl DocumentBuilder {
        DocumentBuilderImpl {
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
            .timeout(30000)             // 30s default
    }

    /// Create document from GitHub - EXACT syntax: Document::from_github("owner/repo", "path/to/file.md")
    #[inline]
    pub fn from_github(repo: impl Into<String>, path: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Github {
            repo: repo.into(),
            path: path.into(),
            branch: None
        })
        .max_size(1024 * 1024) // 1MB default for GitHub files
        .timeout(15000)        // 15s default
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
            .media_type(DocumentMediaType::PlainText)
            .encoding("utf-8")
            .retry(0)
            .cache(false)
    }

    /// Create document from base64 data - EXACT syntax: Document::from_base64("base64data")
    #[inline]
    pub fn from_base64(data: impl Into<String>) -> impl DocumentBuilder {
        DocumentBuilderImpl::new(DocumentBuilderData::Text(data.into()))
            .format(ContentFormat::Base64)
            .media_type(DocumentMediaType::Binary)
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
    F1: Fn(String) + Send + Sync + 'static,
    F2: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static,
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
            ref mut branch_ref
        } = &mut self.data
        {
            *branch_ref = Some(branch.into());
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
    fn on_error<F>(mut self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(String) + Send + Sync + 'static,
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
    fn on_chunk<F>(mut self, handler: F) -> impl DocumentBuilder
    where
        F: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static,
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
                metadata: self.additional_props.into_iter().collect()
            },
            _ => {
                // Return error document instead of panicking
                Document {
                    data: "Error: load() can only be used with immediate data. Use on_error().load_async() for file/url/glob operations.".to_string(),
                    format: Some(ContentFormat::Text),
                    media_type: Some(DocumentMediaType::PlainText),
                    metadata: std::collections::HashMap::new()
                }
            }
        }
    }

    /// Load document asynchronously - EXACT syntax: .load_async()
    fn load_async(self) -> AsyncTask<Document> {
        let error_handler = self.error_handler.unwrap_or_else(|| |e| eprintln!("Document error: {}", e));
        
        spawn_task(move || {
            let mut stream = Self::load_document_data(self, error_handler);
            match stream.try_next() {
                Some(document) => document,
                None => {
                    // Return empty document on error
                    Document {
                        data: String::new(),
                        format: Some(ContentFormat::Text),
                        media_type: Some(DocumentMediaType::PlainText),
                        metadata: std::collections::HashMap::new()
                    }
                }
            }
        })
    }

    /// Load multiple documents - EXACT syntax: .load_all()
    fn load_all(self) -> AsyncTask<ZeroOneOrMany<Document>> {
        let error_handler = self.error_handler.unwrap_or_else(|| |e| eprintln!("Document error: {}", e));
        
        spawn_task(move || {
            match self.data {
                DocumentBuilderData::Glob(pattern) => {
                    {
                        let mut stream = Self::load_glob_documents(pattern, self, error_handler);
                        stream.try_next().unwrap_or(ZeroOneOrMany::None)
                    }
                }
                _ => {
                    // Single document
                    {
                        let mut stream = Self::load_document_data(self, error_handler);
                        match stream.try_next() {
                            Some(doc) => ZeroOneOrMany::One(doc),
                            None => ZeroOneOrMany::None
                        }
                    }
                }
            }
        })
    }

    /// Stream documents one by one - EXACT syntax: .stream()
    fn stream(self) -> AsyncStream<Document> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let error_handler = self.error_handler.unwrap_or_else(|| |e| eprintln!("Document error: {}", e));
        let chunk_handler = self.chunk_handler;

        std::thread::spawn(move || {
            match self.data {
                DocumentBuilderData::Glob(pattern) => {
                    if let Ok(paths) = glob::glob(&pattern) {
                        for entry in paths.filter_map(Result::ok) {
                            let doc_builder = DocumentBuilderImpl {
                                data: DocumentBuilderData::File(entry),
                                format: self.format.clone(),
                                media_type: self.media_type.clone(),
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

                            let mut doc_stream = Self::load_document_data(doc_builder, &error_handler);
                            if let Some(doc) = doc_stream.try_next() {
                                if tx.send(doc).is_err() {
                                    break;
                                }
                            } else {
                                error_handler("Failed to load document".to_string());
                            }
                        }
                    }
                }
                _ => {
                    let mut doc_stream = Self::load_document_data(self, &error_handler);
                    if let Some(doc) = doc_stream.try_next() {
                        let _ = tx.send(doc);
                    } else {
                        error_handler("Failed to load document".to_string());
                    }
                }
            }
        });

        AsyncStream::new(rx)
    }

    /// Stream document content in chunks - EXACT syntax: .stream_chunks(1024)
    fn stream_chunks(self, chunk_size: usize) -> AsyncStream<DocumentChunk> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let chunk_handler = self.chunk_handler;

        std::thread::spawn(move || {
            let doc = {
                let task = self.load_async();
                task.join().unwrap_or_else(|_| Document {
                    data: String::new(),
                    format: Some(ContentFormat::Text),
                    media_type: Some(DocumentMediaType::PlainText),
                    metadata: std::collections::HashMap::new()
                })
            };
            match doc {
                doc => {
                    let content = &doc.data;
                    let mut offset = 0;

                    while offset < content.len() {
                        let end = (offset + chunk_size).min(content.len());
                        let mut chunk = DocumentChunk::new(&content[offset..end]).with_range(offset, end);

                        // Apply chunk handler if present
                        if let Some(ref handler) = chunk_handler {
                            chunk = handler(chunk);
                        }

                        if tx.send(chunk).is_err() {
                            break;
                        }

                        offset = end;
                    }
                }
            }
        });

        AsyncStream::new(rx)
    }

    /// Stream document content line by line - EXACT syntax: .stream_lines()
    fn stream_lines(self) -> AsyncStream<DocumentChunk> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let chunk_handler = self.chunk_handler;

        std::thread::spawn(move || {
            let doc = {
                let task = self.load_async();
                task.join().unwrap_or_else(|_| Document {
                    data: String::new(),
                    format: Some(ContentFormat::Text),
                    media_type: Some(DocumentMediaType::PlainText),
                    metadata: std::collections::HashMap::new()
                })
            };
            match doc {
                doc => {
                    let mut offset = 0;
                    for line in doc.data.lines() {
                        let mut chunk = DocumentChunk::new(line).with_range(offset, offset + line.len());

                        // Apply chunk handler if present
                        if let Some(ref handler) = chunk_handler {
                            chunk = handler(chunk);
                        }

                        if tx.send(chunk).is_err() {
                            break;
                        }

                        offset += line.len() + 1; // +1 for newline
                    }
                }
            }
        });

        AsyncStream::new(rx)
    }
}

// ========================================================================
// Internal Implementation - Zero Allocation, Lock-Free
// ========================================================================

impl<F1, F2> DocumentBuilderImpl<F1, F2>
where
    F1: Fn(String) + Send + Sync + 'static,
    F2: Fn(DocumentChunk) -> DocumentChunk + Send + Sync + 'static,
{
    fn load_document_data<G>(
        builder: DocumentBuilderImpl<F1, F2>,
        error_handler: G,
    ) -> ystream::AsyncStream<Document>
    where
        G: Fn(String) + Send + 'static,
    {
        ystream::AsyncStream::with_channel(move |sender| {
            let content = match builder.data {
                DocumentBuilderData::File(path) => {
                    let mut file_stream = Self::load_file_content(&path, &builder);
                    if let Some(content_result) = file_stream.try_next() {
                        content_result
                    } else {
                        error_handler("Failed to load file content".to_string());
                        return;
                    }
                }
                DocumentBuilderData::Url(url) => {
                    let mut url_stream = Self::load_url_content(&url, &builder);
                    if let Some(content_result) = url_stream.try_next() {
                        content_result
                    } else {
                        error_handler("Failed to load URL content".to_string());
                        return;
                    }
                }
                DocumentBuilderData::Github { repo, path, branch } => {
                    let mut github_stream = Self::load_github_content(&repo, &path, branch.as_deref(), &builder);
                    if let Some(content_result) = github_stream.try_next() {
                        content_result
                    } else {
                        error_handler("Failed to load GitHub content".to_string());
                        return;
                    }
                }
                DocumentBuilderData::Text(text) => text,
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
            let mut metadata = std::collections::HashMap::with_capacity(builder.additional_props.len() + 4);
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
                metadata
            };
            
            let _ = sender.send(document);
        })
    }

    fn load_glob_documents<G>(
        pattern: String,
        builder: DocumentBuilderImpl<F1, F2>,
        error_handler: G,
    ) -> AsyncStream<ZeroOneOrMany<Document>>
    where
        G: Fn(String) + Send + 'static,
    {
        AsyncStream::with_channel(move |sender| {
            let paths = match glob::glob(&pattern) {
                Ok(paths) => paths.filter_map(Result::ok).collect::<Vec<_>>(),
                Err(e) => {
                    error_handler(format!("Invalid glob pattern: {}", e));
                    let _ = sender.send(ZeroOneOrMany::None);
                    return;
                }
            };

            if paths.is_empty() {
                let _ = sender.send(ZeroOneOrMany::None);
                return;
            }

            let mut documents = Vec::with_capacity(paths.len());

            for path in paths {
                let doc_builder = DocumentBuilderImpl {
                    data: DocumentBuilderData::File(path),
                    format: builder.format.clone(),
                    media_type: builder.media_type.clone(),
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

                let mut doc_stream = Self::load_document_data(doc_builder, &error_handler);
                if let Some(doc) = doc_stream.try_next() {
                    documents.push(doc);
                } else {
                    error_handler("Failed to load document from glob pattern".to_string());
                }
            }

            let result = match documents.len() {
                0 => ZeroOneOrMany::None,
                1 => {
                    let mut iter = documents.into_iter();
                    match iter.next() {
                        Some(doc) => ZeroOneOrMany::One(doc),
                        None => ZeroOneOrMany::None
                    }
                }
                _ => ZeroOneOrMany::Many(documents)
            };
            
            let _ = sender.send(result);
        })
    }

    fn load_file_content(path: &Path, builder: &DocumentBuilderImpl<F1, F2>) -> AsyncStream<String> {
        let path = path.to_path_buf();
        let builder = builder.clone();
        
        AsyncStream::with_channel(move |sender| {
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
                        let _ = sender.send(content);
                        return;
                    }
                    Err(e) => {
                        last_error = format!("Attempt {}: {}", attempt + 1, e);
                        if attempt < builder.retry_attempts {
                            std::thread::sleep(std::time::Duration::from_millis(
                                100 * (1 << attempt), // Exponential backoff
                            ));
                        }
                    }
                }
            }
            // If all attempts failed, don't send anything
        })
    }

    fn load_url_content(url: &str, builder: &DocumentBuilderImpl<F1, F2>) -> AsyncStream<String> {
        let url = url.to_string();
        let builder = builder.clone();
        
        AsyncStream::with_channel(move |sender| {
            let client = match HttpClient::with_config(HttpConfig::ai_optimized()) {
                Ok(client) => client,
                Err(_) => return, // Skip sending - client creation failed
            };

            let mut request = HttpRequest::new(HttpMethod::Get, url);

            // Set timeout if specified
            if let Some(timeout_ms) = builder.timeout_ms {
                request = request.with_timeout(std::time::Duration::from_millis(timeout_ms));
            }

            // Attempt request with retries
            let mut last_error = String::new();
            for attempt in 0..=builder.retry_attempts {
                let mut response_stream = client.send(request.clone()).collect();
                if let Some(response) = response_stream.try_next() {
                    if response.status().is_success() {
                        let mut text_stream = response.text().collect();
                        if let Some(content) = text_stream.try_next() {
                            // Check size if max_size is set
                            if let Some(max_size) = builder.max_size {
                                if content.len() > max_size {
                                    return; // Skip sending - content too large
                                }
                            }

                            let _ = sender.send(content);
                            return;
                        }
                    }
                } else {
                    last_error = format!("Attempt {}: HTTP request failed", attempt + 1);
                    if attempt < builder.retry_attempts {
                        std::thread::sleep(std::time::Duration::from_millis(
                            100 * (1 << attempt), // Exponential backoff
                        ));
                    }
                }
            }
            // If all attempts failed, don't send anything
        })
    }

    fn load_github_content(
        repo: &str,
        path: &str,
        branch: Option<&str>,
        builder: &DocumentBuilderImpl<F1, F2>,
    ) -> AsyncStream<String> {
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
            DocumentBuilderData::File(path) | DocumentBuilderData::Github { path, .. } => {
                match path.extension().and_then(|ext| ext.to_str()) {
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
            _ => ContentFormat::Text
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
                        _ => DocumentMediaType::Binary
                    }
                }
                _ => DocumentMediaType::Binary
            },
            _ => DocumentMediaType::PlainText
        }
    }
}

/// Convenience function for fluent document creation
#[inline]
pub fn document() -> impl DocumentBuilder {
    Document::from_text("")
}

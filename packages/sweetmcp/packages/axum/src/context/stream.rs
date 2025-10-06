use futures::Stream;
use rpc_router::HandlerResult;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use log::{debug, error};

// Import types from sibling modules
use super::rpc::{ContextItem, GetContextRequest, ContextContent};

// Stream type for context items (optional streaming API)
#[derive(Debug)] // Added Debug derive
pub struct ContextItemStream {
    inner: ReceiverStream<HandlerResult<ContextItem>>,
}

impl ContextItemStream {
    pub(crate) fn new(rx: mpsc::Receiver<HandlerResult<ContextItem>>) -> Self {
        Self {
            inner: ReceiverStream::new(rx),
        }
    }
}

impl Stream for ContextItemStream {
    type Item = HandlerResult<ContextItem>;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.inner).poll_next(cx)
    }
}

/// Search context store and stream results
/// 
/// This function mirrors the logic from context_get() in rpc.rs but streams
/// results through an mpsc channel instead of collecting into a Vec.
async fn search_and_stream_contexts(
    request: GetContextRequest,
    tx: mpsc::Sender<HandlerResult<ContextItem>>,
) -> Result<(), anyhow::Error> {
    // Access global APPLICATION_CONTEXT (same pattern as rpc.rs)
    let app_lock = crate::context::APPLICATION_CONTEXT.read().await;
    
    if let Some(app_context) = app_lock.as_ref() {
        let memory_adapter = app_context.memory_adapter();
        
        // Search using semantic search with BERT embeddings
        let max_results = request.max_results.map(|r| r as usize);
        match memory_adapter.search_contexts(&request.query, max_results).await {
            Ok(search_results) => {
                debug!(
                    "Streaming {} context results for query: {}", 
                    search_results.len(), 
                    request.query
                );
                
                // Stream each result as ContextItem
                for (key, value) in search_results {
                    let item = ContextItem {
                        id: key.clone(),
                        source: "memory".to_string(),
                        title: Some(key),
                        content: ContextContent {
                            type_: "text".to_string(),
                            text: Some(value.to_string()),
                            data: None,
                            mime_type: Some("application/json".to_string()),
                        },
                        metadata: Some(value),
                        relevance: None,
                    };
                    
                    // Send with backpressure handling
                    if tx.send(Ok(item)).await.is_err() {
                        debug!("Context stream receiver dropped, stopping search");
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Context search failed: {}", e);
                // Send error to stream
                let _ = tx.send(Err(e.to_string().into())).await;
            }
        }
    } else {
        error!("APPLICATION_CONTEXT not initialized");
        let _ = tx.send(Err("Application context not initialized".into())).await;
    }
    
    Ok(())
}

// Streaming version of context_get_context (optional)
pub fn context_get_context_stream(request: GetContextRequest) -> ContextItemStream {
    let (tx, rx) = mpsc::channel(16);
    
    // Spawn async task to search and stream results
    tokio::spawn(async move {
        if let Err(e) = search_and_stream_contexts(request, tx).await {
            error!("Context stream task failed: {}", e);
        }
    });
    
    ContextItemStream::new(rx)
}

//! Simple message-based chat without full orchestration

use super::super::*;
use super::chat_orchestration;
use std::pin::Pin;
use tokio_stream::Stream;

pub(super) fn execute_simple_chat(
    builder: CandleAgentBuilderImpl,
    message: String,
) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
    match chat_orchestration::execute_chat(builder, move |_| {
        let msg = message.clone();
        async move { CandleChatLoop::UserPrompt(msg) }
    }) {
        Ok(stream) => stream,
        Err(_) => Box::pin(crate::async_stream::spawn_stream(|sender| async move {
            let _ = sender.send(CandleMessageChunk::Error("Chat failed".to_string()));
        })),
    }
}

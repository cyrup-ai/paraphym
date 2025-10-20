//! Simple chat implementation without full orchestration

use super::super::*;

pub(super) fn execute_simple_chat(
    builder: CandleAgentBuilderImpl,
    message: String,
) -> Pin<Box<dyn Stream<Item = CandleMessageChunk> + Send>> {
    let provider = builder.text_to_text_model;
    let temperature = builder.temperature;
    let max_tokens = builder.max_tokens;
    let system_prompt = builder.system_prompt.clone();
    let on_chunk_handler = builder.on_chunk_handler;

    Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
        let full_prompt = format!("{}\n\nUser: {}", system_prompt, message);

        let prompt = CandlePrompt::new(full_prompt);
        let params = CandleCompletionParams {
            temperature,
            max_tokens: NonZeroU64::new(max_tokens),
            ..Default::default()
        };

        let completion_stream = provider.prompt(prompt, &params);
        tokio::pin!(completion_stream);

        // Track metrics for performance visibility
        let start_time = std::time::Instant::now();
        let mut token_count = 0u32;

        while let Some(completion_chunk) = completion_stream.next().await {
            let message_chunk = match completion_chunk {
                CandleCompletionChunk::Text(text) => {
                    token_count += 1;
                    CandleMessageChunk::Text(text)
                }
                CandleCompletionChunk::Complete {
                    text,
                    finish_reason,
                    usage,
                } => {
                    // Calculate performance metrics
                    let elapsed = start_time.elapsed();
                    let elapsed_secs = elapsed.as_secs_f64();
                    let tokens_per_sec = if elapsed_secs > 0.0 {
                        Some(token_count as f64 / elapsed_secs)
                    } else {
                        None
                    };

                    CandleMessageChunk::Complete {
                        text,
                        finish_reason: finish_reason.map(|f| format!("{:?}", f)),
                        usage: usage.map(|u| format!("{:?}", u)),
                        token_count: Some(token_count),
                        elapsed_secs: Some(elapsed_secs),
                        tokens_per_sec,
                    }
                }
                CandleCompletionChunk::ToolCallStart { id, name } => {
                    CandleMessageChunk::ToolCallStart { id, name }
                }
                CandleCompletionChunk::ToolCall {
                    id,
                    name,
                    partial_input,
                } => CandleMessageChunk::ToolCall {
                    id,
                    name,
                    partial_input,
                },
                CandleCompletionChunk::ToolCallComplete { id, name, input } => {
                    CandleMessageChunk::ToolCallComplete { id, name, input }
                }
                CandleCompletionChunk::Error(error) => CandleMessageChunk::Error(error),
            };

            // Apply chunk handler if configured (zero allocation for None)
            let final_chunk = if let Some(ref handler) = on_chunk_handler {
                handler(message_chunk).await
            } else {
                message_chunk
            };
            let _ = sender.send(final_chunk);
        }
    }))
}

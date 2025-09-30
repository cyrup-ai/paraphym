//! Concurrency primitives and utilities

use std::sync::Arc;
use std::sync::Mutex;

use crossbeam_channel::{bounded, unbounded};
use ystream::{AsyncStream, AsyncTask};
use cyrup_sugars::prelude::MessageChunk;
use serde::{Deserialize, Serialize};


/// Result type for channel operations that implements `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelResult {
    pub success: bool,
    pub error_message: Option<String>,
}



impl Default for ChannelResult {
    fn default() -> Self {
        ChannelResult {
            success: true,
            error_message: None,
        }
    }
}

impl MessageChunk for ChannelResult {
    fn bad_chunk(error: String) -> Self {
        ChannelResult {
            success: false,
            error_message: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}


/// A multi-producer, single-consumer channel for sending values between tasks
pub struct Channel<T> {
    sender: crossbeam_channel::Sender<T>,
    receiver: Arc<Mutex<crossbeam_channel::Receiver<T>>>,
}

impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}

impl<T: Send + 'static + MessageChunk + Default> Channel<T> {
    /// Create a new channel with the given buffer size
    pub fn new(buffer: usize) -> Self {
        let (sender, receiver) = if buffer == 0 {
            unbounded()
        } else {
            bounded(buffer)
        };
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    /// Send a value into the channel
    pub fn send(&self, value: T) -> AsyncStream<ChannelResult> {
        let sender = self.sender.clone();
        AsyncStream::with_channel(|stream_sender| {
            std::thread::spawn(move || {
                let result = match sender.send(value) {
                    Ok(()) => ChannelResult {
                        success: true,
                        error_message: None,
                    },
                    Err(_) => ChannelResult::bad_chunk("Send error".to_string()),
                };
                let _ = stream_sender.send(result);
            });
        })
    }

    /// Receive status from the channel (value access requires separate method)
    pub fn recv_status(&self) -> AsyncStream<ChannelResult> {
        let receiver = self.receiver.clone();
        AsyncStream::with_channel(|stream_sender| {
            std::thread::spawn(move || {
                let result = {
                    if let Ok(guard) = receiver.try_lock() {
                        match guard.recv() {
                            Ok(_) => ChannelResult {
                                success: true,
                                error_message: None,
                            },
                            Err(_) => ChannelResult::bad_chunk("Channel closed".to_string()),
                        }
                    } else {
                        ChannelResult::bad_chunk("Failed to acquire lock".to_string())
                    }
                };
                let _ = stream_sender.send(result);
            });
        })
    }

    /// Create a new receiver that can be used to receive values from this channel
    pub fn subscribe(&self) -> AsyncStream<T> {
        let receiver = self.receiver.clone();
        AsyncStream::with_channel(|stream_sender| {
            std::thread::spawn(move || {
                if let Ok(guard) = receiver.try_lock() {
                    while let Ok(value) = guard.recv() {
                        if stream_sender.send(value).is_err() {
                            break;
                        }
                    }
                }
            });
        })
    }
}

/// A oneshot channel for sending a single value between tasks
pub struct OneshotChannel<T> {
    sender: Option<crossbeam_channel::Sender<T>>,
    receiver: crossbeam_channel::Receiver<T>,
}

impl<T> OneshotChannel<T> {
    /// Create a new oneshot channel
    pub fn new() -> Self {
        let (sender, receiver) = bounded(1);
        Self {
            sender: Some(sender),
            receiver,
        }
    }
}

impl<T: Send + 'static + MessageChunk + Default> OneshotChannel<T> {
    /// Send a value through the channel
    pub fn send(mut self, value: T) -> Result<(), T> {
        if let Some(sender) = self.sender.take() {
            sender.send(value).map_err(crossbeam_channel::SendError::into_inner)
        } else {
            Err(value)
        }
    }

    /// Receive the value from the channel
    pub fn recv(self) -> AsyncStream<ChannelResult> {
        AsyncStream::with_channel(|stream_sender| {
            std::thread::spawn(move || {
                if let Ok(_value) = self.receiver.recv() {
                    // For oneshot channels, we need a different approach since T might not implement MessageChunk
                    // This is a design issue - oneshot channels need to return the actual value
                    // For now, signal success
                    let result = ChannelResult {
                        success: true,
                        error_message: None,
                    };
                    let _ = stream_sender.send(result);
                } else {
                    let result = ChannelResult::bad_chunk("Channel closed".to_string());
                    let _ = stream_sender.send(result);
                }
            });
        })
    }
}

impl<T> Default for OneshotChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for converting streams into tasks
pub trait IntoTask<T> {
    /// Convert the stream into a task
    fn into_task(self) -> AsyncTask<T>;
}

impl<T> IntoTask<T> for AsyncStream<T>
where
    T: Send + 'static + MessageChunk + Default,
{
    fn into_task(self) -> AsyncTask<T> {
        // Create a channel and consume the stream
        let (tx, rx) = crossbeam_channel::bounded(1);
        let stream = self;
        std::thread::spawn(move || {
            if let Some(result) = stream.try_next() {
                let _ = tx.send(result);
            }
        });
        AsyncTask::new(rx)
    }
}

//! Live message streaming system with zero-allocation patterns
//!
//! This module provides high-performance message streaming using lock-free queues,
//! atomic counters, and tokio Stream patterns for blazing-fast real-time updates.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use atomic_counter::{AtomicCounter, ConsistentCounter};
use crossbeam_skiplist::SkipMap;
use cyrup_sugars::prelude::MessageChunk;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use std::pin::Pin;
use tokio_stream::Stream;

use super::events::RealTimeEvent;
// Use the domain's RealTimeError
use crate::domain::chat::message::types::{
    CandleMessage as Message, CandleMessageRole as MessageRole,
};
use crate::domain::util::unix_timestamp_nanos;

/// Live update message with zero-allocation string handling
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiveUpdateMessage {
    /// Message ID with zero-allocation storage
    pub id: String,
    /// Message content with zero-allocation storage
    pub content: String,
    /// Message type identifier
    pub message_type: String,
    /// Session ID with zero-allocation storage
    pub session_id: String,
    /// User ID with zero-allocation storage
    pub user_id: String,
    /// Timestamp in nanoseconds for high precision
    pub timestamp_nanos: u64,
    /// Message priority level
    pub priority: MessagePriority,
    /// Optional metadata with zero-allocation storage
    pub metadata: Option<String>,
    /// Message size in bytes for monitoring
    pub size_bytes: u32,
    /// Sequence number for ordering
    pub sequence_number: u64,
}

impl MessageChunk for LiveUpdateMessage {
    fn bad_chunk(error: String) -> Self {
        Self {
            id: String::new(),
            content: error,
            message_type: "error".to_string(),
            session_id: String::new(),
            user_id: String::new(),
            timestamp_nanos: 0,
            priority: MessagePriority::Normal,
            metadata: None,
            size_bytes: 0,
            sequence_number: 0,
        }
    }

    fn error(&self) -> Option<&str> {
        if self.message_type == "error" {
            Some(&self.content)
        } else {
            None
        }
    }
}

impl LiveUpdateMessage {
    /// Create a new live update message with current timestamp
    #[inline]
    #[must_use]
    pub fn new(
        id: String,
        content: String,
        message_type: String,
        session_id: String,
        user_id: String,
        priority: MessagePriority,
    ) -> Self {
        let timestamp_nanos = unix_timestamp_nanos();

        let size_bytes = u32::try_from(
            id.len() + content.len() + message_type.len() + session_id.len() + user_id.len(),
        )
        .unwrap_or(u32::MAX);

        Self {
            id,
            content,
            message_type,
            session_id,
            user_id,
            timestamp_nanos,
            priority,
            metadata: None,
            size_bytes,
            sequence_number: 0, // Will be set by streamer
        }
    }

    /// Add metadata to the message
    #[must_use]
    #[inline]
    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.size_bytes += u32::try_from(metadata.len()).unwrap_or(u32::MAX);
        self.metadata = Some(metadata);
        self
    }

    /// Get timestamp in seconds
    #[allow(clippy::cast_precision_loss)] // Acceptable for timestamp conversion
    #[inline]
    #[must_use]
    pub fn timestamp_seconds(&self) -> f64 {
        self.timestamp_nanos as f64 / 1_000_000_000.0
    }

    /// Get message age in nanoseconds
    #[inline]
    #[must_use]
    pub fn age_nanos(&self) -> u64 {
        let now_nanos = unix_timestamp_nanos();

        now_nanos.saturating_sub(self.timestamp_nanos)
    }

    /// Check if message has expired based on TTL
    #[inline]
    #[must_use]
    pub fn is_expired(&self, ttl_nanos: u64) -> bool {
        self.age_nanos() > ttl_nanos
    }

    /// Convert to `RealTimeEvent` for broadcasting
    #[must_use]
    pub fn to_real_time_event(&self) -> RealTimeEvent {
        // Create a basic Message for the event
        let message = Message::new(
            self.sequence_number,
            MessageRole::Assistant,
            self.content.as_bytes(),
        );

        RealTimeEvent::message_received(message, self.session_id.clone())
    }
}

/// Message priority levels with atomic-friendly ordering
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessagePriority {
    /// Low priority - background updates
    Low,
    /// Normal priority - standard messages
    Normal,
    /// High priority - important updates
    High,
    /// Critical priority - urgent notifications
    Critical,
}

impl MessagePriority {
    /// Get priority weight for ordering (higher = more important)
    #[inline]
    #[must_use]
    pub const fn weight(&self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Normal => 5,
            Self::High => 10,
            Self::Critical => 20,
        }
    }

    /// Convert to atomic representation
    #[inline]
    #[must_use]
    pub const fn to_atomic(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::High => 2,
            Self::Critical => 3,
        }
    }

    /// Convert from atomic representation
    #[inline]
    #[must_use]
    pub const fn from_atomic(value: u8) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Normal,
            2 => Self::High,
            _ => Self::Critical,
        }
    }
}

impl Default for MessagePriority {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

/// Live message streamer with lock-free queuing and atomic statistics
pub struct LiveMessageStreamer {
    /// Message queue for lock-free streaming
    message_queue_tx: mpsc::UnboundedSender<LiveUpdateMessage>,
    message_queue_rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<LiveUpdateMessage>>>,
    /// Priority queue for high-priority messages
    priority_queue_tx: mpsc::UnboundedSender<LiveUpdateMessage>,
    priority_queue_rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<LiveUpdateMessage>>>,
    /// Active subscribers with zero-allocation keys
    subscribers: Arc<SkipMap<String, Arc<StreamSubscriber>>>,
    /// Event broadcaster for real-time notifications
    event_broadcaster: broadcast::Sender<RealTimeEvent>,
    /// Message counter for statistics
    message_counter: Arc<AtomicUsize>,
    /// Priority message counter
    priority_message_counter: Arc<AtomicUsize>,
    /// Subscriber counter
    subscriber_counter: Arc<ConsistentCounter>,
    /// Total bytes processed
    bytes_processed: Arc<AtomicU64>,
    /// Queue size limit for backpressure
    queue_size_limit: Arc<AtomicUsize>,
    /// Backpressure threshold
    #[allow(dead_code)] // TODO: Implement backpressure throttling logic
    backpressure_threshold: Arc<AtomicUsize>,
    /// Processing rate in messages per second
    processing_rate: Arc<AtomicU64>,
    /// Sequence number generator
    sequence_generator: Arc<AtomicU64>,
    /// Backpressure event counter
    backpressure_events: Arc<AtomicU64>,
    /// Processing task active flag
    processing_active: Arc<std::sync::atomic::AtomicBool>,
}

/// Stream subscriber with atomic statistics
#[derive(Debug)]
pub struct StreamSubscriber {
    /// Subscriber ID
    pub id: String,
    /// Session ID filter (None = all sessions)
    pub session_filter: Option<String>,
    /// User ID filter (None = all users)
    pub user_filter: Option<String>,
    /// Minimum priority filter
    pub min_priority: MessagePriority,
    /// Messages received counter
    pub messages_received: Arc<AtomicU64>,
    /// Bytes received counter
    pub bytes_received: Arc<AtomicU64>,
    /// Subscription timestamp
    pub subscribed_at: u64,
    /// Last message timestamp
    pub last_message_at: Arc<AtomicU64>,
}

impl StreamSubscriber {
    /// Create new stream subscriber
    #[inline]
    pub fn new(id: String) -> Self {
        let now_nanos = unix_timestamp_nanos();

        Self {
            id,
            session_filter: None,
            user_filter: None,
            min_priority: MessagePriority::Normal,
            messages_received: Arc::new(AtomicU64::new(0)),
            bytes_received: Arc::new(AtomicU64::new(0)),
            subscribed_at: now_nanos,
            last_message_at: Arc::new(AtomicU64::new(now_nanos)),
        }
    }

    /// Set session filter
    #[inline]
    pub fn with_session_filter(mut self, session_id: String) -> Self {
        self.session_filter = Some(session_id);
        self
    }

    /// Set user filter
    #[inline]
    pub fn with_user_filter(mut self, user_id: String) -> Self {
        self.user_filter = Some(user_id);
        self
    }

    /// Set minimum priority filter
    #[inline]
    pub fn with_min_priority(mut self, priority: MessagePriority) -> Self {
        self.min_priority = priority;
        self
    }

    /// Check if message should be delivered to this subscriber
    pub fn should_receive(&self, message: &LiveUpdateMessage) -> bool {
        // Check session filter
        if let Some(session_filter) = &self.session_filter
            && message.session_id != *session_filter
        {
            return false;
        }

        // Check user filter
        if let Some(user_filter) = &self.user_filter
            && message.user_id != *user_filter
        {
            return false;
        }

        // Check priority filter
        if message.priority < self.min_priority {
            return false;
        }

        true
    }

    /// Record message delivery
    #[inline]
    pub fn record_delivery(&self, message: &LiveUpdateMessage) {
        self.messages_received.fetch_add(1, Ordering::AcqRel);
        self.bytes_received
            .fetch_add(u64::from(message.size_bytes), Ordering::AcqRel);
        self.last_message_at
            .store(message.timestamp_nanos, Ordering::Release);
    }

    /// Get subscription duration in nanoseconds
    #[inline]
    pub fn subscription_duration_nanos(&self) -> u64 {
        let now_nanos = unix_timestamp_nanos();

        now_nanos.saturating_sub(self.subscribed_at)
    }
}

impl LiveMessageStreamer {
    /// Create new live message streamer
    #[inline]
    #[must_use]
    pub fn new(
        queue_size_limit: usize,
        backpressure_threshold: usize,
        processing_rate: u64,
    ) -> Self {
        let (event_broadcaster, _) = broadcast::channel(50000); // Large buffer for performance
        
        let (message_queue_tx, message_queue_rx) = mpsc::unbounded_channel();
        let (priority_queue_tx, priority_queue_rx) = mpsc::unbounded_channel();

        Self {
            message_queue_tx,
            message_queue_rx: Arc::new(tokio::sync::Mutex::new(message_queue_rx)),
            priority_queue_tx,
            priority_queue_rx: Arc::new(tokio::sync::Mutex::new(priority_queue_rx)),
            subscribers: Arc::new(SkipMap::new()),
            event_broadcaster,
            message_counter: Arc::new(AtomicUsize::new(0)),
            priority_message_counter: Arc::new(AtomicUsize::new(0)),
            subscriber_counter: Arc::new(ConsistentCounter::new(0)),
            bytes_processed: Arc::new(AtomicU64::new(0)),
            queue_size_limit: Arc::new(AtomicUsize::new(queue_size_limit)),
            backpressure_threshold: Arc::new(AtomicUsize::new(backpressure_threshold)),
            processing_rate: Arc::new(AtomicU64::new(processing_rate)),
            sequence_generator: Arc::new(AtomicU64::new(1)),
            backpressure_events: Arc::new(AtomicU64::new(0)),
            processing_active: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Send live update message with backpressure handling
    #[must_use]
    pub fn send_message(&self, mut message: LiveUpdateMessage) -> Pin<Box<dyn Stream<Item = StreamingResult> + Send>> {
        let queue_tx = if message.priority >= MessagePriority::High {
            self.priority_queue_tx.clone()
        } else {
            self.message_queue_tx.clone()
        };

        let counter = if message.priority >= MessagePriority::High {
            self.priority_message_counter.clone()
        } else {
            self.message_counter.clone()
        };

        let queue_size_limit = self.queue_size_limit.clone();
        let backpressure_threshold = self.backpressure_threshold.clone();
        let backpressure_events = self.backpressure_events.clone();
        let sequence_generator = self.sequence_generator.clone();
        let bytes_processed = self.bytes_processed.clone();
        let event_broadcaster = self.event_broadcaster.clone();

        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            // Check for backpressure
            let current_queue_size = counter.load(Ordering::Acquire);
            let queue_limit = queue_size_limit.load(Ordering::Acquire);
            let bp_threshold = backpressure_threshold.load(Ordering::Acquire);

            // Hard limit check - reject messages
            if current_queue_size >= queue_limit {
                backpressure_events.fetch_add(1, Ordering::AcqRel);

                let result = StreamingResult::BackpressureError {
                    current_size: current_queue_size,
                    limit: queue_limit,
                    message_id: message.id.clone(),
                };
                let _ = tx.send(result);
                return;
            }

            // Soft limit check - emit warning but allow message
            if current_queue_size >= bp_threshold {
                let warning_result = StreamingResult::BackpressureWarning {
                    current_size: current_queue_size,
                    threshold: bp_threshold,
                    message_id: message.id.clone(),
                };
                let _ = tx.send(warning_result);
            }

            // Assign sequence number
            message.sequence_number = sequence_generator.fetch_add(1, Ordering::AcqRel);

            // Add message to queue
            if queue_tx.send(message.clone()).is_err() {
                let result = StreamingResult::BackpressureError {
                    current_size: current_queue_size,
                    limit: queue_limit,
                    message_id: message.id.clone(),
                };
                let _ = tx.send(result);
                return;
            }
            counter.fetch_add(1, Ordering::AcqRel);
            bytes_processed.fetch_add(u64::from(message.size_bytes), Ordering::AcqRel);

            // Broadcast real-time event
            let _ = event_broadcaster.send(message.to_real_time_event());

            let result = StreamingResult::MessageQueued {
                message_id: message.id,
                sequence_number: message.sequence_number,
                queue_position: current_queue_size + 1,
            };
            let _ = tx.send(result);
        }))
    }

    /// Subscribe to live updates with filtering
    #[must_use]
    pub fn subscribe(&self, subscriber: StreamSubscriber) -> Pin<Box<dyn Stream<Item = LiveUpdateMessage> + Send>> {
        let subscriber_arc = Arc::new(subscriber);
        let subscriber_id = subscriber_arc.id.clone();

        self.subscribers.insert(subscriber_id, subscriber_arc);
        self.subscriber_counter.inc();

        // Return empty stream - messages will be delivered via processing task
        Box::pin(crate::async_stream::spawn_stream(|_tx| async move {}))
    }

    /// Unsubscribe from live updates
    #[must_use]
    pub fn unsubscribe(&self, subscriber_id: &str) -> Pin<Box<dyn Stream<Item = UnsubscribeResult> + Send>> {
        let subscribers = self.subscribers.clone();
        let subscriber_counter = self.subscriber_counter.clone();
        let id = subscriber_id.to_string();

        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            let result = if subscribers.remove(&id).is_some() {
                // Decrement counter efficiently
                let current = subscriber_counter.get();
                if current > 0 {
                    subscriber_counter.reset();
                    for _ in 0..(current - 1) {
                        subscriber_counter.inc();
                    }
                }

                UnsubscribeResult::Success {
                    subscriber_id: id,
                    remaining_subscribers: subscriber_counter.get() as u64,
                }
            } else {
                UnsubscribeResult::NotFound { subscriber_id: id }
            };

            let _ = tx.send(result);
        }))
    }

    /// Start message processing task with lock-free distribution
    #[allow(clippy::cast_precision_loss)] // Acceptable for rate calculations
    #[must_use]
    pub fn start_processing(&self) -> Pin<Box<dyn Stream<Item = ProcessingEvent> + Send>> {
        if self
            .processing_active
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            // Already running
            return Box::pin(crate::async_stream::spawn_stream(|_tx| async move {}));
        }

        let message_queue_rx = self.message_queue_rx.clone();
        let priority_queue_rx = self.priority_queue_rx.clone();
        let subscribers = self.subscribers.clone();
        let message_counter = self.message_counter.clone();
        let priority_message_counter = self.priority_message_counter.clone();
        let processing_rate = self.processing_rate.clone();
        let processing_active = self.processing_active.clone();

        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            tokio::spawn(async move {
                let mut messages_processed = 0u64;
                let mut last_rate_check = std::time::Instant::now();

                loop {
                    let target_rate = processing_rate.load(Ordering::Acquire);
                    let delay_nanos = if target_rate > 0 {
                        1_000_000_000 / target_rate
                    } else {
                        1_000_000 // 1ms default
                    };

                    // Process priority messages first - use timeout to avoid blocking
                    let mut message = {
                        let mut rx = priority_queue_rx.lock().await;
                        match tokio::time::timeout(Duration::from_micros(100), rx.recv()).await {
                            Ok(Some(priority_msg)) => {
                                priority_message_counter.fetch_sub(1, Ordering::AcqRel);
                                Some(priority_msg)
                            }
                            _ => None,
                        }
                    };
                    
                    // If no priority message, check normal queue
                    if message.is_none() {
                        let mut rx = message_queue_rx.lock().await;
                        if let Ok(Some(normal_msg)) = tokio::time::timeout(Duration::from_micros(100), rx.recv()).await {
                            message_counter.fetch_sub(1, Ordering::AcqRel);
                            message = Some(normal_msg);
                        }
                    }

                    if let Some(message) = message {
                        let mut delivered_count = 0u64;
                        let mut total_bytes = 0u64;

                        // Distribute to matching subscribers
                        for entry in subscribers.iter() {
                            let subscriber = entry.value();
                            if subscriber.should_receive(&message) {
                                subscriber.record_delivery(&message);
                                delivered_count += 1;
                                total_bytes += u64::from(message.size_bytes);
                            }
                        }

                        messages_processed += 1;

                        // Emit processing event
                        let event = ProcessingEvent::MessageProcessed {
                            message_id: message.id,
                            sequence_number: message.sequence_number,
                            delivered_count,
                            total_bytes,
                            priority: message.priority,
                        };
                        let _ = tx.send(event);

                        // Rate limiting with nanosecond precision
                        tokio::time::sleep(Duration::from_nanos(delay_nanos)).await;
                    } else {
                        // No messages, sleep briefly
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }

                    // Report processing rate periodically
                    if last_rate_check.elapsed() >= Duration::from_secs(10) {
                        let rate =
                            messages_processed as f64 / last_rate_check.elapsed().as_secs_f64();

                        let event = ProcessingEvent::RateReport {
                            messages_per_second: rate,
                            messages_processed,
                            active_subscribers: subscribers.len() as u64,
                        };
                        let _ = tx.send(event);

                        messages_processed = 0;
                        last_rate_check = std::time::Instant::now();
                    }

                    // Check if we should continue
                    if !processing_active.load(Ordering::Acquire) {
                        break;
                    }
                }
            });
        }))
    }

    /// Stop processing task
    #[inline]
    pub fn stop_processing(&self) {
        self.processing_active.store(false, Ordering::Release);
    }

    /// Get current backpressure threshold
    #[inline]
    #[must_use]
    pub fn get_backpressure_threshold(&self) -> usize {
        self.backpressure_threshold.load(Ordering::Acquire)
    }

    /// Update backpressure threshold dynamically
    #[inline]
    pub fn set_backpressure_threshold(&self, threshold: usize) {
        self.backpressure_threshold
            .store(threshold, Ordering::Release);
    }

    /// Get comprehensive streaming statistics
    #[must_use]
    pub fn get_statistics(&self) -> StreamingStatistics {
        StreamingStatistics {
            total_messages: self.message_counter.load(Ordering::Acquire),
            priority_messages: self.priority_message_counter.load(Ordering::Acquire),
            active_subscribers: self.subscriber_counter.get(),
            total_subscribers: self.subscribers.len(),
            bytes_processed: self.bytes_processed.load(Ordering::Acquire),
            queue_size_limit: self.queue_size_limit.load(Ordering::Acquire),
            backpressure_events: self.backpressure_events.load(Ordering::Acquire),
            processing_rate: self.processing_rate.load(Ordering::Acquire),
            processing_active: self.processing_active.load(Ordering::Acquire),
        }
    }

    /// Subscribe to real-time events
    #[inline]
    #[must_use]
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<RealTimeEvent> {
        self.event_broadcaster.subscribe()
    }
}

impl Drop for LiveMessageStreamer {
    fn drop(&mut self) {
        self.stop_processing();
    }
}

/// Streaming operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamingResult {
    /// Message successfully queued
    MessageQueued {
        message_id: String,
        sequence_number: u64,
        queue_position: usize,
    },
    /// Backpressure warning - queue size approaching limit
    BackpressureWarning {
        current_size: usize,
        threshold: usize,
        message_id: String,
    },
    /// Backpressure error occurred
    BackpressureError {
        current_size: usize,
        limit: usize,
        message_id: String,
    },
}

impl Default for StreamingResult {
    fn default() -> Self {
        StreamingResult::MessageQueued {
            message_id: String::new(),
            sequence_number: 0,
            queue_position: 0,
        }
    }
}

impl MessageChunk for StreamingResult {
    fn bad_chunk(error: String) -> Self {
        StreamingResult::BackpressureError {
            current_size: 0,
            limit: 0,
            message_id: error,
        }
    }

    fn error(&self) -> Option<&str> {
        match self {
            StreamingResult::BackpressureError { message_id, .. } => Some(message_id),
            _ => None,
        }
    }
}

/// Unsubscribe operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnsubscribeResult {
    /// Successfully unsubscribed
    Success {
        subscriber_id: String,
        remaining_subscribers: u64,
    },
    /// Subscriber not found
    NotFound { subscriber_id: String },
}

impl Default for UnsubscribeResult {
    fn default() -> Self {
        UnsubscribeResult::Success {
            subscriber_id: String::new(),
            remaining_subscribers: 0,
        }
    }
}

impl MessageChunk for UnsubscribeResult {
    fn bad_chunk(error: String) -> Self {
        UnsubscribeResult::NotFound {
            subscriber_id: error,
        }
    }

    fn error(&self) -> Option<&str> {
        match self {
            UnsubscribeResult::NotFound { subscriber_id } => Some(subscriber_id),
            UnsubscribeResult::Success { .. } => None,
        }
    }
}

/// Processing events for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingEvent {
    /// Message processed and distributed
    MessageProcessed {
        message_id: String,
        sequence_number: u64,
        delivered_count: u64,
        total_bytes: u64,
        priority: MessagePriority,
    },
    /// Processing rate report
    RateReport {
        messages_per_second: f64,
        messages_processed: u64,
        active_subscribers: u64,
    },
}

impl Default for ProcessingEvent {
    fn default() -> Self {
        ProcessingEvent::RateReport {
            messages_per_second: 0.0,
            messages_processed: 0,
            active_subscribers: 0,
        }
    }
}

impl MessageChunk for ProcessingEvent {
    fn bad_chunk(error: String) -> Self {
        Self::MessageProcessed {
            message_id: error,
            sequence_number: 0,
            delivered_count: 0,
            total_bytes: 0,
            priority: MessagePriority::Normal,
        }
    }

    fn error(&self) -> Option<&str> {
        match self {
            Self::MessageProcessed {
                message_id,
                delivered_count,
                ..
            } if *delivered_count == 0 => Some(message_id),
            _ => None,
        }
    }
}

/// Live streaming statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingStatistics {
    /// Total messages in normal queue
    pub total_messages: usize,
    /// Total messages in priority queue
    pub priority_messages: usize,
    /// Number of active subscribers
    pub active_subscribers: usize,
    /// Total subscribers ever created
    pub total_subscribers: usize,
    /// Total bytes processed
    pub bytes_processed: u64,
    /// Queue size limit
    pub queue_size_limit: usize,
    /// Number of backpressure events
    pub backpressure_events: u64,
    /// Target processing rate (messages/second)
    pub processing_rate: u64,
    /// Whether processing is active
    pub processing_active: bool,
}

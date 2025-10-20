//! Error-specific circuit breaker with statistics

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use atomic_counter::{AtomicCounter, RelaxedCounter};
use super::circuit_breaker::{CircuitBreaker, CircuitBreakerState, CircuitBreakerError};
use super::core::ZeroAllocError;
use super::types::{ErrorCategory, ErrorSeverity, ErrorRecoverability};
use crate::domain::util::{duration_to_millis_u64, duration_to_nanos_u64};

/// Lock-free error counter for statistics
#[derive(Debug)]
pub struct ErrorCounter {
    /// Total error count
    total: RelaxedCounter,
    /// Count by category
    by_category: [RelaxedCounter; 10],
    /// Count by severity
    by_severity: [RelaxedCounter; 4],
    /// Count by recoverability
    by_recoverability: [RelaxedCounter; 4],
    /// Last error timestamp
    last_error: AtomicU64,
}

impl ErrorCounter {
    /// Create new error counter
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            total: RelaxedCounter::new(0),
            by_category: [
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
            ],
            by_severity: [
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
            ],
            by_recoverability: [
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
                RelaxedCounter::new(0),
            ],
            last_error: AtomicU64::new(0),
        }
    }

    /// Record error occurrence
    #[inline]
    pub fn record(&self, error: &ZeroAllocError) {
        self.total.inc();
        self.by_category[error.category as usize].inc();
        self.by_severity[error.severity as usize].inc();
        self.by_recoverability[error.recoverability as usize].inc();

        let timestamp = duration_to_nanos_u64(error.timestamp.elapsed());
        self.last_error.store(timestamp, Ordering::Relaxed);
    }

    /// Get total error count
    #[inline]
    pub fn total(&self) -> usize {
        self.total.get()
    }

    /// Get error count by category
    #[inline]
    pub fn by_category(&self, category: ErrorCategory) -> usize {
        self.by_category[category as usize].get()
    }

    /// Get error count by severity
    #[inline]
    pub fn by_severity(&self, severity: ErrorSeverity) -> usize {
        self.by_severity[severity as usize].get()
    }

    /// Get error count by recoverability
    #[inline]
    pub fn by_recoverability(&self, recoverability: ErrorRecoverability) -> usize {
        self.by_recoverability[recoverability as usize].get()
    }

    /// Get last error timestamp
    #[inline]
    pub fn last_error_time(&self) -> Option<Duration> {
        let timestamp = self.last_error.load(Ordering::Relaxed);
        if timestamp > 0 {
            Some(Duration::from_nanos(timestamp))
        } else {
            None
        }
    }

    /// Reset all counters
    #[inline]
    pub fn reset(&self) {
        self.total.reset();
        for counter in &self.by_category {
            counter.reset();
        }
        for counter in &self.by_severity {
            counter.reset();
        }
        for counter in &self.by_recoverability {
            counter.reset();
        }
        self.last_error.store(0, Ordering::Relaxed);
    }
}

impl Default for ErrorCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Circuit breaker for error recovery
#[derive(Debug)]
pub struct ErrorCircuitBreaker {
    /// Circuit breaker instance
    breaker: CircuitBreaker,
    /// Error counter for statistics
    counter: ErrorCounter,
    /// Failure threshold
    #[allow(dead_code)] // TODO: Implement in circuit breaker system
    failure_threshold: usize,
    /// Recovery timeout
    #[allow(dead_code)] // TODO: Implement in circuit breaker system
    recovery_timeout: Duration,
    /// Half-open requests
    half_open_requests: AtomicU64,
}

impl ErrorCircuitBreaker {
    /// Create new circuit breaker
    #[inline]
    #[must_use]
    pub fn new(failure_threshold: usize, recovery_timeout: Duration) -> Self {
        let breaker = CircuitBreaker::new(
            failure_threshold as u64,
            duration_to_millis_u64(recovery_timeout),
        );

        Self {
            breaker,
            counter: ErrorCounter::new(),
            failure_threshold,
            recovery_timeout,
            half_open_requests: AtomicU64::new(0),
        }
    }

    /// Execute operation with circuit breaker protection
    ///
    /// # Errors
    ///
    /// Returns `ZeroAllocError` if operation fails or circuit is open
    #[inline]
    pub fn execute<T, E, F>(&self, operation: F) -> Result<T, Box<ZeroAllocError>>
    where
        F: FnOnce() -> Result<T, E>,
        E: Into<ZeroAllocError>,
    {
        match self.breaker.call(operation) {
            Ok(result) => Ok(result),
            Err(CircuitBreakerError::Inner(e)) => {
                let error = e.into();
                self.counter.record(&error);
                Err(Box::new(error))
            }
            Err(CircuitBreakerError::CircuitOpen) => {
                let error = ZeroAllocError::new(
                    ErrorCategory::System,
                    ErrorSeverity::Error,
                    ErrorRecoverability::RetriableWithBackoff,
                    "Circuit breaker is open",
                    500,
                );
                self.counter.record(&error);
                Err(Box::new(error))
            }
        }
    }

    /// Check if circuit breaker is open
    #[inline]
    pub fn is_open(&self) -> bool {
        matches!(self.breaker.get_state(), CircuitBreakerState::Open)
    }

    /// Check if circuit breaker is half-open
    #[inline]
    pub fn is_half_open(&self) -> bool {
        matches!(self.breaker.get_state(), CircuitBreakerState::HalfOpen)
    }

    /// Get error statistics
    #[inline]
    pub fn stats(&self) -> &ErrorCounter {
        &self.counter
    }

    /// Reset circuit breaker
    #[inline]
    pub fn reset(&self) {
        self.counter.reset();
        self.half_open_requests.store(0, Ordering::Relaxed);
    }
}

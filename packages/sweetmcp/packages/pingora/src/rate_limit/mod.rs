//! Rate limiting module decomposition
//!
//! This module provides the decomposed rate limiting functionality split into
//! logical modules for better maintainability and adherence to the 300-line limit.


pub mod algorithms;
pub mod distributed;
pub mod limiter;

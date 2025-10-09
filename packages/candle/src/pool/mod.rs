pub mod core;
pub mod capabilities;

pub use core::{Pool, PoolConfig, PoolError};
pub use capabilities::{text_embedding_pool, text_to_text_pool};

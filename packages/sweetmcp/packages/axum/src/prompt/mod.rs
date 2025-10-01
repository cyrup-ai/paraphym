pub mod model;
pub mod persistence;
pub mod service;

// Re-export core service functions
pub use service::{prompts_get_handler, prompts_list_handler};

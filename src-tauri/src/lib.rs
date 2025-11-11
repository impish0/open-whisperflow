// Lib.rs - Core library definitions

pub mod audio;
pub mod commands;
pub mod config;
pub mod error;
pub mod hotkeys;
pub mod injection;
pub mod llm;
pub mod state;
pub mod transcription;
pub mod utils;

// Re-export commonly used types
pub use error::{AppError, Result};
pub use state::AppState;

use std::fmt;

/// Custom error types for the application
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Audio recording error: {0}")]
    AudioRecording(String),

    #[error("Transcription error: {0}")]
    Transcription(String),

    #[error("LLM processing error: {0}")]
    LLMProcessing(String),

    #[error("Text injection error: {0}")]
    TextInjection(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Backend not available: {0}")]
    BackendUnavailable(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Custom result type
pub type Result<T> = std::result::Result<T, AppError>;

// Helper to convert AppError to user-friendly string
impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            AppError::AudioRecording(_) => {
                "Failed to record audio. Please check your microphone permissions.".to_string()
            }
            AppError::Transcription(_) => {
                "Transcription failed. Try using cloud transcription in settings.".to_string()
            }
            AppError::LLMProcessing(_) => {
                "Text processing failed. Check your LLM configuration.".to_string()
            }
            AppError::TextInjection(_) => {
                "Failed to insert text. Try changing the injection method in settings.".to_string()
            }
            AppError::Config(_) => "Configuration error. Please check your settings.".to_string(),
            AppError::Network(_) => {
                "Network error. Check your internet connection or use local mode.".to_string()
            }
            AppError::BackendUnavailable(_) => {
                "Required backend is not available. Please configure it in settings.".to_string()
            }
            _ => "An unexpected error occurred. Please try again.".to_string(),
        }
    }
}

// Allow conversion to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.user_message()
    }
}

use serde::Serialize;

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

    #[error("Docker error: {0}")]
    Docker(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Custom result type
pub type Result<T> = std::result::Result<T, AppError>;

// Helper to convert AppError to user-friendly string
impl AppError {
    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            AppError::AudioRecording(msg) => {
                format!(
                    "Failed to record audio: {}. Please check your microphone permissions and device settings.",
                    self.clean_message(msg)
                )
            }
            AppError::Transcription(msg) => {
                format!(
                    "Transcription failed: {}. Try using cloud transcription or check Docker status.",
                    self.clean_message(msg)
                )
            }
            AppError::LLMProcessing(msg) => {
                format!(
                    "Text processing failed: {}. Check your API key or Ollama installation.",
                    self.clean_message(msg)
                )
            }
            AppError::TextInjection(msg) => {
                format!(
                    "Failed to insert text: {}. Try changing the injection method in settings.",
                    self.clean_message(msg)
                )
            }
            AppError::Config(msg) => {
                format!(
                    "Configuration error: {}. Please check your settings.",
                    self.clean_message(msg)
                )
            }
            AppError::Network(err) => {
                if err.is_timeout() {
                    "Network timeout. Check your internet connection or try again.".to_string()
                } else if err.is_connect() {
                    "Cannot connect to server. Check your internet connection or use local mode.".to_string()
                } else {
                    format!(
                        "Network error: {}. Check your internet connection or use local mode.",
                        self.clean_message(&err.to_string())
                    )
                }
            }
            AppError::BackendUnavailable(msg) => {
                format!(
                    "Backend not available: {}. Please configure it in settings.",
                    self.clean_message(msg)
                )
            }
            AppError::Docker(msg) => {
                format!(
                    "Docker error: {}. Please ensure Docker Desktop is installed and running.",
                    self.clean_message(msg)
                )
            }
            AppError::InvalidState(msg) => {
                format!("Invalid state: {}. Please try again.", self.clean_message(msg))
            }
            AppError::NotFound(msg) => {
                format!("Not found: {}", self.clean_message(msg))
            }
            AppError::Io(err) => {
                format!(
                    "File system error: {}. Check file permissions and disk space.",
                    err
                )
            }
            AppError::Json(err) => {
                format!("Data format error: {}. The configuration may be corrupted.", err)
            }
            AppError::Unknown(msg) => {
                format!(
                    "Unexpected error: {}. Please try again or report this issue.",
                    self.clean_message(msg)
                )
            }
        }
    }

    /// Clean up technical error messages for user display
    fn clean_message(&self, msg: &str) -> String {
        // Remove common technical prefixes
        let msg = msg
            .trim_start_matches("error: ")
            .trim_start_matches("Error: ")
            .trim_start_matches("failed to ")
            .trim_start_matches("Failed to ");

        // Capitalize first letter
        if let Some(first) = msg.chars().next() {
            format!("{}{}", first.to_uppercase(), &msg[first.len_utf8()..])
        } else {
            msg.to_string()
        }
    }

    /// Get a short error category for logging
    pub fn category(&self) -> &str {
        match self {
            AppError::AudioRecording(_) => "audio",
            AppError::Transcription(_) => "transcription",
            AppError::LLMProcessing(_) => "llm",
            AppError::TextInjection(_) => "injection",
            AppError::Config(_) => "config",
            AppError::Io(_) => "io",
            AppError::Network(_) => "network",
            AppError::Json(_) => "json",
            AppError::InvalidState(_) => "state",
            AppError::NotFound(_) => "not_found",
            AppError::BackendUnavailable(_) => "backend",
            AppError::Docker(_) => "docker",
            AppError::Unknown(_) => "unknown",
        }
    }

    /// Check if error is recoverable (user can fix it)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AppError::Network(_)
                | AppError::BackendUnavailable(_)
                | AppError::Docker(_)
                | AppError::Config(_)
                | AppError::InvalidState(_)
        )
    }
}

// Allow conversion to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.user_message()
    }
}

// Implement Serialize for AppError to work with Tauri IPC
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.user_message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_error_message() {
        let error = AppError::AudioRecording("device not found".to_string());
        let message = error.user_message();

        assert!(message.contains("Failed to record audio"));
        assert!(message.contains("Device not found"));
        assert!(message.contains("microphone permissions"));
    }

    #[test]
    fn test_network_error_message() {
        let error = AppError::Network(
            reqwest::Error::new(reqwest::StatusCode::UNAUTHORIZED, "Unauthorized"),
        );
        let message = error.user_message();

        assert!(message.contains("Network error") || message.contains("Cannot connect"));
    }

    #[test]
    fn test_docker_error_message() {
        let error = AppError::Docker("container not running".to_string());
        let message = error.user_message();

        assert!(message.contains("Docker error"));
        assert!(message.contains("Container not running"));
        assert!(message.contains("Docker Desktop"));
    }

    #[test]
    fn test_backend_unavailable_message() {
        let error = AppError::BackendUnavailable("faster-whisper not configured".to_string());
        let message = error.user_message();

        assert!(message.contains("Backend not available"));
        assert!(message.contains("configure it in settings"));
    }

    #[test]
    fn test_error_category() {
        assert_eq!(
            AppError::AudioRecording("test".to_string()).category(),
            "audio"
        );
        assert_eq!(
            AppError::Transcription("test".to_string()).category(),
            "transcription"
        );
        assert_eq!(
            AppError::LLMProcessing("test".to_string()).category(),
            "llm"
        );
        assert_eq!(AppError::Docker("test".to_string()).category(), "docker");
    }

    #[test]
    fn test_recoverable_errors() {
        assert!(AppError::Network(
            reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        )
        .is_recoverable());
        assert!(AppError::BackendUnavailable("test".to_string()).is_recoverable());
        assert!(AppError::Docker("test".to_string()).is_recoverable());
        assert!(AppError::Config("test".to_string()).is_recoverable());
    }

    #[test]
    fn test_non_recoverable_errors() {
        assert!(!AppError::AudioRecording("test".to_string()).is_recoverable());
        assert!(!AppError::Json(serde_json::Error::custom("test")).is_recoverable());
        assert!(!AppError::Unknown("test".to_string()).is_recoverable());
    }

    #[test]
    fn test_clean_message() {
        let error = AppError::Config("test".to_string());

        // Test prefix removal
        assert_eq!(error.clean_message("error: something failed"), "Something failed");
        assert_eq!(error.clean_message("Error: something failed"), "Something failed");
        assert_eq!(error.clean_message("failed to connect"), "Connect");
        assert_eq!(error.clean_message("Failed to connect"), "Connect");

        // Test capitalization
        assert_eq!(error.clean_message("test message"), "Test message");
    }

    #[test]
    fn test_error_to_string_conversion() {
        let error = AppError::Config("test error".to_string());
        let string: String = error.into();

        assert!(string.contains("Configuration error"));
        assert!(string.contains("Test error"));
    }
}

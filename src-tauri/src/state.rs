use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use crate::config::AppConfig;
use crate::error::Result;

/// Application state shared across all Tauri commands
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub recording_state: Arc<RwLock<RecordingState>>,
    pub audio_buffer: Arc<RwLock<Option<Vec<f32>>>>,
}

/// Current state of the recording process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RecordingState {
    Idle,
    Recording {
        started_at: u64, // Unix timestamp in milliseconds
    },
    Processing {
        stage: ProcessingStage,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessingStage {
    Transcribing,
    Rewriting,
    Injecting,
}

use serde::{Deserialize, Serialize};

impl AppState {
    /// Create new application state with default configuration
    pub fn new() -> Result<Self> {
        let config = AppConfig::load().unwrap_or_default();

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            recording_state: Arc::new(RwLock::new(RecordingState::Idle)),
            audio_buffer: Arc::new(RwLock::new(None)),
        })
    }

    /// Get current recording state
    pub async fn get_recording_state(&self) -> RecordingState {
        self.recording_state.read().await.clone()
    }

    /// Update recording state
    pub async fn set_recording_state(&self, state: RecordingState) {
        *self.recording_state.write().await = state;
    }

    /// Check if currently recording
    pub async fn is_recording(&self) -> bool {
        matches!(
            *self.recording_state.read().await,
            RecordingState::Recording { .. }
        )
    }

    /// Check if currently processing
    pub async fn is_processing(&self) -> bool {
        matches!(
            *self.recording_state.read().await,
            RecordingState::Processing { .. }
        )
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new().expect("Failed to create default AppState")
    }
}

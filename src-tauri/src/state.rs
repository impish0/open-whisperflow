use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

use crate::audio::AudioRecorder;
use crate::config::AppConfig;
use crate::error::Result;

/// Application state shared across all Tauri commands
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub recording_state: Arc<RwLock<RecordingState>>,
    /// Reserved for future use (streaming audio, real-time processing)
    #[allow(dead_code)]
    pub audio_buffer: Arc<RwLock<Option<Vec<f32>>>>,
    pub audio_recorder: Arc<Mutex<Option<AudioRecorder>>>,
}

// SAFETY: AudioRecorder contains cpal::Stream which is !Send, but we ensure single-threaded access via Mutex
// All access to audio_recorder is synchronized through the Mutex, so this is safe
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

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
            audio_recorder: Arc::new(Mutex::new(None)),
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
    /// Reserved for frontend status checking
    #[allow(dead_code)]
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

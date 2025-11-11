use async_trait::async_trait;
use std::path::Path;

use crate::config::TranscriptionConfig;
use crate::error::{AppError, Result};

/// Trait for transcription backends
#[async_trait]
pub trait TranscriptionBackend: Send + Sync {
    async fn transcribe(&self, audio_path: &Path) -> Result<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}

/// OpenAI Whisper API backend
pub struct OpenAIWhisperBackend {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl OpenAIWhisperBackend {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model,
        }
    }
}

#[async_trait]
impl TranscriptionBackend for OpenAIWhisperBackend {
    async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        log::info!("Transcribing with OpenAI Whisper: {}", audio_path.display());

        let file = tokio::fs::read(audio_path).await?;

        let form = reqwest::multipart::Form::new()
            .text("model", self.model.clone())
            .part(
                "file",
                reqwest::multipart::Part::bytes(file)
                    .file_name(
                        audio_path
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                    )
                    .mime_str("audio/wav")
                    .map_err(|e| {
                        AppError::Transcription(format!("Failed to create multipart: {}", e))
                    })?,
            );

        let response = self
            .client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::Transcription(format!("API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Transcription(format!(
                "API error: {}",
                error_text
            )));
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["text"]
            .as_str()
            .ok_or_else(|| AppError::Transcription("No text in response".to_string()))?
            .to_string();

        log::info!("Transcription complete: {} characters", text.len());
        Ok(text)
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn name(&self) -> &str {
        "OpenAI Whisper"
    }
}

/// Transcription service that manages backends
pub struct TranscriptionService {
    backend: Box<dyn TranscriptionBackend>,
}

impl TranscriptionService {
    pub fn new(config: &TranscriptionConfig) -> Result<Self> {
        let backend: Box<dyn TranscriptionBackend> = match config.backend {
            crate::config::TranscriptionBackend::OpenAI => {
                let api_key = config
                    .openai_api_key
                    .clone()
                    .ok_or_else(|| AppError::Config("OpenAI API key not configured".to_string()))?;
                Box::new(OpenAIWhisperBackend::new(api_key, config.model.clone()))
            }
            crate::config::TranscriptionBackend::FasterWhisper => {
                // TODO: Implement faster-whisper backend
                return Err(AppError::BackendUnavailable(
                    "faster-whisper backend not yet implemented".to_string(),
                ));
            }
        };

        Ok(Self { backend })
    }

    pub async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        if !self.backend.is_available().await {
            return Err(AppError::BackendUnavailable(format!(
                "{} is not available",
                self.backend.name()
            )));
        }

        self.backend.transcribe(audio_path).await
    }
}

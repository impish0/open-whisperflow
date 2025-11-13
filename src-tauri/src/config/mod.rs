use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::{AppError, Result};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub audio: AudioConfig,
    pub transcription: TranscriptionConfig,
    pub llm: LLMConfig,
    pub injection: InjectionConfig,
    pub hotkeys: HotkeyConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub bit_depth: u16,
    pub device_id: String,
    pub vad_enabled: bool,
    pub max_recording_duration_seconds: u64,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
            bit_depth: 16,
            device_id: "default".to_string(),
            vad_enabled: true,
            max_recording_duration_seconds: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionConfig {
    pub backend: TranscriptionBackend,
    pub model: String,
    pub language: Option<String>,
    pub openai_api_key: Option<String>,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        Self {
            backend: TranscriptionBackend::OpenAI,
            model: "whisper-1".to_string(),
            language: None,
            openai_api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TranscriptionBackend {
    FasterWhisper,
    OpenAI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub backend: LLMBackend,
    pub model: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub default_template: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            backend: LLMBackend::OpenAI,
            model: "gpt-4o-mini".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: None,
            default_template: "balanced".to_string(),
            temperature: 0.7,
            max_tokens: 500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LLMBackend {
    Ollama,
    OpenAI,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionConfig {
    pub method: InjectionMethod,
    pub typing_speed_ms: u64,
    pub clipboard_backup: bool,
}

impl Default for InjectionConfig {
    fn default() -> Self {
        Self {
            method: InjectionMethod::Hybrid,
            typing_speed_ms: 1,
            clipboard_backup: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InjectionMethod {
    Clipboard,
    Typing,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle_recording: String,
    pub cancel_recording: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: Theme,
    pub show_notifications: bool,
    pub minimize_to_tray: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            audio: AudioConfig::default(),
            transcription: TranscriptionConfig::default(),
            llm: LLMConfig::default(),
            injection: InjectionConfig::default(),
            hotkeys: HotkeyConfig {
                toggle_recording: "Ctrl+Shift+Space".to_string(),
                cancel_recording: "Escape".to_string(),
            },
            ui: UIConfig {
                theme: Theme::System,
                show_notifications: true,
                minimize_to_tray: true,
            },
        }
    }
}

impl AppConfig {
    /// Load configuration from disk
    pub fn load() -> Result<Self> {
        confy::load("open-whisperflow", "config")
            .map_err(|e| AppError::Config(format!("Failed to load config: {}", e)))
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<()> {
        confy::store("open-whisperflow", "config", self)
            .map_err(|e| AppError::Config(format!("Failed to save config: {}", e)))
    }

    /// Get config file path
    pub fn get_config_path() -> Result<PathBuf> {
        confy::get_configuration_file_path("open-whisperflow", "config")
            .map_err(|e| AppError::Config(format!("Failed to get config path: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = AppConfig::default();

        assert_eq!(config.audio.sample_rate, 16000);
        assert_eq!(config.audio.channels, 1);
        assert_eq!(config.audio.bit_depth, 16);
        assert_eq!(
            matches!(config.transcription.backend, TranscriptionBackend::OpenAI),
            true
        );
        assert_eq!(matches!(config.llm.backend, LLMBackend::OpenAI), true);
    }

    #[test]
    fn test_audio_config_sensible_defaults() {
        let config = AudioConfig::default();

        // Validate sensible audio settings
        assert!(config.sample_rate >= 8000 && config.sample_rate <= 48000);
        assert!(config.channels == 1 || config.channels == 2);
        assert!(config.bit_depth == 16 || config.bit_depth == 24);
        assert!(config.max_recording_duration_seconds > 0);
        assert!(config.max_recording_duration_seconds <= 3600);
    }

    #[test]
    fn test_transcription_config_defaults() {
        let config = TranscriptionConfig::default();

        assert!(!config.model.is_empty());
        assert!(config.openai_api_key.is_none());
    }

    #[test]
    fn test_llm_config_defaults() {
        let config = LLMConfig::default();

        assert!(!config.model.is_empty());
        assert!(!config.base_url.is_empty());
        assert!(config.base_url.starts_with("http"));
        assert!(config.temperature >= 0.0 && config.temperature <= 2.0);
        assert!(config.max_tokens > 0);
    }

    #[test]
    fn test_injection_config_defaults() {
        let config = InjectionConfig::default();

        assert!(config.typing_speed_ms > 0);
        assert!(config.typing_speed_ms <= 1000);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();

        // Test that config can be serialized
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());

        // Test that it can be deserialized back
        let json = serialized.unwrap();
        let deserialized: std::result::Result<AppConfig, _> = serde_json::from_str(&json);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_config_clone() {
        let config1 = AppConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.audio.sample_rate, config2.audio.sample_rate);
        assert_eq!(
            format!("{:?}", config1.transcription.backend),
            format!("{:?}", config2.transcription.backend)
        );
        assert_eq!(
            format!("{:?}", config1.llm.backend),
            format!("{:?}", config2.llm.backend)
        );
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = AppConfig::default();

        assert_eq!(config.audio.sample_rate, 16000);
        assert_eq!(config.audio.channels, 1);
        assert_eq!(config.audio.bit_depth, 16);
        assert_eq!(config.transcription.backend, TranscriptionBackend::OpenAI);
        assert_eq!(config.llm.backend, LLMBackend::OpenAI);
    }

    #[test]
    fn test_transcription_backend_values() {
        assert_eq!(TranscriptionBackend::OpenAI as i32, 0);
        assert_eq!(TranscriptionBackend::FasterWhisper as i32, 1);
    }

    #[test]
    fn test_llm_backend_values() {
        assert_eq!(LLMBackend::OpenAI as i32, 0);
        assert_eq!(LLMBackend::Ollama as i32, 1);
        assert_eq!(LLMBackend::None as i32, 2);
    }

    #[test]
    fn test_injection_method_values() {
        let clipboard = InjectionMethod::Clipboard;
        let typing = InjectionMethod::Typing;
        let hybrid = InjectionMethod::Hybrid;

        // Test that all methods are distinct
        assert_ne!(
            format!("{:?}", clipboard),
            format!("{:?}", typing)
        );
        assert_ne!(
            format!("{:?}", clipboard),
            format!("{:?}", hybrid)
        );
        assert_ne!(
            format!("{:?}", typing),
            format!("{:?}", hybrid)
        );
    }

    #[test]
    fn test_theme_values() {
        let light = Theme::Light;
        let dark = Theme::Dark;
        let system = Theme::System;

        assert_ne!(
            format!("{:?}", light),
            format!("{:?}", dark)
        );
        assert_ne!(
            format!("{:?}", light),
            format!("{:?}", system)
        );
        assert_ne!(
            format!("{:?}", dark),
            format!("{:?}", system)
        );
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
        assert!(config.language.is_none() || config.language == Some("en".to_string()));
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
        let deserialized: Result<AppConfig, _> = serde_json::from_str(&json);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_config_clone() {
        let config1 = AppConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.audio.sample_rate, config2.audio.sample_rate);
        assert_eq!(config1.transcription.backend, config2.transcription.backend);
        assert_eq!(config1.llm.backend, config2.llm.backend);
    }
}

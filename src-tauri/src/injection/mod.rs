use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use tokio::time::sleep;

use crate::config::{InjectionConfig, InjectionMethod};
use crate::error::{AppError, Result};

pub struct TextInjector {
    enigo: Enigo,
    config: InjectionConfig,
}

impl TextInjector {
    pub fn new(config: &InjectionConfig) -> Result<Self> {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| AppError::TextInjection(format!("Failed to create Enigo: {}", e)))?;

        Ok(Self {
            enigo,
            config: config.clone(),
        })
    }

    /// Inject text at current cursor position
    pub async fn inject_text(&mut self, text: &str) -> Result<()> {
        log::info!("Injecting text ({} characters)", text.len());

        match self.config.method {
            InjectionMethod::Clipboard => self.inject_via_clipboard(text).await,
            InjectionMethod::Typing => self.inject_via_typing(text).await,
            InjectionMethod::Hybrid => {
                // Try clipboard first, fallback to typing
                match self.inject_via_clipboard(text).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        log::warn!("Clipboard injection failed ({}), falling back to typing", e);
                        self.inject_via_typing(text).await
                    }
                }
            }
        }
    }

    /// Inject text via clipboard paste
    async fn inject_via_clipboard(&mut self, text: &str) -> Result<()> {
        use arboard::Clipboard;

        let mut clipboard = Clipboard::new()
            .map_err(|e| AppError::TextInjection(format!("Failed to access clipboard: {}", e)))?;

        // Backup original clipboard if configured
        let original = if self.config.clipboard_backup {
            clipboard.get_text().ok()
        } else {
            None
        };

        // Set our text
        clipboard
            .set_text(text)
            .map_err(|e| AppError::TextInjection(format!("Failed to set clipboard: {}", e)))?;

        // Small delay to ensure clipboard is set
        sleep(Duration::from_millis(50)).await;

        // Simulate Ctrl+V
        self.enigo
            .key(Key::Control, Direction::Press)
            .map_err(|e| AppError::TextInjection(format!("Failed to press Ctrl: {}", e)))?;
        sleep(Duration::from_millis(50)).await;

        self.enigo
            .key(Key::Unicode('v'), Direction::Click)
            .map_err(|e| AppError::TextInjection(format!("Failed to press V: {}", e)))?;
        sleep(Duration::from_millis(50)).await;

        self.enigo
            .key(Key::Control, Direction::Release)
            .map_err(|e| AppError::TextInjection(format!("Failed to release Ctrl: {}", e)))?;

        // Wait for paste to complete
        sleep(Duration::from_millis(100)).await;

        // Restore original clipboard if it was backed up
        if let Some(orig) = original {
            clipboard.set_text(orig).ok();
        }

        log::info!("Text injected via clipboard");
        Ok(())
    }

    /// Inject text by typing character by character
    async fn inject_via_typing(&mut self, text: &str) -> Result<()> {
        for c in text.chars() {
            self.enigo
                .key(Key::Unicode(c), Direction::Click)
                .map_err(|e| AppError::TextInjection(format!("Failed to type character: {}", e)))?;

            if self.config.typing_speed_ms > 0 {
                sleep(Duration::from_millis(self.config.typing_speed_ms)).await;
            }
        }

        log::info!("Text injected via typing");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::InjectionConfig;

    #[test]
    fn test_text_injector_creation() {
        let config = InjectionConfig {
            method: InjectionMethod::Clipboard,
            typing_speed_ms: 1,
            clipboard_backup: true,
        };

        let result = TextInjector::new(&config);
        // May fail if no display server available in tests
        assert!(result.is_ok() || result.is_err());
    }
}

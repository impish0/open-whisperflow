use crate::error::{AppError, Result};

// TODO: Implement global hotkey management
// This will use tauri-plugin-global-shortcut

pub struct HotkeyManager;

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn register_recording_hotkey(&mut self, _shortcut: &str) -> Result<()> {
        // TODO: Implement using tauri-plugin-global-shortcut
        log::warn!("Hotkey registration not yet implemented");
        Ok(())
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new().expect("Failed to create HotkeyManager")
    }
}

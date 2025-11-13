// Phase 4 feature - not yet implemented
#![allow(dead_code)]

use crate::error::Result;

pub struct HotkeyManager;

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn register_recording_hotkey(&mut self, _shortcut: &str) -> Result<()> {
        log::warn!("Hotkey registration not yet implemented");
        Ok(())
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new().expect("Failed to create HotkeyManager")
    }
}

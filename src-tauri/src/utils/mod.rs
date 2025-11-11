use std::path::Path;
use tokio::fs;

use crate::error::Result;

/// Securely delete a file by overwriting with zeros first (paranoid mode)
pub async fn secure_delete_file(path: &Path) -> Result<()> {
    // For MVP, just delete normally (secure overwrite planned for paranoid mode)
    fs::remove_file(path).await?;
    log::debug!("Deleted file: {}", path.display());
    Ok(())
}

/// Get system information
pub fn get_system_info() -> String {
    use sysinfo::System;

    let sys = System::new_all();

    format!(
        "OS: {} {}\nCPU: {}\nTotal RAM: {} GB",
        System::name().unwrap_or_default(),
        System::os_version().unwrap_or_default(),
        sys.cpus()
            .first()
            .map(|cpu| cpu.brand())
            .unwrap_or("Unknown"),
        sys.total_memory() / 1_073_741_824,
    )
}

/// Format duration in human-readable form
pub fn format_duration(seconds: f64) -> String {
    if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else {
        let minutes = (seconds / 60.0).floor();
        let secs = seconds % 60.0;
        format!("{}m {:.0}s", minutes, secs)
    }
}

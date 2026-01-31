mod shell;

use anyhow::Result;
use tracing::{Level, info};


fn main() -> Result<()> {
    // Initialize tracing (adjust based on your setup)
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Test detection
    let shell_type = shell::get_shell(None)?;
    info!("Detected: {}", shell_type);

    // Test override
    let shell_type = shell::get_shell(Some(shell::ShellType::PowerShell))?;
    info!("Override: {}", shell_type);

    Ok(())
}

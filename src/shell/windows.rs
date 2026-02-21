use super::Shell;
use anyhow::Result;
use std::path::PathBuf;

pub struct PowerShell;

impl Shell for PowerShell {
    fn name(&self) -> &'static str {
        "PowerShell"
    }

    fn alias_syntax(&self, name: &str, command: &str) -> String {
        // Set-Alias only works for simple command->command (no args).
        // For commands with arguments/pipes, we use a function wrapper.
        if command.contains(' ') || command.contains('|') || command.contains(';') {
            format!("function {} {{ {} }}", name, command)
        } else {
            format!("Set-Alias -Name {} -Value {}", name, command)
        }
    }

    fn config_path(&self) -> Result<PathBuf> {
        // PowerShell 7+: ~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
        Ok(home
            .join("Documents")
            .join("PowerShell")
            .join("Microsoft.PowerShell_profile.ps1"))
    }

    fn reload_instructions(&self) -> String {
        String::from("Restart PowerShell or run: . $PROFILE")
    }
}

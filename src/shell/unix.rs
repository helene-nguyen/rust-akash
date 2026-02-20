use super::Shell;
use anyhow::Result;
use std::path::PathBuf;

pub struct Bash;
pub struct Zsh;

impl Shell for Bash {
    fn name(&self) -> &'static str {
        "Bash"
    }

    fn alias_syntax(&self, name: &str, command: &str) -> String {
        // Escape single quotes: replace ' with '\''
        format!("alias {}='{}'", name, command.replace("'", "'\\''"))
    }

    fn config_path(&self) -> Result<PathBuf> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
        Ok(home.join(".bashrc"))
    }

    fn reload_instructions(&self) -> String {
        String::from("Restart your terminal or run: source ~/.bashrc or exec bash")
    }
}

impl Shell for Zsh {
    fn name(&self) -> &'static str {
        "Zsh"
    }

    fn alias_syntax(&self, name: &str, command: &str) -> String {
        format!("alias {}='{}'", name, command.replace("'", "'\\''"))
    }

    fn config_path(&self) -> Result<PathBuf> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
        Ok(home.join(".zshrc"))
    }

    fn reload_instructions(&self) -> String {
        String::from("Restart your terminal or run: source ~/.zshrc")
    }
}
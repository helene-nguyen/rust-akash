mod unix;
mod windows;

use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;
use sysinfo::{Pid, Process, System};
use tracing::{debug, info, trace};

// ============================================================================
// SHELL TRAIT
// ============================================================================
/// Trait defining what each shell implementation must provide.
/// Only the methods that differ per shell are required.
pub trait Shell {
    /// Shell display name (e.g., "PowerShell", "Bash", "Zsh")
    fn name(&self) -> &'static str;

    /// Generate the alias syntax for this shell.
    /// e.g. Bash: `alias ll='ls -la'`
    /// e.g. PowerShell: `function ll { ls -la }`
    fn alias_syntax(&self, name: &str, command: &str) -> String;

    /// Path to the shell's config file.
    fn config_path(&self) -> Result<PathBuf>;

    /// Instructions to reload the shell config.
    fn reload_instructions(&self) -> String;

    /// Comment prefix for this shell (default: "#")
    fn comment_prefix(&self) -> &'static str {
        "#"
    }

    /// Begin marker for the akash-managed block
    fn begin_marker(&self) -> String {
        format!("{} BEGIN akash aliases", self.comment_prefix())
    }

    /// End marker for the akash-managed block
    fn end_marker(&self) -> String {
        format!("{} END akash aliases", self.comment_prefix())
    }

    /// Generate the full alias block from a set of aliases.
    fn generate_alias_block(&self, aliases: &BTreeMap<String, String>) -> String {
        let mut lines = Vec::new();
        lines.push(self.begin_marker());
        for (name, command) in aliases {
            lines.push(self.alias_syntax(name, command));
        }
        lines.push(self.end_marker());
        lines.join("\n")
    }
}

/// All shells that akash supports
#[derive(Debug, Clone, Copy, PartialEq)]
//       ↑      ↑      ↑      ↑
//       │      │      │      └── Enables == comparison
//       │      │      └── Auto-copy (no move) because it's small
//       │      └── Enables .clone() method
//       └── Enables {:?} debug printing
pub enum ShellType {
    Bash,
    Zsh,
    PowerShell,
    // Fish,  // For later
}

/// Display trait: how to print ShellType as a user-friendly string
impl std::fmt::Display for ShellType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Bash => write!(formatter, "Bash Shell"),
            ShellType::Zsh => write!(formatter, "Zsh Shell"),
            ShellType::PowerShell => write!(formatter, "PowerShell Shell"),
        }
    }
}

/// FromStr trait: how to parse a string into ShellType
/// Enables: "bash".parse::<ShellType>() → Ok(ShellType::Bash)
/// Used by: clap to parse --shell argument
impl std::str::FromStr for ShellType {
    type Err = anyhow::Error;

    // Method signature from FromStr trait (What to use)
    fn from_str(input: &str) -> Result<Self> {
        // Body: How to parse the string (What to do)
        // Normalize once, reuse
        let normalized = input.to_lowercase();

        match normalized.as_str() {
            "bash" | "git-bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "powershell" | "pwsh" => Ok(Self::PowerShell),
            _ => anyhow::bail!("Unsupported shell: '{}'. Supported: bash, zsh, powershell", input),
        }
    }
}


// ============================================================================
// DETECTION: Parent Process
// ============================================================================

/// Gets the name of the process that launched akash (the shell)
fn get_parent_process_name() -> Result<String> {
    let mut system = System::new_all();
    system.refresh_all();

    // Get current process ID
    let current_pid = std::process::id();
    trace!("Current PID: {}", current_pid);

    // Find current process
    let current_process: &Process = system
        .process(Pid::from_u32(current_pid))
        .ok_or_else(|| anyhow!("Cannot find current process"))?;

    // Get parent PID
    let parent_pid: Pid = current_process
        .parent()
        .ok_or_else(|| anyhow!("No parent process found"))?;
    trace!("Parent PID: {:?}", parent_pid);

    // Get parent process
    let parent_process: &Process = system
        .process(parent_pid)
        .ok_or_else(|| anyhow!("Cannot find parent process"))?;

    // Get parent process name
    let parent_name = parent_process.name().to_string_lossy().into_owned();
    debug!("Parent process name: {}", parent_name);

    Ok(parent_name)
}

/// Try to detect shell from parent process name
fn detect_from_parent_process() -> Option<ShellType> {
    let parent_name: String = get_parent_process_name().ok()?;
    let parent_lower: String = parent_name.to_lowercase();

    if parent_lower.contains("bash") || parent_lower.contains("git-bash") {
        debug!("Detected Bash from parent process");
        return Some(ShellType::Bash);
    }
    if parent_lower.contains("zsh") {
        debug!("Detected Zsh from parent process");
        return Some(ShellType::Zsh);
    }
    if parent_lower.contains("pwsh") || parent_lower.contains("powershell") {
        debug!("Detected PowerShell from parent process");
        return Some(ShellType::PowerShell);
    }

    debug!("Unknown parent process: {}", parent_name);
    None
}


// ============================================================================
// DETECTION: Environment Variables
// ============================================================================

/// Try to detect shell from environment variables
fn detect_from_env() -> Option<ShellType> {
    // Check $SHELL (Unix login shell)
    if let Ok(shell) = std::env::var("SHELL") {
        trace!("$SHELL = {}", shell);
        let shell_lower = shell.to_lowercase();

        if shell_lower.contains("zsh") {
            debug!("Detected Zsh from $SHELL");
            return Some(ShellType::Zsh);
        }
        if shell_lower.contains("bash") {
            debug!("Detected Bash from $SHELL");
            return Some(ShellType::Bash);
        }
    }

    // Check PowerShell-specific variable
    if std::env::var("PSModulePath").is_ok() {
        debug!("Detected PowerShell from $PSModulePath");
        return Some(ShellType::PowerShell);
    }

    // Check version variables
    if std::env::var("BASH_VERSION").is_ok() {
        debug!("Detected Bash from $BASH_VERSION");
        return Some(ShellType::Bash);
    }
    if std::env::var("ZSH_VERSION").is_ok() {
        debug!("Detected Zsh from $ZSH_VERSION");
        return Some(ShellType::Zsh);
    }

    debug!("Could not detect shell from environment");
    None
}

// ============================================================================
// DETECTION: OS Fallback
// ============================================================================

/// Fallback detection based on OS
fn detect_from_os() -> ShellType {
    #[cfg(target_os = "windows")]
    {
        debug!("Fallback: Windows → PowerShell");
        ShellType::PowerShell
    }

    #[cfg(target_os = "macos")]
    {
        debug!("Fallback: macOS → Zsh");
        ShellType::Zsh
    }

    #[cfg(target_os = "linux")]
    {
        debug!("Fallback: Linux → Bash");
        ShellType::Bash
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        warn!("Unknown OS, defaulting to Bash");
        ShellType::Bash
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Detect the current shell using multiple methods
pub fn detect_shell() -> ShellType {
    // Method 1: Parent process (most accurate)
    if let Some(shell) = detect_from_parent_process() {
        return shell;
    }

    // Method 2: Environment variables (fallback)
    if let Some(shell) = detect_from_env() {
        return shell;
    }

    // Method 3: OS default (last resort)
    detect_from_os()
}

/// Factory function to get the appropriate Shell implementation based on the OS and shell type.
/// Create a Shell instance (with optional override)
pub fn get_shell(override_shell: Option<ShellType>) -> Result<Box<dyn Shell>> {
    let shell_type = match override_shell {
        Some(st) => {
            info!("Shell override: {}", st);
            st
        }
        None => {
            let detected = detect_shell();
            info!("Shell detected: {}", detected);
            detected
        }
    };

    match shell_type {
        ShellType::Bash => Ok(Box::new(unix::Bash)),
        ShellType::Zsh => Ok(Box::new(unix::Zsh)),
        ShellType::PowerShell => Ok(Box::new(windows::PowerShell)),
    }
}

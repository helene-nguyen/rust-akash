mod unix;
mod windows;

use anyhow::Result;
// use std::collections::HashMap;
// use std::path::PathBuf;
use sysinfo::{System, Pid};

// / Trait defining shell operations for each supported shell.
// pub trait Shell {
//     // / Shell name (e.g., "bash", "zsh", "powershell") for display purposes.
//     fn name(&self) -> &'static str;

//     /// Path to the shell's configuration file.
//     fn config_path(&self) -> Result<PathBuf>;

//     /// Add an alias to the shell configuration.
//     fn add_alias(&self, alias_name: &str, command: &str) -> Result<()>;

//     // / Remove an alias from the shell configuration.
//     fn remove_alias(&self, alias_name: &str) -> Result<()>;

//     // / List all aliases in the shell configuration.
//     // / Returns a HashMap of alias names to commands.
//     fn list_aliases(&self) -> Result<HashMap<String, String>>;

//     // / Alias exists check.
//     fn alias_exists(&self, alias_name: &str) -> Result<bool> {
//         let aliases = self.list_aliases()?;
//         Ok(aliases.contains_key(alias_name))
//     }

//     // / Reload the shell configuration to apply changes.
//     // / This may involve sourcing the config file or restarting the shell.
//     fn reload_instructions(&self) -> String;

// }

/// Factory function to get the appropriate Shell implementation based on the OS and shell type.
// pub fn get_shell(shell_type: &str) -> Result<Box<dyn Shell>> {
//     #[cfg(target_os = "windows")]
//     {
//         log::debug!("[Shell] OS detected: Windows");
//         Ok(Box::new(windows::Powershell::new()?))
//     }

//     #[cfg(target_os = "linux")]
//     {
//         log::debug!("[Shell] OS detected: Linux");
//         match shell_type {
//             "bash" => Ok(Box::new(unix::Bash::new()?)),
//             "zsh" => Ok(Box::new(unix::Zsh::new()?)),
//             _ => Err(anyhow::anyhow!("Unsupported shell type: {}", shell_type)),
//         }
//     }
// }

pub fn get_parent_process_name() -> Result<String> {
      // 1. Create a System instance to query process info
      let mut system = System::new_all();
      system.refresh_all();

      // 2. Get current process ID
      let current_pid = std::process::id();

      // 3. Find current process in the system
      let current_process = system.process(Pid::from_u32(current_pid))
          .ok_or_else(|| anyhow::anyhow!("Cannot find current process"))?;

      // 4. Get parent PID
      let parent_pid = current_process.parent()
          .ok_or_else(|| anyhow::anyhow!("No parent process"))?;

      // 5. Get parent process
      let parent_process = system.process(parent_pid)
          .ok_or_else(|| anyhow::anyhow!("Cannot find parent process"))?;

      // 6. Get parent process name
      // OsStr = Operating System String (can contain non-UTF8 characters on some systems)
      let parent_name = parent_process.name();
      
      // to_string_lossy() = Converts OsStr to String, replacing invalid UTF-8 with if needed  
      Ok(parent_name.to_string_lossy().into_owned())
  }
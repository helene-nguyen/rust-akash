use crate::shell::ShellType;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "akash",
    about = "also known as + shell -- A cross-platform alias manager",
    version
)]
pub struct Cli {
    /// Override detected shell (bash, zsh, powershell)
    #[arg(long, short, global = true)]
    pub shell: Option<ShellType>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new alias
    Add {
        /// Alias name (e.g. "gs")
        name: String,
        /// Command the alias expands to (e.g. "git status")
        command: String,
    },
    /// Remove an existing alias
    Remove {
        /// Alias name to remove
        name: String,
    },
    /// List all aliases
    List,
    /// Write aliases to your shell config file
    Apply,
    /// Configure shell to auto-load akash aliases on startup
    Init,
}

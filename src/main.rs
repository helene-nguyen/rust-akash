mod shell;
mod store;
mod cli;
mod interactive;

use anyhow::{ Context, Result };
use clap::Parser;
use colored::Colorize;
use shell::Shell;
use store::{ AliasStore };
use tracing::Level;

fn main() -> Result<()> {
    // Initialize tracing (adjust based on your setup)
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cli = cli::Cli::parse();
    let shell = shell::get_shell(cli.shell)?;

    match cli.command {
        Some(cli::Command::Add { name, command }) => cmd_add(&name, &command)?,
        Some(cli::Command::Remove { name }) => cmd_remove(&name, shell.as_ref())?,
        Some(cli::Command::List) => cmd_list()?,
        Some(cli::Command::Apply) => cmd_apply(shell.as_ref())?,
        Some(cli::Command::Init) => cmd_init(shell.as_ref())?,
        None => interactive::run(shell.as_ref())?,
    }

    Ok(())
}

// ============================================================================
// COMMANDS
// ============================================================================

fn cmd_add(name: &str, command: &str) -> Result<()> {
    AliasStore::validate_alias_name(name)?;

    let mut store = AliasStore::store_load()?;

    if store.has_key(name) {
        println!("{} alias '{}' already exists, overwriting", "Warning:".yellow(), name);
    }

    let is_new = store.add_alias(name.to_string(), command.to_string());
    store.store_save()?;

    if is_new {
        println!("{} {} -> {}", "Added:".green(), name.bold(), command);
    } else {
        println!("{} {} -> {}", "Updated:".yellow(), name.bold(), command);
    }
    println!("Run {} to write to your shell config", "akash apply".cyan());
    Ok(())
}

fn cmd_remove(name: &str, shell: &dyn Shell) -> Result<()> {
    let mut store = AliasStore::store_load()?;

    if store.remove_alias(name) {
        store.store_save()?;
        println!("{} {}", "Removed:".green(), name.bold());
        cmd_apply(shell)?;
    } else {
        println!("{} alias '{}' not found", "Error:".red(), name);
    }
    Ok(())
}

fn cmd_list() -> Result<()> {
    let store = AliasStore::store_load()?;
    let aliases = store.list_aliases();

    if aliases.is_empty() {
        println!("No aliases defined. Use {} to create one.", "akash add <name> <command>".cyan());
        return Ok(());
    }

    // Find the longest alias name for alignment
    let max_len = aliases
        .keys()
        .map(|k| k.len())
        .max()
        .unwrap_or(0);

    println!("{}", "Aliases:".bold());
    for (name, command) in aliases {
        println!("  {:width$}  ->  {}", name.green(), command, width = max_len);
    }
    Ok(())
}

pub fn cmd_apply(shell: &dyn Shell) -> Result<()> {
    let store = AliasStore::store_load()?;
    let aliases = store.list_aliases();

    let block = shell.generate_alias_block(aliases);
    let config_path = shell.config_path()?;

    // Read existing config (or empty string if file doesn't exist)
    let content = if config_path.exists() {
        std::fs
            ::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?
    } else {
        String::new()
    };

    let new_content = replace_or_append_block(
        &content,
        &shell.begin_marker(),
        &shell.end_marker(),
        &block
    );

    // Create parent directories if needed
    if let Some(parent) = config_path.parent() {
        std::fs
            ::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    std::fs
        ::write(&config_path, new_content)
        .with_context(|| format!("Failed to write {}", config_path.display()))?;

    println!(
        "{} Wrote {} aliases to {}",
        "Done!".green().bold(),
        aliases.len(),
        config_path.display()
    );

    if aliases.is_empty() {
        println!("{} Cleared all aliases from {}", "Done!".green().bold(), config_path.display());
    } else {
        println!("{} Wrote {} aliases to {}", "Done!".green().bold(), aliases.len(), config_path.display());
    }

    println!("{}", shell.reload_instructions().cyan());
    Ok(())
}

pub fn cmd_init(shell: &dyn Shell) -> Result<()> {
    cmd_apply(shell)?;
    println!("\n{} akash initialized for {}.", "Ready!".green().bold(), shell.name().bold());
    println!("Your aliases will be loaded when you open a new {} session.", shell.name());
    Ok(())
}

// ============================================================================
// HELPER: Block replacement
// ============================================================================

/// Replace the akash block between markers, or append if not found.
fn replace_or_append_block(
    content: &str,
    begin_marker: &str,
    end_marker: &str,
    new_block: &str
) -> String {
    if
        let (Some(begin_pos), Some(end_pos)) = (
            content.find(begin_marker),
            content.find(end_marker),
        )
    {
        // Found existing block -> replace it
        let before = &content[..begin_pos];
        let after_end = end_pos + end_marker.len();
        let after = &content[after_end..];

        let mut result = before.trim_end_matches('\n').to_string();
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(new_block);
        result.push_str(after);
        result
    } else {
        // No existing block -> append
        let mut result = content.to_string();
        if !result.ends_with('\n') && !result.is_empty() {
            result.push('\n');
        }
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(new_block);
        result.push('\n');
        result
    }
}

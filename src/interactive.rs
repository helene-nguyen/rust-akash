use anyhow::Result;
use colored::Colorize;
use std::io::{self, BufRead, Write};

use crate::config::Config;
use crate::shell::Shell;
use crate::store::AliasStore;

pub fn run(config: &Config, shell: &dyn Shell) -> Result<()> {
    println!(
        "{}",
        r#" .--..--..--..--..--..--..--..--..--..--..--..--..--. 
/ .. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \
\ \/\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ \/ /
 \/ /`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'\/ / 
 / /\                                            / /\ 
/ /\ \  █████╗ ██╗  ██╗ █████╗ ███████╗██╗  ██╗ / /\ \
\ \/ / ██╔══██╗██║ ██╔╝██╔══██╗██╔════╝██║  ██║ \ \/ /
 \/ /  ███████║█████╔╝ ███████║███████╗███████║  \/ / 
 / /\  ██╔══██║██╔═██╗ ██╔══██║╚════██║██╔══██║  / /\ 
/ /\ \ ██║  ██║██║  ██╗██║  ██║███████║██║  ██║ / /\ \
\ \/ / ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝ \ \/ /
 \/ /                                            \/ / 
 / /\.--..--..--..--..--..--..--..--..--..--..--./ /\ 
/ /\ \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \.. \/\ \
\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `'\ `' /
 `--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--' "#
            .bold()
    );
    println!("Detected shell: {}\n", shell.name().cyan());

    loop {
        println!("What would you like to do?");
        println!("  1) Add an alias");
        println!("  2) Remove an alias");
        println!("  3) List aliases");
        println!("  4) Apply aliases to shell config");
        println!("  5) Init (setup shell config)");
        println!("  q) Quit");
        print!("\n> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => interactive_add(config)?,
            "2" => interactive_remove(config)?,
            "3" => interactive_list(config)?,
            "4" => {
                // Reuse the cmd_apply logic from main
                crate::cmd_apply(config, shell)?;
            }
            "5" => {
                crate::cmd_init(config, shell)?;
            }
            "q" | "Q" | "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("{}", "Invalid choice. Try again.".red()),
        }
        println!();
    }
    Ok(())
}

fn prompt(label: &str) -> Result<String> {
    print!("{}: ", label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn interactive_add(config: &Config) -> Result<()> {
    let name = prompt("Alias name")?;
    let command = prompt("Command")?;

    if name.is_empty() || command.is_empty() {
        println!("{}", "Name and command cannot be empty.".red());
        return Ok(());
    }

    AliasStore::validate_alias_name(&name)?;

    let mut store = AliasStore::store_load(config.aliases_path.as_ref())?;
    let is_new = store.add_alias(name.clone(), command.clone());
    store.store_save(config.aliases_path.as_ref())?;

    if is_new {
        println!("{} {} -> {}", "Added:".green(), name.bold(), command);
    } else {
        println!("{} {} -> {}", "Updated:".yellow(), name.bold(), command);
    }
    Ok(())
}

fn interactive_remove(config: &Config) -> Result<()> {
    let name = prompt("Alias name to remove")?;

    if name.is_empty() {
        println!("{}", "Name cannot be empty.".red());
        return Ok(());
    }

    let mut store = AliasStore::store_load(config.aliases_path.as_ref())?;
    if store.remove_alias(&name) {
        store.store_save(config.aliases_path.as_ref())?;
        println!("{} {}", "Removed:".green(), name.bold());
    } else {
        println!("{} alias '{}' not found", "Error:".red(), name);
    }
    Ok(())
}

fn interactive_list(config: &Config) -> Result<()> {
    let store = AliasStore::store_load(config.aliases_path.as_ref())?;
    let aliases = store.list_aliases();

    if aliases.is_empty() {
        println!("No aliases defined yet.");
        return Ok(());
    }

    let max_len = aliases.keys().map(|k| k.len()).max().unwrap_or(0);

    println!("{}", "Aliases:".bold());
    for (name, command) in aliases {
        println!(
            "  {:width$}  ->  {}",
            name.green(),
            command,
            width = max_len
        );
    }
    Ok(())
}

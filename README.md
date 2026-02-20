# akash

> Name decomposition: **a**lso **k**nown **a**s + **sh**ell

**Tired of rewriting the same aliases every time you switch shells or machines?**

Whether you're jumping between Bash on Linux, Zsh on macOS, and PowerShell on Windows, keeping your aliases in sync is a nightmare. Each shell has its own syntax, its own config file, and your carefully crafted shortcuts get scattered across different dotfiles.

Akash solves this by giving you **one place to rule them all** 💍 A single, portable alias store that automatically translates to the right syntax for whatever shell you're using. Write once, use everywhere.

A cross-platform CLI tool for managing shell aliases, written in Rust.

## Features

- Centralized alias management across multiple shells
- Cross-platform support (Windows, macOS, Linux)
- Interactive mode with menu-driven interface
- Automatic shell detection
- Persistent storage in JSON format
- Safe config file modification with block markers

## Supported Shells

| Shell      | Config File                                               | Platform               |
| ---------- | --------------------------------------------------------- | ---------------------- |
| Bash       | `~/.bashrc`                                               | Linux, macOS, Git Bash |
| Zsh        | `~/.zshrc`                                                | macOS, Linux           |
| PowerShell | `~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1` | Windows                |

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/helene-nguyen/rust-akash.git
cd rust-akash

# Build and install
cargo build --release

# The binary will be at target/release/akash (or akash.exe on Windows)
```

### Add to PATH

After building, add the binary to your PATH or copy it to a directory in your PATH:

```bash
# Linux/macOS
cp target/release/akash ~/.local/bin/

# Windows (PowerShell)
Copy-Item target\release\akash.exe $env:USERPROFILE\.local\bin\
```

## Usage

Akash provides two binary names for convenience: `akash` and `aka`.

### Commands

```bash
# Add a new alias
akash add <name> <command>
aka add gs "git status"

# Remove an alias
akash remove <name>
aka remove gs

# List all aliases
akash list
aka list

# Apply aliases to shell config
akash apply
aka apply

# Initialize shell configuration (first-time setup)
akash init
aka init

# Start interactive mode (default when no command)
akash
aka
```

### Global Flags

```bash
# Override detected shell
akash --shell <SHELL> <command>
akash -s bash add gs "git status"

# Supported shell values: bash, zsh, powershell, pwsh, git-bash
```

### Examples

```bash
# Add common git aliases
aka add gs "git status"
aka add gp "git push"
aka add gl "git log --oneline -10"
aka add gco "git checkout"

# Add system aliases
aka add ll "ls -la"
aka add ..  "cd .."
aka add cls "clear"

# Apply to shell and see them take effect
aka apply

# Reload your shell or source the config
source ~/.bashrc  # Bash
source ~/.zshrc   # Zsh
. $PROFILE        # PowerShell
```

## Interactive Mode

Running `akash` or `aka` without arguments starts the interactive mode:

```sh
 .--..--..--..--..--..--..--..--..--..--..--..--..--.
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
 `--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'`--'
Detected shell: PowerShell

What would you like to do?
  1) Add an alias
  2) Remove an alias
  3) List aliases
  4) Apply aliases to shell config
  5) Init (setup shell config)
  q) Quit
```

## How It Works

### Alias Storage

Aliases are stored in `~/.akash/aliases.json`:

```json
{
  "aliases": {
    "gs": "git status",
    "gp": "git push",
    "ll": "ls -la"
  }
}
```

### Shell Config Modification

When you run `akash apply`, the tool:

1. Reads your shell's config file
2. Looks for akash markers (`# BEGIN akash aliases` / `# END akash aliases`)
3. Replaces the block between markers (or appends if not found)
4. Preserves all other content in your config

Example block added to `.bashrc`:

```bash
# BEGIN akash aliases
alias gs='git status'
alias gp='git push'
alias ll='ls -la'
# END akash aliases
```

### Shell Detection

Akash automatically detects your current shell using:

1. Parent process name (most accurate)
2. Environment variables (`$SHELL`, `$BASH_VERSION`, `$ZSH_VERSION`)
3. OS fallback (Windows → PowerShell, macOS → Zsh, Linux → Bash)

Override with `--shell` flag if detection is incorrect.

## Alias Name Rules

Valid alias names must:

- Be non-empty
- Contain only alphanumeric characters, underscores (`_`), or hyphens (`-`)

```bash
# Valid
aka add my-alias "echo hello"
aka add my_alias "echo hello"
aka add alias123 "echo hello"

# Invalid
aka add "my alias" "echo hello"  # Contains space
aka add "alias!" "echo hello"    # Contains special character
```

## Development

### Prerequisites

- Rust 1.85+ (Edition 2024)
- Cargo

### Building

```bash
cargo build          # Debug build
cargo build --release # Release build
```

### Running Tests

```bash
cargo test
```

Tests use Gherkin-style naming (`given_X_when_Y_then_Z`) for clarity.

### Project Structure

```
src/
├── main.rs           # Entry point and command handlers
├── cli.rs            # CLI argument parsing (clap)
├── store.rs          # Alias storage and persistence
├── interactive.rs    # Interactive mode UI
└── shell/
    ├── mod.rs        # Shell trait and detection
    ├── unix.rs       # Bash and Zsh implementations
    └── windows.rs    # PowerShell implementation
```

## Dependencies

| Crate              | Purpose              |
| ------------------ | -------------------- |
| clap               | CLI argument parsing |
| serde + serde_json | JSON serialization   |
| colored            | Terminal colors      |
| dirs               | Cross-platform paths |
| anyhow             | Error handling       |
| tracing            | Structured logging   |
| sysinfo            | Process detection    |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

[Helene Nguyen](https://github.com/helene-nguyen) aka Yumi

---

## Other similar tools

<details>
<summary>RUST, Go and Shell scripts projects</summary>

Here are the nice existing projects with their links:

#### Rust Projects

| Name              | Link                                   | Description                               |
| ----------------- | -------------------------------------- | ----------------------------------------- |
| **aliasmgr**      | <https://github.com/Faria22/aliasmgr>    | TOML config, syncs aliases, no PowerShell |
| **rbam**          | <https://crates.io/crates/rbam>          | Bash Alias Manager, simple, Bash only     |
| **alias-manager** | <https://crates.io/crates/alias-manager> | Index-based command picker, Linux only    |
| **easy-alias**    | <https://crates.io/crates/easy-alias>    | Similar to alias-manager                  |

---

#### Go Projects

| Name         | Link                                 | Description                              |
| ------------ | ------------------------------------ | ---------------------------------------- |
| **aliasctl** | <https://github.com/aliasctl/aliasctl> | Full-featured, AI-powered, all platforms |

---

#### Shell Script Projects

| Name              | Link                                           | Description                         |
| ----------------- | ---------------------------------------------- | ----------------------------------- |
| **aliasman**      | <https://github.com/BeyondCodeBootcamp/aliasman> | Bash, Zsh, Fish support, no Windows |
| **alf**           | <https://github.com/DannyBen/alf>                | Bash Alias Generator, Bash/Zsh only |
| **alias_manager** | <https://github.com/KazeTachinuu/alias_manager>  | Lightweight, Linux/macOS only       |
| **aliasme**       | <https://github.com/Jintin/aliasme>              | Simple shell script                 |

</details>

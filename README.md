![Akash image](https://raw.githubusercontent.com/helene-nguyen/rust-akash/refs/heads/main/docs/media/akash.svg)

# Akash

> Name decomposition: **a**lso **k**nown **a**s + **sh**ell

**Tired of rewriting the same aliases every time you switch shells or machines?**

Whether you're jumping between Bash on Linux, Zsh on macOS, and PowerShell on Windows, keeping your aliases in sync is a nightmare. Each shell has its own syntax, its own config file, and your carefully crafted shortcuts get scattered across different dotfiles.

Akash solves this by giving you **one place to rule them all** 💍 A single, portable alias store that automatically translates to the right syntax for whatever shell you're using. Write once, use everywhere.

A cross-platform CLI tool for managing shell aliases, written in Rust.

Table of Contents

- [Akash](#akash)
  - [Quick Start](#quick-start)
  - [Features](#features)
  - [Supported Shells](#supported-shells)
  - [Installation](#installation)
    - [From Source](#from-source)
    - [Install from crates](#install-from-crates)
    - [Add to PATH](#add-to-path)
    - [Uninstall](#uninstall)
  - [Usage](#usage)
    - [Commands](#commands)
    - [Global Flags](#global-flags)
    - [Examples](#examples)
  - [Interactive Mode](#interactive-mode)
  - [How It Works](#how-it-works)
    - [Alias Storage](#alias-storage)
    - [Shell Config Modification](#shell-config-modification)
    - [Shell Detection](#shell-detection)
  - [Alias Name Rules](#alias-name-rules)
  - [Development](#development)
    - [Prerequisites](#prerequisites)
    - [Building](#building)
    - [Running Tests](#running-tests)
    - [Project Structure](#project-structure)
    - [Understand CI/CD pipeline](#understand-cicd-pipeline)
  - [Dependencies](#dependencies)
  - [Troubleshooting](#troubleshooting)
    - [Aliases not working after `apply`](#aliases-not-working-after-apply)
    - [Wrong shell detected](#wrong-shell-detected)
    - [Permission denied on config file](#permission-denied-on-config-file)
    - [Aliases not persisting](#aliases-not-persisting)
  - [Contributing](#contributing)
  - [Releasing (Maintainers only)](#releasing-maintainers-only)
    - [Using the release script](#using-the-release-script)
    - [Manual release](#manual-release)
    - [Using Makefile](#using-makefile)
    - [What happens after the tag is pushed](#what-happens-after-the-tag-is-pushed)
  - [License](#license)
  - [Author](#author)
  - [Why Akash?](#why-akash)
  - [Other similar tools](#other-similar-tools)
    - [Rust Projects](#rust-projects)
    - [Go Projects](#go-projects)
    - [Shell Script Projects](#shell-script-projects)

## Quick Start

```bash
# 1. Build the project
cargo build --release

# 2. Add some aliases
aka add gs "git status"
aka add gp "git push"
aka add ll "ls -la"

# 3. Apply to your shell config
aka apply

# 4. Reload your shell
source ~/.bashrc   # or ~/.zshrc, or restart PowerShell
```

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

### Install from crates

```bash
cargo install akash
```

### Add to PATH

After building, add the binary to your PATH or copy it to a directory in your PATH:

```bash
# Linux/macOS
cp target/release/akash ~/.local/bin/

# Windows (PowerShell)
Copy-Item target\release\akash.exe $env:USERPROFILE\.local\bin\
```

### Uninstall

```bash
# 1. Remove the binary
rm ~/.local/bin/akash  # or wherever you installed it

# 2. Remove the alias store (optional)
rm -rf ~/.akash

# 3. Remove the alias block from your shell config
# Edit ~/.bashrc, ~/.zshrc, or $PROFILE and delete the block between:
# BEGIN akash aliases ... END akash aliases
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

```sh
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

### Understand CI/CD pipeline

The CI/CD and release process is documented in [Build Release Guide](docs/build-release-guide.md), covering:

- GitHub Actions workflow structure
- Build and release steps
- Caching strategy
- Common issues and troubleshooting tips

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

## Troubleshooting

### Aliases not working after `apply`

Make sure to reload your shell configuration:

```bash
source ~/.bashrc   # Bash
source ~/.zshrc    # Zsh
. $PROFILE         # PowerShell
```

Or simply restart your terminal.

### Wrong shell detected

Use the `--shell` flag to override detection:

```bash
aka --shell bash apply
aka -s zsh list
```

### Permission denied on config file

Ensure you have write permissions to your shell's config file:

```bash
ls -la ~/.bashrc   # Check permissions
chmod u+w ~/.bashrc # Add write permission if needed
```

### Aliases not persisting

Check that your alias store exists and is valid JSON:

```bash
cat ~/.akash/aliases.json
```

If corrupted, you can reset it:

```bash
rm ~/.akash/aliases.json
aka add test "echo hello"  # Creates a fresh store
```

## Contributing

Contribution is welcomed :)

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## Releasing (Maintainers only)

Releases are fully automated via GitHub Actions. Pushing a `v*` tag triggers the pipeline which builds binaries, publishes to crates.io, pushes Docker images, and creates a GitHub Release.

### Using the release script

```bash
./scripts/release.sh
```

The script will:

1. Read the current version from `Cargo.toml`
2. Prompt you for the new version number
3. Create an annotated git tag (`v<version>`)
4. Push the tag to origin, which triggers the [Auto Release](.github/workflows/auto-release.yml) workflow

### Manual release

```bash
git tag -a v0.2.0 -m ":bookmark: Release v0.2.0"
git push origin v0.2.0
```

### Using Makefile

```bash
make release
# This will run the same steps as the release script

#Output
akash on  main [✓] is 📦 v0.1.0 via 🦀 v1.93.0
> make release
Current version: 0.1.0

New version (without v): 0.2.0
Release title will be: :bookmark: Release v0.2.0

  Tag:   v0.2.0
  Title: :bookmark: Release v0.2.0

Confirm? (y/n): y
Enumerating objects: 1, done.
Counting objects: 100% (1/1), done.
Writing objects: 100% (1/1), 347 bytes | 347.00 KiB/s, done.
Total 1 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
To github.com:helene-nguyen/rust-akash.git
 * [new tag]         v0.2.0 -> v0.2.0

✅ Tag v0.2.0 pushed. Release workflow will start shortly.
   Track it at: https://github.com/helene-nguyen/rust-akash/actions
```

### What happens after the tag is pushed

1. **CI Gate** verifies the CI workflow already passed on this commit
2. **Build** compiles release binaries for Linux, macOS, and Windows (5 targets)
3. **Docker** builds and pushes multi-arch images to Docker Hub and GHCR
4. **Publish** uploads the crate to [crates.io](https://crates.io/crates/akash)
5. **GitHub Release** is created with auto-generated notes and attached binaries
6. **Post-Release PR** is opened to bump the version in `Cargo.toml` and update `CHANGELOG.md`

> [!IMPORTANT]
> Make sure CI has passed on the commit before tagging. The release pipeline will fail at the CI Gate step otherwise.

After the release, merge the auto-generated post-release PR to keep `main` in sync.

For full details, see the [Build Release Guide](docs/build-release-guide.md).

## License

MIT License - see [LICENSE](./COPYRIGHT.md) for details.

## Author

[Helene Nguyen](https://github.com/helene-nguyen) aka Yumi

> [!NOTE]
> This project is a personal side project and not affiliated with any company or organization. I've created this for my own usage and learning, but I hope it can be useful to others as well! ☀️

---

## Why Akash?

Compared to existing tools, Akash offers:

| Feature              | Akash                                  | Most Alternatives        |
| -------------------- | -------------------------------------- | ------------------------ |
| PowerShell support   | Yes                                    | Rarely                   |
| Cross-platform       | Windows, macOS, Linux                  | Usually Linux/macOS only |
| Interactive mode     | Yes                                    | Varies                   |
| Single binary        | Yes (Rust)                             | Often requires runtime   |
| Safe config editing  | Block markers preserve existing config | Often overwrites         |
| Shell auto-detection | Multi-method fallback                  | Basic or manual          |

## Other similar tools

<details>
<summary>RUST, Go and Shell scripts projects</summary>

Here are the nice existing projects with their links:

### Rust Projects

| Name              | Link                                     | Description                               |
| ----------------- | ---------------------------------------- | ----------------------------------------- |
| **aliasmgr**      | <https://github.com/Faria22/aliasmgr>    | TOML config, syncs aliases, no PowerShell |
| **rbam**          | <https://crates.io/crates/rbam>          | Bash Alias Manager, simple, Bash only     |
| **alias-manager** | <https://crates.io/crates/alias-manager> | Index-based command picker, Linux only    |
| **easy-alias**    | <https://crates.io/crates/easy-alias>    | Similar to alias-manager                  |

---

### Go Projects

| Name         | Link                                   | Description                              |
| ------------ | -------------------------------------- | ---------------------------------------- |
| **aliasctl** | <https://github.com/aliasctl/aliasctl> | Full-featured, AI-powered, all platforms |

---

### Shell Script Projects

| Name              | Link                                             | Description                         |
| ----------------- | ------------------------------------------------ | ----------------------------------- |
| **aliasman**      | <https://github.com/BeyondCodeBootcamp/aliasman> | Bash, Zsh, Fish support, no Windows |
| **alf**           | <https://github.com/DannyBen/alf>                | Bash Alias Generator, Bash/Zsh only |
| **alias_manager** | <https://github.com/KazeTachinuu/alias_manager>  | Lightweight, Linux/macOS only       |
| **aliasme**       | <https://github.com/Jintin/aliasme>              | Simple shell script                 |

</details>

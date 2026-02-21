# Changelog

All notable changes to this project will be documented in this file.

## What's Changed

## [0.1.0] - 2026-02-21

### Added

- Alias management commands: `add`, `remove`, `list`, `apply`, `init`
- Cross-platform shell support: PowerShell, Bash, Zsh
- Automatic shell detection (parent process, environment variables, OS fallback)
- Shell override with `--shell` flag
- JSON alias storage at `~/.akash/aliases.json`
- Safe shell config modification with `# BEGIN akash aliases` / `# END akash aliases` markers
- Interactive menu mode when run without arguments
- Configuration file at `~/.akash/config.toml` (log level, default shell)
- Dual binary: `akash` and `aka`
- Colored terminal output
- Gherkin-style unit tests

[0.1.0]: https://github.com/helene-nguyen/rust-akash/releases/tag/v0.1.0

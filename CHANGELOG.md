# Changelog

All notable changes to this project will be documented in this file.

## [v0.1.0] - 2026-02-21

<!-- Release notes generated using configuration in .github/release.yml at v0.1.0 -->

## What's Changed
### Other Changes
* Update CHANGELOG for v0.1.0 by @github-actions[bot] in https://github.com/helene-nguyen/rust-akash/pull/1

## New Contributors
* @github-actions[bot] made their first contribution in https://github.com/helene-nguyen/rust-akash/pull/1

**Full Changelog**: https://github.com/helene-nguyen/rust-akash/commits/v0.1.0



The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

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

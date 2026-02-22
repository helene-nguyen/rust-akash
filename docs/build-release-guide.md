# Build & Release Guide

## Overview

The Akash project uses a two-workflow CI/CD pipeline powered by GitHub Actions.

**CI workflow** (`ci.yml`) runs on every push and pull request. It checks formatting, lints, builds across platforms, and runs tests.

**Auto Release workflow** (`auto-release.yml`) is triggered by pushing a `v*` tag and orchestrates the full release pipeline:

```
push v* tag
  │
  ├── [CI Checks]        ─── fmt, clippy, tests
  ├── [Build Binaries]   ─── 5-platform matrix
  │
  ├── [Docker]           ─── multi-arch build → Docker Hub + GHCR  (needs: ci)
  ├── [Publish]          ─── cargo publish to crates.io             (needs: build + ci)
  │
  ├── [GitHub Release]   ─── attach binaries, generate notes        (needs: build + docker + ci)
  │
  └── [Post-Release PR]  ─── version bump + CHANGELOG update        (needs: release)
```

---

## Prerequisites / Secrets Setup

The following secrets must be configured in **Settings → Secrets and variables → Actions** of the GitHub repository:

| Secret                 | Purpose                | How to obtain                                               |
| ---------------------- | ---------------------- | ----------------------------------------------------------- |
| `DOCKERHUB_USERNAME`   | Docker Hub login       | Your Docker Hub username                                    |
| `DOCKERHUB_TOKEN`      | Docker Hub push access | Docker Hub → Account Settings → Security → New Access Token |
| `CARGO_REGISTRY_TOKEN` | Publish to crates.io   | `cargo login` → https://crates.io/settings/tokens           |

**Built-in tokens** (no setup required):

| Token          | Purpose                                             |
| -------------- | --------------------------------------------------- |
| `GITHUB_TOKEN` | GitHub Release creation, GHCR push, post-release PR |

**Repository permissions** declared at the top level of the workflow:

```yaml
permissions:
  contents: write # create releases, push branches
  pull-requests: write # open post-release PRs
  packages: write # push to GHCR
```

---

## How to Trigger a Release

### 1. Ensure `main` is clean

All CI checks should pass on the latest commit.

### 2. Create and push a tag

```bash
git tag v0.2.0
git push origin v0.2.0
```

**Naming convention:** tags must match the `v*` pattern (e.g., `v0.1.0`, `v1.0.0-rc.1`). The `v` prefix is stripped automatically when setting version numbers.

### 3. What happens automatically

Once the tag is pushed, the **Auto Release** workflow:

1. Runs CI checks (fmt, clippy, tests)
2. Builds release binaries for 5 platform targets in parallel
3. Builds and pushes multi-arch Docker images to Docker Hub and GHCR
4. Publishes the crate to crates.io
5. Creates a GitHub Release with auto-generated release notes and attached binaries
6. Opens a PR against `main` that bumps the version in `Cargo.toml` and updates `CHANGELOG.md`

---

## Pipeline Stages

### Stage 1: CI Checks

**Job:** `ci` | **Runner:** `ubuntu-latest`

Runs the same quality gates as the regular CI workflow, gating all downstream jobs:

- `cargo fmt --all -- --check` enforces consistent formatting
- `cargo clippy --all-targets --all-features -- -D warnings` applies a zero-warning lint policy
- `cargo test --all-features` runs the full test suite

### Stage 2: Binary Builds

**Job:** `build` | **Strategy:** `fail-fast: false` (all targets build even if one fails)

| OS               | Target                      | Artifact name                |
| ---------------- | --------------------------- | ---------------------------- |
| `ubuntu-latest`  | `x86_64-unknown-linux-gnu`  | `akash-linux-x86_64.tar.gz`  |
| `ubuntu-latest`  | `aarch64-unknown-linux-gnu` | `akash-linux-aarch64.tar.gz` |
| `windows-latest` | `x86_64-pc-windows-msvc`    | `akash-windows-x86_64.zip`   |
| `macos-latest`   | `x86_64-apple-darwin`       | `akash-macos-x86_64.tar.gz`  |
| `macos-latest`   | `aarch64-apple-darwin`      | `akash-macos-aarch64.tar.gz` |

Each artifact contains two binaries: `akash` and `aka` (both entry points defined in `Cargo.toml`).

- **Linux ARM64** installs `gcc-aarch64-linux-gnu` for cross-compilation and sets `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER`.
- **Unix** targets are packaged with `tar czf`.
- **Windows** targets are packaged with `7z a`.
- Artifacts are uploaded via `actions/upload-artifact@v4` and later downloaded by the release job.

### Stage 3: Docker Build

**Job:** `docker` | **Depends on:** `ci`

Builds and pushes multi-arch images to **two** registries:

- **Docker Hub:** `<DOCKERHUB_USERNAME>/akash:latest` and `:<version>`
- **GHCR:** `ghcr.io/<owner>/akash:latest` and `:<version>`

**Platforms:** `linux/amd64`, `linux/arm64`

Uses QEMU + Docker Buildx for multi-architecture builds. See the [Dockerfile](#dockerfile) section for build details and the [Caching Strategy](#caching-strategy) section for cache configuration.

### Stage 4: Publish to crates.io

**Job:** `publish` | **Depends on:** `build` + `ci`

Runs `cargo publish` using the `CARGO_REGISTRY_TOKEN` secret. This makes the new version available via `cargo install akash`.

### Stage 5: GitHub Release

**Job:** `release` | **Depends on:** `build` + `docker` + `ci`

- Downloads all build artifacts using `actions/download-artifact@v4` with `merge-multiple: true`
- Creates a GitHub Release via `softprops/action-gh-release@v2`
- Auto-generates release notes from commits since the previous tag
- Attaches all binary archives (`.tar.gz` and `.zip`) to the release

### Stage 6: Post-Release PR

**Job:** `post-release` | **Depends on:** `release`

After the release is published:

1. Extracts the version from the tag (strips `v` prefix)
2. Updates the `version` field in `Cargo.toml` via `sed`
3. Updates (or creates) `CHANGELOG.md` with the release notes pulled from the GitHub Release
4. Opens a PR against `main` on branch `post-release-<tag>` using `peter-evans/create-pull-request@v5`
5. The PR branch is auto-deleted after merge

> [!NOTE]
> **Why a PR instead of a direct push?**
> When you tag `v0.2.0`, `cargo publish` publishes version `0.2.0` to crates.io, but `Cargo.toml` on `main` might still say `0.1.0`. The post-release PR syncs `main` so that `Cargo.toml` and `CHANGELOG.md` reflect what was actually released.
>
> A PR is preferred over a direct push because branch protection rules on `main` would block a direct push, it gives you a chance to review the auto-generated changelog before merging, and if something went wrong in the release, you can close the PR without polluting `main`.
>
> The alternative would be to bump `Cargo.toml` manually before tagging, but then you would need to remember to do it every time, and the changelog update would still need automation.

---

## Dockerfile

The Dockerfile uses a **4-stage multi-stage build** optimized for fast rebuilds and minimal runtime images.

### Stage 1: Chef (base)

```dockerfile
FROM --platform=$BUILDPLATFORM rust:1.88-bookworm AS chef
RUN cargo install cargo-chef
```

Installs [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) on the build platform for dependency caching.

### Stage 2: Planner

```dockerfile
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
```

Analyzes `Cargo.toml` and `Cargo.lock` to produce a `recipe.json`, a fingerprint of the project's dependencies. This file only changes when dependencies change, enabling Docker layer caching.

### Stage 3: Builder

- Determines the Rust target triple from `TARGETPLATFORM` (`linux/amd64` → `x86_64-unknown-linux-gnu`, `linux/arm64` → `aarch64-unknown-linux-gnu`)
- Installs `gcc-aarch64-linux-gnu` for ARM64 cross-compilation
- Runs `cargo chef cook` with the recipe to build and cache **only** dependencies
- Copies source code and runs the final `cargo build --release`
- Strips binaries and copies `akash` + `aka` to `/app/out/`

The key insight is that `cargo chef cook` uses the recipe from the planner stage. Since the recipe only changes when dependencies change, Docker can cache this expensive step across source code changes.

### Stage 4: Runtime

```dockerfile
FROM gcr.io/distroless/cc-debian12
```

Uses Google's [distroless](https://github.com/GoogleContainerTools/distroless) image, a minimal container with only the C runtime libraries needed to run the binary. No shell, no package manager, no unnecessary attack surface.

Final image contains only `/usr/local/bin/akash` and `/usr/local/bin/aka`.

---

## Supported Platforms

### Native Binaries

| Platform       | Target Triple               | Runner           | Notes                                       |
| -------------- | --------------------------- | ---------------- | ------------------------------------------- |
| Linux x86_64   | `x86_64-unknown-linux-gnu`  | `ubuntu-latest`  | Native build                                |
| Linux ARM64    | `aarch64-unknown-linux-gnu` | `ubuntu-latest`  | Cross-compiled with `gcc-aarch64-linux-gnu` |
| macOS x86_64   | `x86_64-apple-darwin`       | `macos-latest`   | Intel Macs                                  |
| macOS ARM64    | `aarch64-apple-darwin`      | `macos-latest`   | Apple Silicon (M1/M2/M3/M4)                 |
| Windows x86_64 | `x86_64-pc-windows-msvc`    | `windows-latest` | MSVC toolchain                              |

### Docker Images

| Platform      | Architecture                      |
| ------------- | --------------------------------- |
| `linux/amd64` | x86_64                            |
| `linux/arm64` | ARM64 / Apple Silicon via Rosetta |

Docker images are available from two registries:

```bash
# Docker Hub
docker pull <username>/akash:latest
docker pull <username>/akash:0.1.0

# GitHub Container Registry
docker pull ghcr.io/<owner>/akash:latest
docker pull ghcr.io/<owner>/akash:0.1.0
```

---

## Caching Strategy

### Docker Registry Cache

```yaml
cache-from: type=registry,ref=<username>/akash:cache
cache-to: type=registry,ref=<username>/akash:cache,mode=max
```

Docker build layers are cached in a dedicated `cache` tag on Docker Hub. The `mode=max` setting caches all layers (including intermediate stages), not just the final image layers. This dramatically speeds up rebuilds when only source code changes.

### Docker with cargo-chef

The `cargo-chef` tool splits the build into two phases:

1. **`cargo chef cook`** builds only dependencies (cached when `recipe.json` is unchanged)
2. **`cargo build`** builds only the project source code

Since dependencies change far less frequently than source code, this avoids recompiling the entire dependency tree on every build.

### CI without explicit caching

The CI and binary build jobs use `dtolnay/rust-toolchain@stable` without additional caching. For a project of this size, the overhead of saving/restoring a cargo cache often exceeds the time saved.

---

## Troubleshooting

### CI failures

| Symptom                            | Cause                  | Fix                                                            |
| ---------------------------------- | ---------------------- | -------------------------------------------------------------- |
| `cargo fmt` fails                  | Unformatted code       | Run `cargo fmt` locally before pushing                         |
| `cargo clippy` fails with warnings | Clippy lint violations | Run `cargo clippy -- -D warnings` locally and fix all warnings |
| Tests fail                         | Code regression        | Run `cargo test --all-features` locally                        |

### Release failures

| Symptom                                       | Cause                                                     | Fix                                                                              |
| --------------------------------------------- | --------------------------------------------------------- | -------------------------------------------------------------------------------- |
| Docker push fails with 401                    | `DOCKERHUB_USERNAME` or `DOCKERHUB_TOKEN` missing/expired | Regenerate the token at Docker Hub → Account Settings → Security                 |
| `cargo publish` fails with 403                | `CARGO_REGISTRY_TOKEN` missing/expired                    | Generate a new token at https://crates.io/settings/tokens                        |
| `cargo publish` fails with "already uploaded" | Tag version matches an existing crates.io version         | Bump the version because crates.io does not allow overwriting published versions |
| Cross-compilation fails for `aarch64`         | Missing cross-compile toolchain                           | Ensure `gcc-aarch64-linux-gnu` install step hasn't been removed                  |
| GitHub Release creation fails                 | Insufficient permissions                                  | Verify `contents: write` is set in the workflow permissions                      |
| Post-release PR fails                         | Branch already exists from a previous run                 | Delete the `post-release-v*` branch and re-run the job                           |

### Rust version mismatch

The project uses `rust:1.88-bookworm` in the Dockerfile and `dtolnay/rust-toolchain@stable` in CI. If the Dockerfile pins a version newer than stable, CI may fail on features not yet available. Keep the Dockerfile Rust version aligned with the stable channel or pin both to the same version.

### Version conflicts

The `post-release` job updates `Cargo.toml` via `sed` to match the tag version. If you tag `v0.2.0` but `Cargo.toml` already says `0.2.0`, the `cargo publish` step will fail because that version is already on crates.io. Always ensure the tag version is **new** and not yet published.

**Recommended release flow:**

1. Develop on `main` with the current version in `Cargo.toml`
2. Tag with the next version: `git tag v0.2.0 && git push origin v0.2.0`
3. The pipeline publishes `0.2.0` and the post-release PR updates `Cargo.toml` to `0.2.0`
4. Merge the post-release PR so `main` reflects the released version

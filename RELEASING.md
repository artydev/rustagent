# Releasing

This document describes how to build and publish a release of the coding
assistant.

## Versioning

This project follows [Semantic Versioning](https://semver.org/). The current
version is declared in `Cargo.toml` (`[package] version`). Releases are tagged
with a `v` prefix (e.g., `v0.1.0`).

## Changelog

Every release should update [`CHANGELOG.md`](CHANGELOG.md) following the
[Keep a Changelog](https://keepachangelog.com/) convention. Move the relevant
entries from `[Unreleased]` into a new dated release section before tagging.

## Local release build

To build an optimized release binary locally:

```sh
make release          # cargo build --release
```

The binary is written to `target/release/coding-assistant`. The release
profile uses `lto`, `codegen-units = 1`, `strip`, and `panic = "abort"` for a
small, fast binary.

## Packaging

Packaging is automated in [`.github/workflows/release.yml`](.github/workflows/release.yml).
It produces:

| Platform | Tool | Artifact |
|----------|------|----------|
| Windows  | `cargo-wix` | `.msi` installer |
| macOS    | `cargo-bundle` + `hdiutil` | `.app` bundle inside a `.dmg` |
| Linux    | `cargo-deb` | `.deb` package |

The workflow runs on each platform's native runner (Freya GUI apps cannot be
cross-compiled from a single host). It is triggered either manually
(`workflow_dispatch`) or by pushing a `v*` tag.

### Prerequisites

- **Windows**: `cargo-wix` (installed by the workflow). The
  `windows_subsystem = "windows"` attribute in `src/main.rs` makes the release
  binary run as a GUI app without a console window.
- **macOS**: `cargo-bundle` (installed by the workflow). Bundle metadata lives
  in `[package.metadata.bundle]` in `Cargo.toml`.
- **Linux**: `cargo-deb` (installed by the workflow). Package metadata lives in
  `[package.metadata.deb]` in `Cargo.toml`.

## Publishing a release

1. **Update the version** in `Cargo.toml` (e.g., `0.1.0` → `0.2.0`).
2. **Update `CHANGELOG.md`**: move `[Unreleased]` entries into a new dated
   section for the new version.
3. **Commit** the changes.
4. **Tag** the release and push it:
   ```sh
   git tag v0.2.0
   git push origin v0.2.0
   ```
   Pushing the tag triggers the `Release` workflow, which builds and uploads
   the platform artifacts.
5. **Download the artifacts** from the workflow run and attach them to a
   GitHub Release, or publish them through your distribution channel.

## Signing / notarization

Code signing and notarization are **not yet configured**. For public
distribution on Windows and macOS you should:

- **Windows**: sign the MSI with a code-signing certificate (e.g., via
  `signtool` or a CI secret).
- **macOS**: sign and notarize the `.app`/`.dmg` with an Apple Developer ID
  certificate and `notarytool`.

These steps require certificates/credentials and are left as a follow-up.

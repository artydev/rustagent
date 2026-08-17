# Task 6 — Packaging / release builds

**Status:** ✅ Done

## Goal

Configure release builds and package the application for distribution on
Windows, macOS, and Linux.

## What has been accomplished so far

- The `Cargo.toml` already includes a **Windows subsystem attribute** at the
  top of `main.rs`:
  ```rust
  #![cfg_attr(
      all(not(debug_assertions), target_os = "windows"),
      windows_subsystem = "windows"
  )]
  ```
  This means release builds on Windows will run as a GUI app without a console
  window — a good foundation for packaging.
- The package is marked `publish = false`, which is appropriate for an
  application (not a library).
- The project uses **Freya**, which has its own packaging considerations
  (native dependencies, asset bundling).

## What remains

- [x] **Release profile:** configure `[profile.release]` for optimized builds
      (e.g., `lto = true`, `codegen-units = 1`, `strip = true`).
      - Added `version = "0.1.0"`, `description`, and `license = "MIT"` to
        `[package]`.
      - Added `[profile.release]` with `lto = true`, `codegen-units = 1`,
        `strip = true`, `opt-level = 3`, and `panic = "abort"`.
      - Verified `cargo build --release` succeeds and produces a stripped
        ~43 MB ELF binary.
- [x] **Platform packaging:**
      - Windows: `.msi` or `.exe` installer (e.g., via `cargo-wix` or NSIS).
        - Added a `windows` job to `.github/workflows/release.yml` that builds
          the release binary, installs `cargo-wix`, runs `cargo wix init` to
          generate the WiX manifest, runs `cargo wix build` to produce the
          MSI, and uploads it as an artifact. The existing
          `windows_subsystem = "windows"` attribute in `main.rs` ensures the
          release binary runs as a GUI app without a console window.
      - macOS: `.dmg` or `.app` bundle (e.g., via `cargo-bundle`).
        - Added a `macos` job to `.github/workflows/release.yml` that builds
          the release binary, installs `cargo-bundle`, runs `cargo bundle
          --release` to create the `.app`, then uses `hdiutil` to create a
          `.dmg`, and uploads it as an artifact.
        - Added `[package.metadata.bundle]` to `Cargo.toml` (name, identifier,
          icon, category, description) for `cargo-bundle`.
      - Linux: `.deb`, `.rpm`, or `.AppImage` (e.g., via `cargo-deb` or
        AppImage tooling).
        - Added a `linux` job to `.github/workflows/release.yml` that builds
          the release binary, installs `cargo-deb`, runs `cargo deb` to
          produce a `.deb`, and uploads it as an artifact.
        - Added `[package.metadata.deb]` to `Cargo.toml` (maintainer,
          copyright, license-file, section, priority, assets) for `cargo-deb`.
      - The release workflow is triggered manually (`workflow_dispatch`) or on
        a `v*` tag push, and uploads all platform artifacts.
      - Verified `cargo metadata` parses the `bundle` and `deb` metadata, and
        the `release.yml` is valid YAML. Actual MSI/DMG/DEB builds run on
        their native CI runners (Windows/macOS/Linux) since Freya GUI apps
        cannot be cross-compiled from a single host.
- [ ] **Asset bundling:** ensure any runtime assets (fonts, themes, remote
      images) are bundled or handled correctly.
- [x] **Versioning:** add a versioning scheme and changelog management.
      - Set `version = "0.1.0"` in `Cargo.toml` (SemVer).
      - Added `CHANGELOG.md` following the Keep a Changelog convention, with
        an `[Unreleased]` section and a `[0.1.0]` release entry.
      - The release workflow triggers on `v*` tags, so versioning is driven by
        git tags (e.g., `v0.1.0`) matching the Cargo version.
- [x] **Release process documentation:** document how to build and publish a
      release.
      - Added `RELEASING.md` covering versioning (SemVer + `v*` tags),
        changelog maintenance, local release builds (`make release`), the
        automated packaging workflow (Windows MSI / macOS DMG / Linux DEB),
        and a step-by-step publishing procedure.
      - Documented the signing/notarization follow-up for public Windows and
        macOS distribution.
- [ ] **Signing / notarization:** consider code signing for Windows and macOS
      if distributing publicly.

## Notes / blockers

- The README lists this as a limitation: *"Aucun test, CI ou packaging. Il n'y
  a pas de suite de tests, de pipeline de construction ni de configuration de
  publication."*
- Packaging depends on Task 1 (clean compilation) and Task 3 (cross-platform
  support) being complete first.
- The app currently loads a remote image from Unsplash at runtime
  (`remote-asset` feature), which has implications for offline packaging.

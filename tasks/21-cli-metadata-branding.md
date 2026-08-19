# Task 21 — Add CLI metadata, version flag, and app branding

**Status:** 🔴 Not started

## Goal

Wire the package version into a `--version` flag and the window title, and add
app icon/branding assets for the packaged builds.

## Why

The release pipeline packages binaries but there is no `--version` flag, no
version in the window title, and no app icon. These are standard for a
shippable desktop app.

## Steps

- [ ] **Step 1 — Add a `--version` CLI flag.**
      Parse command-line arguments and print the crate version (from
      `env!("CARGO_PKG_VERSION")`) when `--version` or `-V` is passed, then
      exit.
      *Testable:* Running the binary with `--version` prints `0.1.0` (or the
      current `Cargo.toml` version) and exits with code 0.

- [ ] **Step 2 — Add a `--help` flag.**
      Print a short usage message when `--help` or `-h` is passed.
      *Testable:* Running the binary with `--help` prints usage text and exits
      with code 0.

- [ ] **Step 3 — Show the version in the window title.**
      Set the Freya window title to include the version, e.g.
      `RustAgent v0.1.0`.
      *Testable:* A test (or code review) confirms the title string is built
      from `CARGO_PKG_VERSION`.

- [ ] **Step 4 — Add an app icon asset.**
      Add an icon file (e.g. `src/assets/icon.png` or `.ico`/`.icns`) and
      reference it in the `[package.metadata.bundle]` and `[package.metadata.deb]`
      sections of `Cargo.toml`.
      *Testable:* `cargo metadata` parses the bundle/deb metadata and the icon
      path resolves to an existing file.

- [ ] **Step 5 — Verify the release build still works.**
      *Testable:* `cargo build --release` succeeds and produces a binary that
      responds to `--version`.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes (total count increases above 81) and
      `cargo clippy --all-targets -- -D warnings` is clean.

## Files changed

- `src/main.rs`
- `Cargo.toml`
- `src/assets/` (new icon asset)

# Task 2 — Add a workspace root / `Cargo.lock` and pin dependency versions

**Status:** 🟢 Done

## Goal

Make builds reproducible by adding a workspace root, committing a
`Cargo.lock`, and pinning dependency versions.

## What has been accomplished

- ✅ **Added a `[workspace]` section to `Cargo.toml`:**
  ```toml
  [workspace]
  members = ["."]
  resolver = "2"
  ```
  This makes the project a proper single-crate Cargo workspace.

- ✅ **Generated a `Cargo.lock` file** (624 packages locked). The lockfile
  pins exact versions of all direct and transitive dependencies, including:
  - `freya 0.5.0-rc.3`
  - `rig-core 0.32.0`
  - `tokio 1.53.1`
  - `ropey`, `rio-vt`, all `tree-sitter-*` grammars, etc.

- ✅ **Decided `Cargo.lock` should be committed.** Since this is an
  **application** (not a library), the lockfile must be committed to ensure
  reproducible builds. This decision is documented in `.gitignore`.

- ✅ **Verified reproducible builds:**
  - `cargo build --locked` succeeds — confirms the lockfile is fully
    consistent with `Cargo.toml` (with `--locked`, Cargo fails if the
    lockfile needs updating).
  - `cargo update --dry-run` reports "Locking 0 packages" — nothing is out
    of date.
  - `cargo metadata` confirms the workspace root and member are correct.

- ✅ **Reviewed `.gitignore`:** removed the `Cargo.lock` exclusion and added
  a comment explaining why it must be committed for an application.

## Verification details

- `cargo build --locked` → `Finished dev profile` (no errors)
- `cargo update --dry-run` → `Locking 0 packages`
- `cargo metadata` → workspace_root `/tmp/rustagent-main`, 1 package
  (`coding-assistant`), 1 workspace member
- `Cargo.lock` → 624 packages, version 4 format

## Notes

- The `Cargo.lock` was originally generated during the Task 1 build and is
  now ready to be committed (no longer gitignored).
- The `[workspace]` section uses `resolver = "2"`, which is the modern
  resolver and matches the `edition = "2024"` package.

# Task 24 — Cross-platform runtime verification (Windows & macOS)

**Status:** 🔴 Not started

## Goal

Actually exercise the cross-platform logic (`src/platform.rs`) on Windows and
macOS, since it was only validated on Linux.

## Why

The cross-platform shell selection and execution commands exist but have only
been run on Linux. The README itself notes the Windows/macOS paths are
unverified. Real verification prevents shipping broken platform code.

## Steps

- [ ] **Step 1 — Add a CI matrix for Windows and macOS.**
      Extend `.github/workflows/ci.yml` to run `cargo build` and `cargo test`
      on `windows-latest` and `macos-latest` runners in addition to Linux.
      *Testable:* The CI workflow has a matrix with `ubuntu-latest`,
      `windows-latest`, and `macos-latest`, and all three pass.

- [ ] **Step 2 — Verify shell selection per platform.**
      Confirm `src/platform.rs` selects `cmd.exe`/PowerShell on Windows and
      `bash` on macOS/Linux.
      *Testable:* Unit tests (or CI logs) assert the correct shell command is
      chosen for each target platform.

- [ ] **Step 3 — Verify execution commands per platform.**
      Confirm the run commands use platform-appropriate temp paths and
      interpreters (e.g. `%TEMP%` on Windows, `/tmp` on Unix).
      *Testable:* Unit tests assert the temp path and command differ correctly
      per platform, and CI runs the tests on all three OSes.

- [ ] **Step 4 — Verify the terminal launches on Windows/macOS.**
      Add a CI smoke test (or documented manual check) that the terminal panel
      launches the correct shell on each OS.
      *Testable:* CI (or a manual checklist entry) confirms the terminal starts
      on Windows and macOS.

- [ ] **Step 5 — Document platform-specific limitations.**
      Update the README's limitations section to reflect what was actually
      verified on each platform.
      *Testable:* The README accurately states which platforms were exercised
      and any remaining gaps.

- [ ] **Step 6 — Verify the full suite still passes on all platforms.**
      *Testable:* `cargo test` passes on Linux, Windows, and macOS CI runners.

## Files changed

- `.github/workflows/ci.yml`
- `src/platform.rs` (if fixes are needed)
- `README.md`

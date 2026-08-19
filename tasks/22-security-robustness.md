# Task 22 — Security and robustness hardening

**Status:** 🔴 Not started

## Goal

Harden the app against common input and interaction issues: sanitize the API
key before saving, debounce the Enter-key send to prevent double-submits, and
confirm before destructive terminal actions.

## Why

These are low-effort, high-value robustness improvements that prevent
frustrating or unsafe behavior in daily use.

## Steps

- [ ] **Step 1 — Sanitize the API key before saving.**
      Trim whitespace and reject empty/obviously-invalid keys before writing to
      the config file from the settings panel.
      *Testable:* A unit test saves a key with surrounding whitespace and
      asserts the stored value is trimmed; a test with an empty key asserts it
      is rejected.

- [ ] **Step 2 — Debounce / guard the Enter-key send.**
      Prevent a double-submit when the user presses Enter rapidly or holds the
      key, by ignoring a send while a request is already in flight.
      *Testable:* A unit test triggers two sends while one is in flight and
      asserts only one request is issued.

- [ ] **Step 3 — Confirm before resetting the terminal.**
      Show a confirmation dialog before the "Réinitialiser le terminal" action
      kills and restarts the shell.
      *Testable:* A test (or code review) confirms the reset path requires an
      explicit confirmation before executing.

- [ ] **Step 4 — Validate the API key format at startup.**
      Add a lightweight format check (non-empty, no whitespace, reasonable
      length) at startup in addition to the existing validation.
      *Testable:* A unit test feeds malformed keys and asserts a clear warning
      is produced.

- [ ] **Step 5 — Sanitize the editor content before writing temp files.**
      Ensure the derived file name (`flow::derive_file_name`) cannot contain
      path separators or other unsafe characters that could escape the temp
      directory.
      *Testable:* A unit test feeds a script whose derived name contains `/`,
      `..`, or other unsafe characters and asserts the sanitized name stays
      within the temp directory.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes (total count increases above 81) and
      `cargo clippy --all-targets -- -D warnings` is clean.

## Files changed

- `src/main.rs`
- `src/config.rs`
- `src/flow.rs`

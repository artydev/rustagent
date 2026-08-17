# RustAgent — Improvement Requirements

> This document lists the tasks and requirements needed to take **RustAgent**
> from its current prototype / proof-of-concept state to a buildable, testable,
> cross-platform, distributable product.

---

## 1. Make the project compile cleanly

**Status:** ❌ Not done

**Description:**
The project currently does **not** compile cleanly in a standalone checkout.
The code relies on a specific set of workspace dependencies and Freya features
that may not resolve outside the original workspace.

**Requirements:**
- [ ] Verify the project builds with `cargo build` in a fresh, standalone checkout.
- [ ] Ensure all dependencies resolve without workspace-level configuration.
- [ ] Fix any compilation errors that arise from the current dependency set.
- [ ] Confirm `cargo run` launches the application successfully.

---

## 2. Add a workspace root / `Cargo.lock` and pin dependency versions

**Status:** ❌ Not done

**Description:**
There is currently no committed `Cargo.lock` or workspace root, and dependency
versions are not pinned. This makes builds non-reproducible.

**Requirements:**
- [ ] Add a workspace root (`Cargo.toml` with `[workspace]`).
- [ ] Commit a `Cargo.lock` to lock dependency versions.
- [ ] Pin exact versions of all dependencies (or use lockfile-based resolution).
- [ ] Verify reproducible builds across environments.

---

## 3. Cross-platform support for terminal and execution commands

**Status:** ❌ Not done

**Description:**
The integrated terminal launches `bash`, and several execution commands use
Unix-only paths and tools (`/tmp/...`, `xdg-open`, `python3`, `gcc`, `g++`,
`javac`, `go`, `node`, `npx`). These will not work on Windows without adaptation.

**Requirements:**
- [ ] Support **Windows** (e.g., `cmd.exe` / PowerShell terminal, Windows paths).
- [ ] Support **macOS** (verify Unix commands and paths work correctly).
- [ ] Abstract platform-specific execution commands behind a platform layer.
- [ ] Handle platform-specific temporary file locations.
- [ ] Handle platform-specific "open file" behavior (e.g., `xdg-open` vs `open` vs `start`).

---

## 4. Proper API key configuration and error handling

**Status:** ❌ Not done

**Description:**
The application reads `ALBERT_API_KEY` from the environment at runtime, but
there is no configuration UI, validation, or progressive integration. Missing
or invalid keys are not handled gracefully.

**Requirements:**
- [ ] Add a configuration mechanism for the API key (env var, config file, or UI).
- [ ] Validate the API key before/at startup.
- [ ] Provide clear, user-friendly error messages when the key is missing or invalid.
- [ ] Handle API errors (timeouts, rate limits, network failures) gracefully.
- [ ] Add progressive integration / retry logic where appropriate.

---

## 5. Tests and CI

**Status:** ❌ Not done

**Description:**
There is currently no test suite or continuous integration pipeline.

**Requirements:**
- [ ] Add unit tests for core logic (language detection, code extraction, etc.).
- [ ] Add integration tests for the chat → editor → terminal flow.
- [ ] Set up a CI pipeline (e.g., GitHub Actions) that runs on every push/PR.
- [ ] CI should run `cargo build`, `cargo test`, and `cargo clippy`.
- [ ] Add linting / formatting checks (`cargo fmt --check`).

---

## 6. Packaging / release builds

**Status:** ❌ Not done

**Description:**
There is no packaging or release configuration for distributing the application.

**Requirements:**
- [ ] Configure release builds (optimized, stripped binaries).
- [ ] Package the application for target platforms (Windows, macOS, Linux).
- [ ] Add installers / distributable artifacts (e.g., `.msi`, `.dmg`, `.deb`/`.AppImage`).
- [ ] Document the release process.
- [ ] Add versioning and changelog management.

---

## 7. End-to-end validation of the chat → editor → terminal flow

**Status:** ❌ Not done

**Description:**
The chat, editor, and terminal panels are wired together but have not been
exercised end-to-end. The full user flow has not been validated.

**Requirements:**
- [ ] Validate the full flow: user message → AI response → code extraction → editor insertion.
- [ ] Validate the execution flow: editor content → temp file → terminal execution.
- [ ] Test all supported languages end-to-end.
- [ ] Verify the "clear editor" command works as expected.
- [ ] Verify toolbar actions (clear chat, reset terminal, run code) work correctly.
- [ ] Test edge cases (empty editor, no code in response, unsupported language, etc.).

---

## 8. Send chat messages on Enter key or Send button

**Status:** ❌ Not done

**Description:**
The chat input currently only sends a message when the **Send** button is
pressed. There is no keyboard shortcut, so a user must click the button every
time. The expected behaviour is that pressing the **Enter** key in the chat
input field sends the message to the LLM, exactly as pressing the **Send**
button does.

**Requirements:**
- [ ] Pressing **Enter** in the chat input field sends the current message to
      the LLM (same behaviour as the Send button).
- [ ] The **Send** button continues to work as before (sends the message).
- [ ] Both triggers share the same send handler / code path (no duplication).
- [ ] Empty or whitespace-only messages are ignored (not sent) in both cases.
- [ ] The input field is cleared after sending, in both cases.
- [ ] The Enter key does not interfere with multi-line input (if the input
      field supports it) or with other keyboard shortcuts.
- [ ] Add tests covering the shared send logic (Enter and button both trigger
      the same behaviour).

---

## Summary

| # | Task | Status |
|---|------|--------|
| 1 | Compile cleanly in a standalone checkout | ❌ |
| 2 | Workspace root / `Cargo.lock` / pinned versions | ❌ |
| 3 | Cross-platform terminal & execution commands | ❌ |
| 4 | Proper API key configuration & error handling | ❌ |
| 5 | Tests and CI | ❌ |
| 6 | Packaging / release builds | ❌ |
| 7 | End-to-end validation of chat → editor → terminal | ❌ |
| 8 | Send chat messages on Enter key or Send button | ❌ |

---

*Source: [README.md](README.md) — Roadmap section.*

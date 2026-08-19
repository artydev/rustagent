# Task 16 — Add automated tests for the GUI layer

**Status:** 🔴 Not started

## Goal

Add automated tests for the Freya GUI components and their shared-state
interactions, which currently have no test coverage.

## Why

The 81 existing tests cover pure logic (language detection, code extraction,
command generation, config parsing, file-tree scanning) but the UI panels
(chat, editor, terminal, file-tree, settings) and the shared-state wiring
between them are untested. This is the largest coverage gap.

## Steps

- [ ] **Step 1 — Extract testable state logic out of the UI components.**
      Identify the state transitions in `src/main.rs` that can be tested
      without rendering (e.g. the `current_dir` update from terminal output,
      the `current_file_name` update when code is inserted, the
      `should_send_message` gate, the settings save/load). Move each into a
      pure function or a small state struct in a testable module.
      *Testable:* Each extracted function has a `#[cfg(test)]` module or is
      covered by a `tests/` integration test.

- [ ] **Step 2 — Add tests for the chat send state logic.**
      Test that a non-empty message triggers the send path, an empty/whitespace
      message does not, and the input is cleared after a send.
      *Testable:* New tests pass and cover the empty, whitespace, and non-empty
      cases.

- [ ] **Step 3 — Add tests for the terminal → file-tree directory sync.**
      Test that when the terminal reports a new working directory (OSC 7
      output), the shared `current_dir` state updates, and that the file-tree
      scan reflects the new directory.
      *Testable:* A test feeds a fake OSC 7 path and asserts `current_dir`
      equals the expected value.

- [ ] **Step 4 — Add tests for the editor title update.**
      Test that inserting code updates `current_file_name` via
      `flow::derive_file_name` for each supported language.
      *Testable:* New tests assert the title matches the derived file name for
      all 10 languages.

- [ ] **Step 5 — Add tests for the settings panel API-key save/load.**
      Test that saving a key writes to the config file and that loading reads
      it back, with the env-var > config-file > UI priority order preserved.
      *Testable:* New tests write a temp config file, save a key, and assert it
      loads back correctly.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes with the new tests included (total count
      increases above 81), and `cargo clippy --all-targets -- -D warnings` is
      clean.

## Files changed

- `src/main.rs` (extract state logic into testable functions)
- New test modules or `tests/` files for the extracted logic

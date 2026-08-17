# Task 5 — Tests and CI

**Status:** ✅ Done

## Goal

Add a test suite and a continuous integration pipeline to validate the
project automatically.

## What has been accomplished so far

- The codebase contains several **pure, testable functions** that are good
  candidates for unit tests:
  - `SupportedLanguage::detect()` — language detection from a user message.
  - `SupportedLanguage::extension()` / `file_name()` — file naming logic.
  - `SupportedLanguage::run_command()` — execution command generation.
  - `extract_code_from_response()` — markdown code-block extraction.
  - `looks_like_code()` — heuristic for inline code detection.
  - `language_name()` — human-readable language names.
- These functions are **pure** (no I/O, no UI state), which makes them easy to
  test in isolation.
- The project uses `thiserror` for error types, which is a good foundation for
  structured error handling in tests.
- The pure chat → editor → terminal flow logic was extracted from `main.rs`
  into a new `src/flow.rs` module. This module is UI-independent and contains
  integration-style tests that simulate the full flow: detect language → decide
  whether to generate code → extract code from the AI response → build the temp
  file and run command. See `full_flow_python_chat_to_terminal`,
  `full_flow_rust_chat_to_terminal`, `full_flow_clear_editor_short_circuits`,
  and `full_flow_plain_question_does_not_touch_editor`.

## What remains

- [x] **Unit tests** for the pure functions listed above (language detection,
      code extraction, command generation, etc.).
- [x] **Integration tests** for the chat → editor → terminal flow (may require
      refactoring to separate UI from logic).
- [x] **CI pipeline** (e.g., GitHub Actions) that runs on every push/PR:
      - `cargo build`
      - `cargo test`
      - `cargo clippy` (linting)
      - `cargo fmt --check` (formatting)
      - Added `.github/workflows/ci.yml` with three jobs: `fmt`, `clippy`, and
        `test` (build + test). Uses `dtolnay/rust-toolchain@stable` and
        `Swatinem/rust-cache@v2` for caching, and installs the system
        dependencies (`libgtk-3-dev`, `libxdo-dev`, `libssl-dev`) needed to
        build the Freya GUI on Linux.
      - Fixed `cargo fmt` drift in `src/main.rs` (ran `cargo fmt --all`).
      - Fixed a `clippy::explicit-counter-loop` warning in `src/flow.rs` by
        replacing a manual loop counter with `.enumerate()`.
      - Verified locally that `cargo fmt --check`, `cargo clippy --all-targets
        --all-features -- -D warnings`, `cargo build`, and `cargo test` (44
        tests) all pass.
- [x] Add `#[cfg(test)]` modules or a `tests/` directory.
      - The codebase already uses `#[cfg(test)]` modules in four source files:
        `src/main.rs` (18 tests), `src/flow.rs` (16 tests), `src/api.rs`
        (5 tests), and `src/config.rs` (3 tests).
      - This is a **binary-only** crate (no `[lib]` target), so a `tests/`
        integration-test directory cannot link against the crate. The
        `#[cfg(test)]` unit-test modules are therefore the correct and
        idiomatic choice here, and they already cover the pure logic
        (language detection, code extraction, command generation, API retry,
        config parsing) plus the chat → editor → terminal flow.
- [x] Consider adding a `Makefile` or `justfile` for local dev commands.
      - Added a `Makefile` with targets: `build`, `run`, `test`, `lint`
        (clippy -D warnings), `fmt`, `fmt-check`, `check` (runs fmt-check +
        lint + test, mirroring CI), `release`, and `clean`.
      - Verified locally that `make fmt-check`, `make lint`, and `make check`
        all pass (44 tests).

## Notes / blockers

- The README lists this as a limitation: *"Aucun test, CI ou packaging. Il n'y
  a pas de suite de tests, de pipeline de construction ni de configuration de
  publication."*
- The UI code (Freya components) is harder to test; the pure logic functions
  should be tested first, and the UI flow validated via integration tests or
  manual testing.
- No CI config file (e.g., `.github/workflows/`) exists yet.

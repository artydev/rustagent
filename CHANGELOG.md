# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of the coding assistant GUI.
- Chat → editor → terminal flow: natural-language requests are turned into
  runnable code in an editor and executed in a terminal.
- Support for 10 languages: Python, Rust, JavaScript, TypeScript, HTML, CSS,
  C, C++, Java, and Go.
- API key configuration via UI, environment variable, or config file, with
  validation, retry, and timeout handling.
- Cross-platform support for terminal shell selection and execution commands
  (Windows, macOS, Linux).
- Unit and integration test suite (58 tests) covering language detection,
  code extraction, command generation, the chat → editor → terminal flow,
  API retry logic, and config parsing.
- End-to-end validation of the chat → editor → terminal flow, including a
  manual testing checklist (`E2E_TESTING.md`) covering startup & API key
  configuration, chat → editor and editor → terminal flows for all 10
  languages, and edge cases.
- Continuous integration pipeline (`.github/workflows/ci.yml`) running
  `cargo fmt --check`, `cargo clippy -D warnings`, `cargo build`, and
  `cargo test` on every push/PR.
- Release pipeline (`.github/workflows/release.yml`) producing Windows MSI,
  macOS DMG, and Linux DEB packages.
- Optimized release profile (`lto`, `codegen-units = 1`, `strip`, `panic =
  "abort"`).
- Local development `Makefile` with `build`, `run`, `test`, `lint`, `fmt`,
  `fmt-check`, `check`, `release`, and `clean` targets.

## [0.1.0] - 2026-08-17

### Added
- First tagged release. Includes the full feature set described under
  [Unreleased] above.

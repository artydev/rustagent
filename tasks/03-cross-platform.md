# Task 3 — Cross-platform support for terminal and execution commands

**Status:** ✅ Done

## Goal

Make the integrated terminal and the code-execution commands work on Windows,
macOS, and Linux.

## What has been accomplished so far

- The terminal and execution logic is **centralized** in a single place, which
  makes it easier to adapt for cross-platform support:
  - `spawn_terminal()` — spawns the shell.
  - `SupportedLanguage::run_command()` — returns the shell command for each
    language.
  - `execute_code` handler — writes the editor content to a temp file and runs
    the language command.
- The temp file path is already written using `std::env::temp_dir()`, which is
  **platform-agnostic** (returns the correct temp directory on Windows, macOS,
  and Linux). This is a good foundation.
- The code uses `std::path::Path` for file paths rather than hardcoded strings,
  which is portable.

## What remains

- [x] **Terminal shell:** currently hardcoded to `bash` in `spawn_terminal()`.
      Needs to select `cmd.exe` / PowerShell on Windows and `bash`/`zsh` on
      Unix.
- [x] **Execution commands:** `run_command()` uses Unix-only tools and paths:
      - `python3` → needs `python` on Windows.
      - `rustc ... -o /tmp/main_rs` → `/tmp` is Unix-only; use `temp_dir()`.
      - `npx ts-node` → verify availability on all platforms.
      - `xdg-open` (HTML) → needs `open` on macOS and `start` on Windows.
      - `gcc` / `g++` → need Windows equivalents (e.g., MinGW) or MSVC.
      - `javac` / `java` → generally portable but path handling differs.
      - `go run` → portable.
- [x] **Environment variables:** `spawn_terminal()` sets `TERM`,
      `COLORTERM`, and `LANG` — these are Unix-specific and may need
      conditional handling on Windows.
- [x] Abstract platform-specific behavior behind a small platform layer
      (e.g., a `platform` module) rather than inline `cfg!` checks scattered
      through the code.

## What was done

A new `src/platform.rs` module centralizes all platform-specific behavior:

- `terminal_shell()` — returns `powershell.exe` on Windows, `bash` on Unix.
- `terminal_env()` — returns `TERM`/`COLORTERM`/`LANG` only on non-Windows.
- `python_command()` — `python` on Windows, `python3` on Unix.
- `open_command(file)` — `start "" "file"` on Windows, `open "file"` on
  macOS, `xdg-open "file"` on Linux.
- `c_compiler()` / `cpp_compiler()` / `java_compiler()` / `java_runtime()` /
  `go_runner()` / `node_runner()` / `ts_runner()` / `rust_compiler()` —
  named accessors for each tool.

`main.rs` was refactored to use the module:

- `spawn_terminal()` now uses `platform::terminal_shell()` and applies
  `platform::terminal_env()`.
- `SupportedLanguage::run_command()` now uses the platform accessors and
  writes compiled binaries to `std::env::temp_dir()` instead of hardcoded
  `/tmp` paths.
- `execute_code` already used `std::env::temp_dir()` for the source file,
  which is consistent with the compiled-binary paths.

## Notes / blockers

- The README explicitly lists this as a limitation: *"Le terminal lance `bash`
  et plusieurs commandes d'exécution utilisent des chemins Unix (`/tmp/...`,
  `xdg-open`, `python3`, `gcc`, `g++`, `javac`, `go`, `node`, `npx`). Celles-ci
  ne fonctionneront pas sur Windows sans adaptation."* — this limitation is
  now addressed.
- The `#![cfg_attr(... windows_subsystem = "windows")]` attribute at the top
  of `main.rs` shows Windows is already being considered, but the runtime
  logic is not yet adapted.

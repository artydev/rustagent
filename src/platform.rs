//! Cross-platform helpers for the integrated terminal and code execution.
//!
//! All platform-specific behavior (shell selection, environment variables,
//! executable names, and file-opening commands) is centralized here so that
//! the rest of the application can stay platform-agnostic.

/// The shell used to spawn the integrated terminal.
///
/// - Windows: PowerShell (widely available and scriptable).
/// - Unix (Linux/macOS): bash.
pub fn terminal_shell() -> &'static str {
    if cfg!(windows) {
        "powershell.exe"
    } else {
        "bash"
    }
}

/// Environment variables to set on the terminal shell.
///
/// `TERM`, `COLORTERM`, and `LANG` are Unix-specific and are only set on
/// non-Windows platforms. Windows terminals do not use these variables.
pub fn terminal_env() -> Vec<(&'static str, &'static str)> {
    let mut env = Vec::new();
    if !cfg!(windows) {
        env.push(("TERM", "xterm-256color"));
        env.push(("COLORTERM", "truecolor"));
        env.push(("LANG", "en_GB.UTF-8"));
    }
    env
}

/// The Python interpreter command.
///
/// - Windows: `python` (the standard launcher).
/// - Unix: `python3`.
pub fn python_command() -> &'static str {
    if cfg!(windows) { "python" } else { "python3" }
}

/// Build the command that opens a file with the default application.
///
/// - Windows: `start "" "file"`.
/// - macOS: `open "file"`.
/// - Linux: `xdg-open "file"`.
pub fn open_command(file: &str) -> String {
    if cfg!(windows) {
        format!("start \"\" \"{}\"\n", file)
    } else if cfg!(target_os = "macos") {
        format!("open \"{}\"\n", file)
    } else {
        format!("xdg-open \"{}\"\n", file)
    }
}

/// The C compiler command.
///
/// - Windows: `gcc` (assumes MinGW or similar in PATH).
/// - Unix: `gcc`.
pub fn c_compiler() -> &'static str {
    "gcc"
}

/// The C++ compiler command.
///
/// - Windows: `g++` (assumes MinGW or similar in PATH).
/// - Unix: `g++`.
pub fn cpp_compiler() -> &'static str {
    "g++"
}

/// The Java compiler command.
pub fn java_compiler() -> &'static str {
    "javac"
}

/// The Java runtime command.
pub fn java_runtime() -> &'static str {
    "java"
}

/// The Go runner command.
pub fn go_runner() -> &'static str {
    "go"
}

/// The Node.js runner command.
pub fn node_runner() -> &'static str {
    "node"
}

/// The TypeScript runner command (via ts-node).
pub fn ts_runner() -> &'static str {
    "npx ts-node"
}

/// The Rust compiler command.
pub fn rust_compiler() -> &'static str {
    "rustc"
}

/// The directory where the application stores its configuration files.
///
/// - Windows: `%APPDATA%\rustagent`
/// - macOS: `$HOME/Library/Application Support/rustagent`
/// - Linux: `$XDG_CONFIG_HOME/rustagent` or `$HOME/.config/rustagent`
pub fn config_dir() -> std::path::PathBuf {
    if cfg!(windows) {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            return std::path::PathBuf::from(appdata).join("rustagent");
        }
    } else if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            return std::path::PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("rustagent");
        }
    } else {
        // Linux / other Unix
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            return std::path::PathBuf::from(xdg).join("rustagent");
        }
        if let Some(home) = std::env::var_os("HOME") {
            return std::path::PathBuf::from(home)
                .join(".config")
                .join("rustagent");
        }
    }
    // Fallback: current directory
    std::path::PathBuf::from(".rustagent")
}

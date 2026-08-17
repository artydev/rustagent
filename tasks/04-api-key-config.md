# Task 4 — Proper API key configuration and error handling

**Status:** ✅ Done

## Goal

Provide a proper configuration mechanism for the Albert API key, validate it,
and handle API errors gracefully.

## What has been accomplished

### 1. Configuration mechanism (UI / file / env)

The API key can now be provided through three channels, in order of precedence:

1. **Environment variable** `ALBERT_API_KEY` (highest priority).
2. **Config file** at the platform config directory:
   - Linux: `~/.config/rustagent/config.toml`
   - macOS: `~/Library/Application Support/rustagent/config.toml`
   - Windows: `%APPDATA%\rustagent\config.toml`
3. **Settings UI:** a new **Settings** button in the toolbar opens a modal
   panel where the user can enter and save an API key to the config file.

New module: `src/config.rs` (`ApiKeyConfig`, `FileConfig`, `KeySource`).
New helper: `platform::config_dir()`.

### 2. Validation at startup

`ApiKeyConfig::validate()` checks that the key is non-empty and contains no
whitespace. At startup, `app()` loads the config and, if the key is missing or
invalid, pushes a clear warning message into the chat immediately (rather than
failing silently on the first message).

### 3. Clear error messages for missing/invalid keys

- A startup warning explains how to configure the key (Settings button, env
  var, or config file path).
- If a message is sent without a valid key, the chat shows a clear message
  instead of a generic failure.

### 4. Graceful API error handling

New module: `src/api.rs`.

`ApiErrorCategory` distinguishes:
- `Authentication` (401/403, invalid/expired key)
- `RateLimit` (429, too many requests)
- `Network` (connection refused/reset, DNS, unreachable)
- `Timeout` (request exceeded the deadline)
- `Model` (provider/model errors)
- `Other`

Each category has a human-friendly `user_message()`. `classify_error()` maps a
raw error string to a category.

### 5. Retry / progressive integration

`prompt_with_retry()` wraps the completion call with:
- Up to 3 attempts.
- Exponential backoff (500ms base, capped at 4s) between retries.
- Retries only for **transient** failures (network, timeout, rate limit).
- Non-transient failures (e.g. auth) fail fast without retrying.

### 6. Request timeout handling

Each request is wrapped in `tokio::time::timeout(REQUEST_TIMEOUT)` (60s) so the
UI never hangs indefinitely. A timeout is classified as `Timeout` and retried
as a transient failure.

## Tests

Unit tests added in `src/config.rs` and `src/api.rs`:
- `validate_rejects_empty`, `validate_rejects_whitespace`, `validate_accepts_valid_key`
- `classifies_auth`, `classifies_rate_limit`, `classifies_network`,
  `classifies_timeout`, `classifies_unknown_as_other`
- `retries_transient_then_succeeds`, `does_not_retry_non_transient`

All 10 tests pass; `cargo build` and `cargo clippy` are clean (no new warnings).

## Notes

- The README limitation about API key handling is now resolved.
- The `KeySource` field records where the key came from (env vs file) for
  future user-facing reporting.

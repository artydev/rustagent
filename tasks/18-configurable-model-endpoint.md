# Task 18 — Make the AI model and endpoint configurable

**Status:** 🔴 Not started

## Goal

Make `ALBERT_MODEL` and `ALBERT_ENDPOINT` configurable through the same
channels as the API key (environment variable, config file, settings panel)
instead of being hardcoded constants.

## Why

The model and endpoint are currently hardcoded at the top of `src/main.rs`.
Making them configurable is a natural extension of the existing config system
and lets users switch models or endpoints without recompiling.

## Steps

- [ ] **Step 1 — Add model/endpoint fields to the config struct.**
      In `src/config.rs`, add `model` and `endpoint` fields to the config
      struct with sensible defaults (`deepseek-v4-flash` and
      `https://albert.api.etalab.gouv.fr/v1`).
      *Testable:* A unit test asserts the defaults are applied when the config
      file omits these fields.

- [ ] **Step 2 — Read model/endpoint from environment variables.**
      Support `ALBERT_MODEL` and `ALBERT_ENDPOINT` env vars with the same
      priority as `ALBERT_API_KEY` (env var > config file > default).
      *Testable:* A unit test sets the env var and asserts the config resolves
      to the env value.

- [ ] **Step 3 — Read model/endpoint from the config file.**
      Parse `model` and `endpoint` keys from `config.toml`.
      *Testable:* A unit test writes a temp config file with custom model and
      endpoint and asserts they are loaded.

- [ ] **Step 4 — Add model/endpoint fields to the settings panel.**
      Add inputs in the settings panel so the user can edit and save the model
      and endpoint alongside the API key.
      *Testable:* A test (or manual check) confirms saving from the panel
      writes the values to the config file.

- [ ] **Step 5 — Replace the hardcoded constants in `src/main.rs`.**
      Use the resolved config values for the model and endpoint instead of the
      `const` declarations.
      *Testable:* `grep -n "ALBERT_MODEL\|ALBERT_ENDPOINT" src/main.rs` shows
      the constants are no longer used as the source of truth.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes (total count increases above 81) and
      `cargo clippy --all-targets -- -D warnings` is clean.

## Files changed

- `src/config.rs`
- `src/main.rs`
- `src/api.rs` (if it reads the endpoint/model)

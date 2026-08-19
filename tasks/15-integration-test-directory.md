# Task 15 — Add a dedicated `tests/` integration test directory

**Status:** 🔴 Not started

## Goal

Move integration-style tests out of the `#[cfg(test)]` modules inside `src/`
and into a dedicated `tests/` directory that exercises the public API surface
as an external consumer would.

## Why

Currently tests live inside `src/main.rs`, `src/flow.rs`, `src/api.rs`,
`src/config.rs`, and `src/file_tree.rs`. A dedicated `tests/` directory
improves test isolation, reduces per-crate compile time, and makes the suite
more maintainable as the codebase grows.

## Steps

- [ ] **Step 1 — Identify which tests are integration-style vs unit-style.**
      Review the existing `#[cfg(test)]` modules. Integration-style tests
      (those that exercise `flow::`, `api::`, `config::`, `file_tree::` public
      functions end-to-end) are candidates for `tests/`. Pure internal helpers
      that need private access stay as unit tests.
      *Testable:* Produce a list of test names and classify each as
      "move to tests/" or "keep as unit test".

- [ ] **Step 2 — Create the `tests/` directory and a first integration test file.**
      Create `tests/flow_integration.rs` that imports the crate's public API
      (e.g. `use rustagent::flow::...`) and re-hosts the chat → editor →
      terminal flow tests, including the all-10-languages full-flow test.
      *Testable:* `cargo test --test flow_integration` compiles and runs.

- [ ] **Step 3 — Move the API retry/config/file-tree integration tests.**
      Create `tests/api_integration.rs`, `tests/config_integration.rs`, and
      `tests/file_tree_integration.rs` and move the corresponding
      integration-style tests there.
      *Testable:* `cargo test` runs the new test binaries and all moved tests
      pass.

- [ ] **Step 4 — Ensure the crate exposes a public API for the tests.**
      If `flow`, `api`, `config`, or `file_tree` modules are not `pub`, make
      them public (or add a `pub use` re-export) so the integration tests can
      import them.
      *Testable:* `cargo test` compiles the `tests/` binaries without
      "private module" errors.

- [ ] **Step 5 — Remove the moved tests from the `#[cfg(test)]` modules.**
      Delete the duplicated tests from `src/` so each test exists in exactly
      one place.
      *Testable:* `grep -rn "fn <moved_test_name>" src/` returns nothing for
      each moved test.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes with the same total test count as before
      the move (81 tests), and `cargo clippy --all-targets -- -D warnings` is
      clean.

## Files changed

- `tests/flow_integration.rs` (new)
- `tests/api_integration.rs` (new)
- `tests/config_integration.rs` (new)
- `tests/file_tree_integration.rs` (new)
- `src/flow.rs`, `src/api.rs`, `src/config.rs`, `src/file_tree.rs` (remove moved tests)
- `src/main.rs` (remove moved tests, if any)
- `src/lib.rs` or `Cargo.toml` (expose public API if needed)

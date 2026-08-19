# Task 23 — Documentation and developer experience

**Status:** 🔴 Not started

## Goal

Add a `CONTRIBUTING.md`, doc comments on the public functions, and `cargo doc`
generation to CI to improve the developer experience.

## Why

The project has good README and release docs but no contribution guide, no
doc comments on the public API, and no doc build in CI. These make the project
easier to onboard and maintain.

## Steps

- [ ] **Step 1 — Add a `CONTRIBUTING.md`.**
      Document how to set up the dev environment, run the Makefile targets,
      run tests, and open a PR. Reference the existing `E2E_TESTING.md` and
      `tasks/` tracker.
      *Testable:* The file exists and links to the Makefile targets and the
      task tracker.

- [ ] **Step 2 — Add doc comments to public functions.**
      Add `///` doc comments to the public functions in `src/flow.rs`,
      `src/api.rs`, `src/config.rs`, and `src/file_tree.rs` describing their
      purpose, arguments, and return values.
      *Testable:* `cargo doc --no-deps` builds without warnings about missing
      docs (if `#![warn(missing_docs)]` is enabled) or a manual review confirms
      every public function has a doc comment.

- [ ] **Step 3 — Add `cargo doc` to CI.**
      Add a job (or step) to `.github/workflows/ci.yml` that runs
      `cargo doc --no-deps` and fails on warnings.
      *Testable:* The CI workflow file contains a `cargo doc` step, and running
      it locally succeeds.

- [ ] **Step 4 — Add a developer setup section to the README.**
      Add a short "Development" section pointing to `CONTRIBUTING.md` and the
      Makefile.
      *Testable:* The README links to `CONTRIBUTING.md`.

- [ ] **Step 5 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes (total count unchanged at 81 or higher)
      and `cargo clippy --all-targets -- -D warnings` is clean.

## Files changed

- `CONTRIBUTING.md` (new)
- `src/flow.rs`, `src/api.rs`, `src/config.rs`, `src/file_tree.rs` (doc comments)
- `.github/workflows/ci.yml`
- `README.md`

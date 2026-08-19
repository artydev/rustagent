# Task 14 — Remove the stale "unfinished" disclaimer and formalize the release

**Status:** 🔴 Not started

## Goal

The README still carries a prominent warning that the project is "in
development / not finished," yet all 13 roadmap tasks are done, 81 tests pass,
and CI/release pipelines exist. This task updates the documentation to reflect
the completed state and cuts the first tagged release.

## Why

The disclaimer is now stale and undermines confidence in a project that is
functionally complete. Formalizing the release makes the project shippable.

## Steps

- [ ] **Step 1 — Audit the README for stale "unfinished" language.**
      Find every occurrence of "in development", "not finished", "incomplete",
      "prototype", "not guaranteed", and "not production-ready" in `README.md`.
      List each location so nothing is missed.
      *Testable:* `grep -n -i "unfinished\|in development\|not finished\|incomplete\|prototype\|not guaranteed\|not production-ready" README.md` returns only the locations you intend to keep or edit.

- [ ] **Step 2 — Rewrite the status banner.**
      Replace the "⚠️ Statut : En cours de développement" banner with a
      "stable / released" statement reflecting that all roadmap items are
      complete and the app builds and passes its test suite.
      *Testable:* The banner no longer contains the words "unfinished",
      "incomplete", or "not finished".

- [ ] **Step 3 — Update the "Ce qui n'est PAS abouti" section.**
      Remove or rewrite the limitations that are no longer true (e.g. "no
      Cargo.lock", "no tests/CI/packaging"). Keep only genuinely true
      limitations (e.g. Windows/macOS runtime not yet exercised).
      *Testable:* Every claim in the section is factually accurate against the
      current repo (Cargo.lock exists, tests exist, CI exists, release
      pipeline exists).

- [ ] **Step 4 — Update the roadmap checklist.**
      Ensure the "Feuille de route" section reflects the completed state (all
      items checked) or is removed in favor of a "future work" section.
      *Testable:* The roadmap no longer lists completed items as unchecked.

- [ ] **Step 5 — Tag the first release.**
      Create a git tag `v0.1.0` pointing at the current HEAD (matching the
      `0.1.0` entry already in `CHANGELOG.md`).
      *Testable:* `git tag -l` shows `v0.1.0` and `git describe --tags` returns
      `v0.1.0`.

- [ ] **Step 6 — Verify the project still builds and tests pass.**
      *Testable:* `cargo build` succeeds with no warnings and `cargo test`
      passes all 81 tests.

## Files changed

- `README.md`
- (git tag `v0.1.0`)

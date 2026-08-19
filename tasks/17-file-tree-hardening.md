# Task 17 — Harden the file-tree against real-world directory errors

**Status:** 🔴 Not started

## Goal

Make the sidebar file-tree robust against permission-denied directories,
symlink loops, and very large directories so it never panics, hangs, or blocks
the UI thread.

## Why

The file-tree scans the terminal's current directory on every prompt. Real
directories can contain unreadable subdirectories, symlink cycles, and huge
numbers of entries — any of which could crash or freeze the app today.

## Steps

- [ ] **Step 1 — Handle permission-denied / unreadable directories.**
      In `src/file_tree.rs`, make the directory scan return an error (or skip
      the entry) instead of panicking when a directory cannot be read.
      *Testable:* A unit test creates a directory with no read permission (or
      mocks a read error) and asserts the scan returns gracefully without
      panicking.

- [ ] **Step 2 — Prevent symlink loops.**
      Track visited canonical paths (or inode/device pairs) during the scan and
      skip directories already visited, so symlink cycles terminate.
      *Testable:* A unit test creates a symlink cycle (dir → itself or A → B →
      A) and asserts the scan terminates and does not recurse infinitely.

- [ ] **Step 3 — Add a max-depth / entry limit.**
      Add a configurable maximum recursion depth and maximum number of entries
      per directory so pathological trees cannot exhaust memory.
      *Testable:* A unit test with a deep tree asserts the scan stops at the
      configured depth, and a test with many entries asserts the entry limit is
      respected.

- [ ] **Step 4 — Make scanning async / debounced.**
      Move the directory scan off the UI thread (e.g. run it in a background
      task) and debounce it so rapid `cd` commands do not trigger overlapping
      scans.
      *Testable:* A test (or code review) confirms the scan runs in a spawned
      task and that a debounce window collapses rapid triggers into one scan.

- [ ] **Step 5 — Add lazy loading for large trees.**
      Only expand a directory's children when the user expands it, rather than
      scanning the whole tree eagerly.
      *Testable:* A test asserts that a directory's children are not scanned
      until it is expanded.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes (total count increases above 81) and
      `cargo clippy --all-targets -- -D warnings` is clean.

## Files changed

- `src/file_tree.rs`
- `src/main.rs` (wire async/debounced scanning into `file_tree_panel`)

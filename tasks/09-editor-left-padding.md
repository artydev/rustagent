# Task 9 — Suppress the left padding in the code editor panel

**Status:** 🟢 Done

## Goal

Remove the left padding around the code editor so the code sits flush against
the left edge of its panel, while keeping the padding on the top, right, and
bottom sides.

## Requirements

- [x] The code editor has **no** left padding (code starts at the left edge).
- [x] The top, right, and bottom padding around the editor is preserved (6px).
- [x] The change is limited to the code editor panel and does not affect the
      chat or terminal panels.
- [x] The project still compiles cleanly with no warnings.

## Current behaviour

In `src/main.rs`, the `code_editor_panel` function wraps the `CodeEditor` in a
`rect()` with `.padding(6.)`, which applies a uniform 6px padding on all four
sides:

```rust
.child(
    rect()
        .expanded()
        .padding(6.)
        .child(CodeEditor::new(editor, a11y_id).background((20, 20, 20))),
)
```

## Proposed approach

1. Replace the uniform `.padding(6.)` with a `Gaps` value that sets the left
   side to `0` while keeping the other sides at `6`.
2. `Gaps::new(top, right, bottom, left)` takes the four sides in that order,
   so `Gaps::new(6., 6., 6., 0.)` keeps 6px on the top, right, and bottom and
   suppresses the left padding.
3. `Gaps` is available in scope through freya's `prelude::*` (re-exported via
   `elements::extensions::*`), so no extra import is needed.

## Implementation

In `src/main.rs`, the `code_editor_panel` function's editor wrapper was
changed from a uniform `.padding(6.)` to an asymmetric `Gaps` value:

```rust
.child(
    rect()
        .expanded()
        .padding(Gaps::new(6., 6., 6., 0.))  // top, right, bottom, left
        .child(CodeEditor::new(editor, a11y_id).background((20, 20, 20))),
)
```

The `Gaps::new(top, right, bottom, left)` argument order was confirmed against
the `torin` crate source (the crate that provides `Gaps`), and `Gaps` is
re-exported through freya's prelude so it is in scope without an extra import.

## Verification

- `cargo check` compiles cleanly with **no warnings**.
- The change is scoped to the code editor panel only; the chat and terminal
  panels are untouched.

## Notes

- `Gaps::new` takes the four sides in `(top, right, bottom, left)` order.
- If the left padding ever needs to be restored, revert to `.padding(6.)` or
  use `Gaps::new(6., 6., 6., 6.)`.

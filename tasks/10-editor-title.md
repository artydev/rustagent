# Task 10 — Update the editor file title based on the script written

**Status:** 🟢 Done

## Goal

Whenever a script is written into the code editor, update the file title shown
in the editor header to match the script that was written, instead of always
showing a generic `main.<ext>` label.

## Requirements

- [x] When code is inserted into the editor, the editor header title reflects
      the script that was written (e.g. a Fibonacci script → `fibonacci.py`).
- [x] The title still reflects the correct language/extension for the script.
- [x] The title stays consistent with the temp file written before execution
      (or the execution flow is updated to match the new title).
- [x] The title updates correctly for all supported languages.
- [x] The project still compiles cleanly with no warnings.

## Current behaviour

In `src/main.rs`, the editor header title is derived **only from the detected
language**, not from the actual script content:

```rust
fn file_name(&self) -> String {
    if *self == SupportedLanguage::Java {
        return "Main.java".to_string();
    }
    format!("main.{}", self.extension())
}
```

and it is passed to the panel as a static value:

```rust
code_editor_panel(editor.into(), current_language.read().file_name())
```

So every script is titled `main.py`, `main.rs`, `main.js`, etc., regardless of
what the script actually does.

## Proposed approach

1. Derive a meaningful file name from the script content when code is inserted
   (e.g. from the first function/class name, or a slug of the user's request),
   falling back to `main.<ext>` when no meaningful name can be derived.
2. Make the editor title a piece of shared state (like `current_language`) so
   it can be updated when code is inserted and read by `code_editor_panel`.
3. Keep the title consistent with the temp file written before execution, or
   update the execution flow to write the temp file using the new title.

## Verification

- `cargo build` compiles cleanly with no warnings.
- Inserting a script updates the editor header title to match the script.
- The title falls back to `main.<ext>` when no meaningful name is available.
- The execution flow still runs the script correctly.

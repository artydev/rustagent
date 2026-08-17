# Task 11 — Place the terminal below the code editor, keeping the chat full screen

**Status:** 🟢 Done

## Goal

Re-arrange the main layout so the **terminal** sits directly **below the code
editor**, while the **chat** keeps the full height of the window (full screen).

## Requirements

- [x] The terminal panel is positioned directly below the code editor panel.
- [x] The chat panel retains the full height of the window (full screen).
- [x] The code editor keeps its current width/behaviour.
- [x] The layout remains usable (resizable panels still work as expected).
- [x] The project still compiles cleanly with no warnings.

## Current behaviour

In `src/main.rs`, the main layout uses a single horizontal
`ResizableContainer` with three side-by-side panels:

```rust
ResizableContainer::new()
    .direction(Direction::Horizontal)
    .panel(ResizablePanel::new(PanelSize::percent(33.)).child(/* chat */))
    .panel(ResizablePanel::new(PanelSize::percent(33.)).child(
        code_editor_panel(editor.into(), file_name.read().clone()),
    ))
    .panel(
        ResizablePanel::new(PanelSize::percent(34.))
            .child(terminal_panel(terminal_handle.into_writable())),
    )
```

So the three panels (chat, editor, terminal) are arranged left-to-right, each
taking roughly a third of the width.

## Proposed approach

1. Restructure the layout so the editor and terminal are stacked vertically
   (editor on top, terminal below) in a single column, while the chat remains
   a full-height panel beside them.
2. Use a nested `ResizableContainer` (or a vertical container) so the editor
   and terminal can be resized relative to each other.
3. Keep the chat panel at full height on the left (or right) side.
4. Verify the layout renders correctly and the project still compiles cleanly.

## Verification

- `cargo build` compiles cleanly with no warnings.
- The terminal appears directly below the code editor.
- The chat panel spans the full height of the window.
- Resizing still works for the editor/terminal split.

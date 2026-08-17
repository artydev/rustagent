# Task 12 — Add a sidebar file-tree panel that follows the terminal's current directory

**Status:** 🔴 Not started

## Goal

Add a **left sidebar panel** that displays the folders/directories of the
current working directory as a file tree. The tree must be driven by the
**path shown in the terminal** and must **update as soon as the path changes**
(e.g. when the user runs `cd` in the terminal).

## Requirements

- [ ] A left sidebar panel is added to the layout, showing folders and
      directories as a file tree.
- [ ] The tree reflects the current working directory indicated by the
      terminal's path.
- [ ] The tree updates immediately whenever the path in the terminal changes.
- [ ] The sidebar is visually integrated with the existing layout and theme.
- [ ] The project still compiles cleanly with no warnings.

## Current behaviour

There is currently no file-tree / directory sidebar. The layout (from
Task 11) is a horizontal `ResizableContainer` with a full-height chat panel on
the left and a right-hand column containing the code editor (top) and terminal
(below). The terminal shows the current working directory, but nothing in the
UI reflects or tracks it.

## Proposed approach

1. Add a new left sidebar panel (file tree) to the main layout, to the left of
   the existing chat panel.
2. Track the terminal's current working directory. The terminal reports its
   title / path; hook into the existing `title_changed` handling (or the
   terminal's working-directory reporting) to detect when the path changes.
3. When the path changes, re-scan that directory and rebuild the file-tree
   state (folders and files).
4. Render the tree in the sidebar using the existing theme (Obsidian-style
   colors used elsewhere in the app).
5. Keep the change scoped to the new sidebar; do not disturb the chat, editor,
   or terminal behaviour.

## Verification

- `cargo build` compiles cleanly with no warnings.
- A left sidebar shows the folders/directories of the current directory.
- Running `cd <dir>` in the terminal updates the sidebar tree immediately.
- The existing chat, editor, and terminal panels still work as before.

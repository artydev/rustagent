# Task 8 — Send chat messages on Enter key or Send button

**Status:** 🟢 Done

## Goal

Allow the user to send a chat message to the LLM either by pressing the
**Enter** key in the chat input field or by clicking the **Send** button.
Both triggers must behave identically and share the same code path.

## Requirements

- [x] Pressing **Enter** in the chat input field sends the current message to
      the LLM (same behaviour as the Send button).
- [x] The **Send** button continues to work as before (sends the message).
- [x] Both triggers share the same send handler / code path (no duplication).
- [x] Empty or whitespace-only messages are ignored (not sent) in both cases.
- [x] The input field is cleared after sending, in both cases.
- [x] The Enter key does not interfere with multi-line input (if the input
      field supports it) or with other keyboard shortcuts.
- [x] Add tests covering the shared send logic (Enter and button both trigger
      the same behaviour).

## Current behaviour

In `src/main.rs`, the chat input is an `Input::new(input_value)` with a
**Send** `Button` whose `on_press` calls `send_message`. There is currently
**no** Enter-key handling on the input field — the message can only be sent by
clicking the Send button.

## Proposed approach

1. Extract the send logic into a single reusable handler (the existing
   `send_message` closure) so both the button and the Enter key call the same
   code.
2. Attach an `on_key_down` (or equivalent) handler to the chat `Input` that
   detects the **Enter** key (`Key::Named(NamedKey::Enter)`) and invokes the
   shared send handler.
3. Ensure the shared handler already guards against empty/whitespace-only
   messages and clears the input after sending (it does today).
4. Add unit tests for the shared send logic (empty input ignored, input
   cleared, both triggers route to the same handler).

## Implementation

All 7 steps are complete. The send logic was refactored in `src/main.rs` so
both triggers share a single code path:

1. **Shared `send_text` closure** — The core send logic (empty-check, adding
   the user message, language detection, clear-editor handling, and the async
   AI call) now lives in one `send_text` closure. Both the Send button and the
   Enter key route through it, so they behave identically.
2. **`should_send_message()` helper** — A single source of truth for the
   empty/whitespace check (`!message.trim().is_empty()`), used by `send_text`
   and unit-tested directly.
3. **Send button handler** (`send_message`) — reads the current input field
   and calls `send_text`.
4. **Enter key handler** (`on_submit`) — the `Input` component calls
   `on_submit` with the committed text when Enter is pressed, so it passes the
   text straight to `send_text`.
5. **Input cleared after sending** — `send_text` clears `input_value` after
   dispatching, so both triggers clear the field.
6. **No interference** — the input field is single-line, so Enter sending is
   unambiguous and does not conflict with the editor or terminal.
7. **Tests** — added 4 unit tests for `should_send_message` (empty,
   whitespace-only, non-empty, and leading/trailing whitespace).

## Verification

- `cargo build` compiles cleanly with **no warnings**.
- `cargo test` passes **all 65 tests** (including the 4 new ones).

## Notes

- The Enter key should only trigger a send when the input field is focused, so
  it does not interfere with typing in the editor or terminal.
- If the input field is single-line (as it is today), Enter sending is
  unambiguous. If multi-line input is added later, Enter should send and
  Shift+Enter should insert a newline.

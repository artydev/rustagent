# Task 19 — Add streaming responses to the AI chat

**Status:** 🔴 Not started

## Goal

Stream the AI response tokens into the chat as they arrive, instead of waiting
for the full response before displaying anything.

## Why

Streaming makes the chat feel dramatically more responsive. Today the app
likely waits for the complete response, which can take many seconds for long
code generations.

## Steps

- [ ] **Step 1 — Investigate the current response handling.**
      Read `src/api.rs` and the chat send path in `src/main.rs` to confirm
      whether the response is currently fetched all-at-once.
      *Testable:* Document the current behavior (full-response vs streaming) in
      the task notes.

- [ ] **Step 2 — Add a streaming API call.**
      Add a function in `src/api.rs` that streams tokens (using rig-core's
      streaming support or the OpenAI-compatible SSE endpoint) and yields each
      chunk.
      *Testable:* A unit test (or a mocked stream) asserts the function yields
      multiple chunks rather than a single final string.

- [ ] **Step 3 — Wire streaming into the chat panel.**
      Update the chat send path to append tokens to the assistant message as
      they arrive, updating the UI incrementally.
      *Testable:* A test feeds a fake multi-chunk stream and asserts the
      assistant message grows incrementally.

- [ ] **Step 4 — Keep code extraction working with streaming.**
      Ensure the code-block extraction (`flow::extract_code_blocks`) still
      works on the final assembled response, and that the editor is populated
      only once the stream completes.
      *Testable:* A test streams a response containing a code block and asserts
      the editor receives the extracted code after the stream finishes.

- [ ] **Step 5 — Handle stream errors and cancellation.**
      Handle mid-stream errors gracefully (show a partial message plus an error
      notice) and support cancelling an in-flight stream.
      *Testable:* A test simulates a mid-stream error and asserts the UI shows
      the partial message and an error, without crashing.

- [ ] **Step 6 — Verify the full suite still passes.**
      *Testable:* `cargo test` passes (total count increases above 81) and
      `cargo clippy --all-targets -- -D warnings` is clean.

## Files changed

- `src/api.rs`
- `src/main.rs`
- `src/flow.rs` (if extraction needs adjustment)

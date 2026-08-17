# RustAgent — Manual End-to-End (E2E) Testing Checklist

> This document is a **manual, human-driven** end-to-end test plan for
> **RustAgent**. It complements the automated unit/integration tests in
> `src/flow.rs` by exercising the real application with a **live API key** and
> a **real GUI**, terminal, and language toolchains.
>
> The automated tests prove that the pure logic (language detection, code
> extraction, run-command construction) is correct in isolation. This checklist
> proves that the **wired-up application** behaves correctly when a real user
> drives the chat → editor → terminal flow.
>
> **Prerequisites before you start:**
> - A working build: `cargo build` (see `README.md`).
> - A valid **Albert API key** exported as `ALBERT_API_KEY` (or set via the
>   Settings panel / config file — see `README.md` → Configuration).
> - The language toolchains you intend to test installed and on `PATH`:
>   `python3`, `rustc`, `node`, `npx`, `gcc`, `g++`, `javac`/`java`, `go`.
> - A graphical session (the app is a Freya desktop GUI).

---

## How to use this checklist

Each section contains a set of **test cases**. For each case:

1. Perform the **Steps**.
2. Compare the **Expected result** with what you observe.
3. Mark the case **PASS** / **FAIL** in the checkbox.
4. If it fails, record the actual behavior in the **Notes** column and file a
   bug report.

> ⚠️ **Important:** These tests call the **real Albert API** and consume
> tokens. Run them when you have quota available. The app has a 60-second
> per-request timeout and automatic retries with backoff, so transient network
> issues should self-heal.

---

## 1. Startup & API key configuration

### 1.1 Launch with a valid API key

- [ ] **PASS / FAIL** — Launch the app with `ALBERT_API_KEY` set to a valid key.
  - **Steps:** `ALBERT_API_KEY=sk-... cargo run`
  - **Expected:** The app window opens. No error banner about a missing or
    invalid key. The chat panel is ready for input.

### 1.2 Launch with a missing API key

- [ ] **PASS / FAIL** — Launch the app **without** any API key configured.
  - **Steps:** `cargo run` (no env var, no config file, no saved key).
  - **Expected:** The app opens but displays a clear, user-friendly warning
    that the API key is missing. Sending a chat message fails gracefully with
    an authentication error message (not a crash).

### 1.3 Launch with an invalid API key

- [ ] **PASS / FAIL** — Launch with a syntactically valid but wrong key.
  - **Steps:** `ALBERT_API_KEY=invalid-key cargo run`, then send a message.
  - **Expected:** The app reports an **authentication** error (401) with a
    clear message. The UI does not freeze or crash.

### 1.4 Configure the key via the Settings panel

- [ ] **PASS / FAIL** — Save a key through the UI.
  - **Steps:** Click **Settings** → enter a key → save. Restart the app.
  - **Expected:** The key is persisted to the platform config file
    (`~/.config/rustagent/config.toml` on Linux) and is used on the next
    launch without needing the env var.

### 1.5 API error handling (rate limit / network)

- [ ] **PASS / FAIL** — Trigger a rate-limit or network error.
  - **Steps:** Send several messages rapidly, or disconnect the network, then
    send a message.
  - **Expected:** The app shows a categorized error (rate limit / network /
    timeout) with an appropriate message. Transient failures are retried with
    backoff. The UI remains responsive.

---

## 2. Chat → Editor flow (code generation)

> For each language below, the flow is the same:
> 1. Type a natural-language request that asks for code.
> 2. Send it.
> 3. Verify the generated code is **inserted into the editor** (not shown as
>    raw text in the chat).
> 4. Verify the chat shows the **insertion confirmation** message.
> 5. Verify the editor's language is set correctly (syntax highlighting).

### 2.1 Python

- [ ] **PASS / FAIL**
  - **Steps:** Send `write a python function that prints hello`.
  - **Expected:** Editor contains a Python function; chat shows the
    confirmation; editor language = Python.

### 2.2 Rust

- [ ] **PASS / FAIL**
  - **Steps:** Send `generate a rust program that prints hello`.
  - **Expected:** Editor contains Rust code (`fn main`); chat confirmation;
    editor language = Rust.

### 2.3 JavaScript

- [ ] **PASS / FAIL**
  - **Steps:** Send `write javascript to print hello`.
  - **Expected:** Editor contains JS code; chat confirmation; editor language
    = JavaScript.

### 2.4 TypeScript

- [ ] **PASS / FAIL**
  - **Steps:** Send `write typescript code with a typed function`.
  - **Expected:** Editor contains TS code; chat confirmation; editor language
    = TypeScript.

### 2.5 HTML

- [ ] **PASS / FAIL**
  - **Steps:** Send `write an html page`.
  - **Expected:** Editor contains HTML; chat confirmation; editor language =
    HTML.

### 2.6 CSS

- [ ] **PASS / FAIL**
  - **Steps:** Send `write css to style a button`.
  - **Expected:** Editor contains CSS; chat confirmation; editor language =
    CSS.

### 2.7 C

- [ ] **PASS / FAIL**
  - **Steps:** Send `write c code that prints hello`.
  - **Expected:** Editor contains C code (`#include`, `int main`); chat
    confirmation; editor language = C.

### 2.8 C++

- [ ] **PASS / FAIL**
  - **Steps:** Send `write c++ code that prints hello`.
  - **Expected:** Editor contains C++ code; chat confirmation; editor language
    = C++.

### 2.9 Java

- [ ] **PASS / FAIL**
  - **Steps:** Send `write java code that prints hello`.
  - **Expected:** Editor contains Java code (`public class`); chat
    confirmation; editor language = Java.

### 2.10 Go

- [ ] **PASS / FAIL**
  - **Steps:** Send `write go code that prints hello`.
  - **Expected:** Editor contains Go code (`package main`, `func main`); chat
    confirmation; editor language = Go.

---

## 3. Editor → Terminal flow (code execution)

> For each language, after the code is in the editor, press **Execute Code**
> and verify it runs in the integrated terminal and produces the expected
> output.

### 3.1 Python

- [ ] **PASS / FAIL**
  - **Steps:** After 2.1, press **Execute Code**.
  - **Expected:** Terminal runs `python3 main.py` and prints `hello`.

### 3.2 Rust

- [ ] **PASS / FAIL**
  - **Steps:** After 2.2, press **Execute Code**.
  - **Expected:** Terminal runs `rustc main.rs -o ... && ...` and prints
    `hello`.

### 3.3 JavaScript

- [ ] **PASS / FAIL**
  - **Steps:** After 2.3, press **Execute Code**.
  - **Expected:** Terminal runs `node main.js` and prints `hello`.

### 3.4 TypeScript

- [ ] **PASS / FAIL**
  - **Steps:** After 2.4, press **Execute Code**.
  - **Expected:** Terminal runs `npx ts-node main.ts` and prints `hello`.

### 3.5 HTML

- [ ] **PASS / FAIL**
  - **Steps:** After 2.5, press **Execute Code**.
  - **Expected:** The default browser opens the generated HTML file
    (`xdg-open` on Linux, `open` on macOS, `start` on Windows).

### 3.6 CSS

- [ ] **PASS / FAIL**
  - **Steps:** After 2.6, press **Execute Code**.
  - **Expected:** CSS has no runnable output; the app handles this gracefully
    (no crash, no spurious error). *(Document the actual behavior in Notes.)*

### 3.7 C

- [ ] **PASS / FAIL**
  - **Steps:** After 2.7, press **Execute Code**.
  - **Expected:** Terminal runs `gcc main.c -o ... && ...` and prints `hello`.

### 3.8 C++

- [ ] **PASS / FAIL**
  - **Steps:** After 2.8, press **Execute Code**.
  - **Expected:** Terminal runs `g++ main.cpp -o ... && ...` and prints
    `hello`.

### 3.9 Java

- [ ] **PASS / FAIL**
  - **Steps:** After 2.9, press **Execute Code**.
  - **Expected:** Terminal runs `javac main.java && java Main` and prints
    `hello`.

### 3.10 Go

- [ ] **PASS / FAIL**
  - **Steps:** After 2.10, press **Execute Code**.
  - **Expected:** Terminal runs `go run main.go` and prints `hello`.

---

## 4. Edge cases

### 4.1 Plain question (no code requested)

- [ ] **PASS / FAIL**
  - **Steps:** Send `what is the capital of France?`.
  - **Expected:** The AI's answer is shown **in the chat** as text. The editor
    is **not** modified. No code is inserted.

### 4.2 Code requested but AI returns no code

- [ ] **PASS / FAIL**
  - **Steps:** Send a request that asks for code but the AI responds with only
    prose (e.g., a refusal).
  - **Expected:** The raw response is shown in the chat. The editor is **not**
    modified.

### 4.3 Clear editor command

- [ ] **PASS / FAIL**
  - **Steps:** Put some code in the editor, then send `clear the editor`.
  - **Expected:** The editor is emptied **locally** without calling the AI.
    The chat shows a confirmation.

### 4.4 Empty editor + Execute Code

- [ ] **PASS / FAIL**
  - **Steps:** With an empty editor, press **Execute Code**.
  - **Expected:** The app handles this gracefully (no crash). *(Document the
    actual behavior in Notes.)*

### 4.5 Unsupported / ambiguous language

- [ ] **PASS / FAIL**
  - **Steps:** Send a request that doesn't clearly map to a supported language
    (e.g., `write some code` with no language hint).
  - **Expected:** The app falls back to a sensible default (likely Python) or
    handles the ambiguity gracefully without crashing.

### 4.6 Multiple code blocks in one response

- [ ] **PASS / FAIL**
  - **Steps:** Ask for code that the AI returns as multiple fenced blocks.
  - **Expected:** All matching blocks are concatenated into the editor.

### 4.7 Code with special characters

- [ ] **PASS / FAIL**
  - **Steps:** Ask for code containing quotes, `$`, or backticks.
  - **Expected:** The code is preserved verbatim in the editor (no mangling).

### 4.8 Very long code response

- [ ] **PASS / FAIL**
  - **Steps:** Ask for a large program.
  - **Expected:** The full code is inserted without truncation.

### 4.9 Toolbar: Clear chat

- [ ] **PASS / FAIL**
  - **Steps:** Have a conversation, then click **Clear chat**.
  - **Expected:** The chat history is cleared. The editor and terminal are
    unaffected.

### 4.10 Toolbar: Reset terminal

- [ ] **PASS / FAIL**
  - **Steps:** Run something in the terminal, then click **Reset terminal**.
  - **Expected:** The shell is killed and restarted fresh.

### 4.11 Case-insensitivity of commands

- [ ] **PASS / FAIL**
  - **Steps:** Send `CLEAR THE EDITOR` and `WRITE A PYTHON FUNCTION`.
  - **Expected:** Both are recognized correctly (clear editor works; Python
    code is generated).

### 4.12 Terminal edge cases

- [ ] **PASS / FAIL** — Run a program that produces no output.
  - **Steps:** Generate and run a program that does nothing (e.g., an empty
    `main`).
  - **Expected:** The terminal completes without error.

- [ ] **PASS / FAIL** — Run a program that errors at runtime.
  - **Steps:** Generate and run a program that throws an exception / exits
    non-zero.
  - **Expected:** The error output appears in the terminal; the app does not
    crash.

- [ ] **PASS / FAIL** — Run a program with user input.
  - **Steps:** Generate and run a program that reads from stdin.
  - **Expected:** The terminal accepts input and the program completes.

- [ ] **PASS / FAIL** — Run a long-running program.
  - **Steps:** Generate and run a program with an infinite loop / sleep.
  - **Expected:** The terminal stays responsive; **Reset terminal** can
    interrupt it.

---

## Test summary

| Section | Cases | Passed | Failed | Notes |
| ------- | ----- | ------ | ------ | ----- |
| 1. Startup & API key | 5 | | | |
| 2. Chat → Editor (10 langs) | 10 | | | |
| 3. Editor → Terminal (10 langs) | 10 | | | |
| 4. Edge cases | 12 | | | |
| **Total** | **37** | | | |

---

## Reporting a failure

When a case fails, please capture:

1. The **test case ID** (e.g., `2.7`).
2. The **exact steps** you performed.
3. The **expected** vs **actual** behavior.
4. Any **error messages** from the terminal or chat.
5. The **platform** (OS, architecture) and **toolchain versions**.
6. Whether the failure is reproducible.

---

## Related documentation

- `README.md` — overview, configuration, supported languages, roadmap.
- `tasks/PROGRESS.md` — the task progress tracker this testing supports.
- `CHANGELOG.md` — version history.
- `src/flow.rs` — the automated unit/integration tests for the core flow.

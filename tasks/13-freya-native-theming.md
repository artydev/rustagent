# Task 13 — Refactor theming to use Freya's native theming system

**Status:** 🟢 Done

## Goal

Replace the custom, hand-rolled theming layer (introduced as an earlier
"external stylesheet" experiment) with **Freya's built-in theming system**.
The app's color palette still lives in an external `theme.json` file, but it is
now loaded into a Freya `Theme` / `ColorsSheet` and provided to the component
tree through Freya's own `use_provide_theme` / `use_theme` hooks.

## Why

The previous implementation defined its own `AppTheme` / `AppColors` types and
custom `use_provide_app_theme` / `use_app_theme` hooks. This was a parallel
theming system bolted onto Freya, which already ships a native one. The
collision was visible in the code: the custom types had to be renamed to
`AppTheme` / `AppColors` specifically to avoid clashing with Freya's
`Theme` / `ColorsSheet` re-exported through the prelude. This refactor removes
that duplication and leans on the framework's idiomatic theming instead.

## What has been accomplished

- ✅ **`src/theme.rs` rewritten** to build a Freya `Theme` from `theme.json`:
  - Deserializes a `RawColors` struct covering **all 26 fields** of Freya's
    `ColorsSheet` (brand/accent, status, surfaces, borders, text, states,
    utility).
  - `load_theme()` returns a Freya `Theme` via `Theme::new("obsidian", colors)`.
  - Removed the custom `AppTheme`, `AppColors`, `use_provide_app_theme`, and
    `use_app_theme` types/hooks entirely.
  - Kept the graceful fallback palette (mirrors `src/theme.json`) and the
    dual-path lookup (`theme.json` in cwd, then `src/theme.json`).

- ✅ **`src/theme.json` expanded** from 16 to **26 color roles** so it maps
  one-to-one onto Freya's `ColorsSheet`. The original Obsidian palette is
  preserved; the new roles (primary/secondary, success/error/info,
  border_focus/border_disabled, focus/active/disabled, surface_inverse_tertiary)
  use sensible dark-theme values.

- ✅ **`src/main.rs` updated** to use Freya's native hooks:
  - `use_provide_theme(theme::load_theme)` at the root (returns `State<Theme>`).
  - `theme.read().colors.clone()` to read the color sheet at the root.
  - `use_theme().read().colors.clone()` in each panel
    (code editor, settings, file tree, terminal).
  - `build_tree_rows(..., colors: ColorsSheet)` now takes Freya's `ColorsSheet`
    instead of the custom `AppColors`.
  - Removed the `use theme::AppColors;` import.

- ✅ **No custom theming code remains** — the app now uses Freya's native
  `Theme` / `ColorsSheet` / `use_provide_theme` / `use_theme` throughout.

- ✅ **Builds cleanly with no warnings** (`cargo build`).

- ✅ **All 81 tests pass** (`cargo test`).

## Verification details

- `cargo build` — succeeds, no warnings.
- `cargo test` — 81 passed, 0 failed.
- `grep` confirms no remaining `AppTheme` / `AppColors` /
  `use_provide_app_theme` / `use_app_theme` references in `src/`.

## Files changed

- `src/theme.rs` — rewritten to build a Freya `Theme` from `theme.json`.
- `src/theme.json` — expanded to all 26 `ColorsSheet` roles.
- `src/main.rs` — switched to Freya's `use_provide_theme` / `use_theme` hooks
  and `ColorsSheet`.

//! External theme ("stylesheet") support built on Freya's native theming.
//!
//! The app's color palette lives in a `theme.json` file. This module loads
//! that file at startup and builds a Freya [`Theme`] / [`ColorsSheet`] from
//! it, which is then provided to the component tree through Freya's own
//! `use_provide_theme` / `use_theme` hooks. If the file is missing or
//! malformed, a sensible dark fallback palette is used so the app still runs.
//!
//! The file is looked up in the current working directory (`theme.json`) and,
//! as a fallback, in the `src/` directory (`src/theme.json`). This lets the
//! theme be edited and reloaded without touching the Rust source.

use freya::prelude::*;
use serde::Deserialize;
use std::path::Path;

/// A raw color as it appears in `theme.json`.
#[derive(Deserialize)]
struct RawColor {
    r: u8,
    g: u8,
    b: u8,
    #[serde(default)]
    a: Option<u8>,
}

impl RawColor {
    fn to_color(&self) -> Color {
        match self.a {
            Some(a) => Color::from_argb(a, self.r, self.g, self.b),
            None => Color::from_rgb(self.r, self.g, self.b),
        }
    }
}

/// The JSON shape of `theme.json`. Every field maps to a [`ColorsSheet`]
/// field, so the UI reads like a stylesheet: components reference
/// `c.surface_primary` instead of hardcoding `(40, 40, 40)`.
#[derive(Deserialize)]
struct RawTheme {
    colors: RawColors,
}

#[derive(Deserialize)]
struct RawColors {
    // Brand & Accent
    primary: RawColor,
    secondary: RawColor,
    tertiary: RawColor,
    // Status / Semantic
    success: RawColor,
    warning: RawColor,
    error: RawColor,
    info: RawColor,
    // Surfaces / Backgrounds
    background: RawColor,
    surface_primary: RawColor,
    surface_secondary: RawColor,
    surface_tertiary: RawColor,
    surface_inverse: RawColor,
    surface_inverse_secondary: RawColor,
    surface_inverse_tertiary: RawColor,
    // Borders
    border: RawColor,
    border_focus: RawColor,
    border_disabled: RawColor,
    // Text / Content
    text_primary: RawColor,
    text_secondary: RawColor,
    text_placeholder: RawColor,
    text_inverse: RawColor,
    text_highlight: RawColor,
    // States / Interaction
    focus: RawColor,
    active: RawColor,
    disabled: RawColor,
    // Utility
    overlay: RawColor,
    shadow: RawColor,
}

impl RawColors {
    fn to_colors_sheet(&self) -> ColorsSheet {
        ColorsSheet {
            primary: self.primary.to_color(),
            secondary: self.secondary.to_color(),
            tertiary: self.tertiary.to_color(),
            success: self.success.to_color(),
            warning: self.warning.to_color(),
            error: self.error.to_color(),
            info: self.info.to_color(),
            background: self.background.to_color(),
            surface_primary: self.surface_primary.to_color(),
            surface_secondary: self.surface_secondary.to_color(),
            surface_tertiary: self.surface_tertiary.to_color(),
            surface_inverse: self.surface_inverse.to_color(),
            surface_inverse_secondary: self.surface_inverse_secondary.to_color(),
            surface_inverse_tertiary: self.surface_inverse_tertiary.to_color(),
            border: self.border.to_color(),
            border_focus: self.border_focus.to_color(),
            border_disabled: self.border_disabled.to_color(),
            text_primary: self.text_primary.to_color(),
            text_secondary: self.text_secondary.to_color(),
            text_placeholder: self.text_placeholder.to_color(),
            text_inverse: self.text_inverse.to_color(),
            text_highlight: self.text_highlight.to_color(),
            focus: self.focus.to_color(),
            active: self.active.to_color(),
            disabled: self.disabled.to_color(),
            overlay: self.overlay.to_color(),
            shadow: self.shadow.to_color(),
        }
    }
}

/// The fallback palette used when `theme.json` cannot be loaded. This mirrors
/// the values shipped in `src/theme.json` so the app looks identical whether
/// or not the file is present.
fn fallback_colors() -> ColorsSheet {
    ColorsSheet {
        primary: Color::from_rgb(103, 80, 164),
        secondary: Color::from_rgb(202, 193, 227),
        tertiary: Color::from_rgb(79, 61, 130),
        success: Color::from_rgb(129, 199, 132),
        warning: Color::from_rgb(255, 200, 120),
        error: Color::from_rgb(229, 115, 115),
        info: Color::from_rgb(100, 181, 246),
        background: Color::from_rgb(30, 30, 30),
        surface_primary: Color::from_rgb(40, 40, 40),
        surface_secondary: Color::from_rgb(20, 20, 20),
        surface_tertiary: Color::from_rgb(65, 65, 65),
        surface_inverse: Color::from_rgb(45, 55, 70),
        surface_inverse_secondary: Color::from_rgb(25, 25, 35),
        surface_inverse_tertiary: Color::from_rgb(60, 60, 70),
        border: Color::from_rgb(55, 55, 55),
        border_focus: Color::from_rgb(110, 110, 110),
        border_disabled: Color::from_rgb(80, 80, 80),
        text_primary: Color::from_rgb(245, 245, 245),
        text_secondary: Color::from_rgb(200, 200, 200),
        text_placeholder: Color::from_rgb(150, 150, 160),
        text_inverse: Color::from_rgb(220, 220, 230),
        text_highlight: Color::from_rgb(180, 180, 190),
        focus: Color::from_rgb(100, 100, 120),
        active: Color::from_rgb(70, 70, 70),
        disabled: Color::from_rgb(50, 50, 50),
        overlay: Color::from_argb(180, 0, 0, 0),
        shadow: Color::from_argb(120, 0, 0, 0),
    }
}

/// Read the raw theme JSON from disk, trying the working directory first and
/// then the `src/` directory.
fn read_raw_theme() -> Option<RawTheme> {
    for candidate in [Path::new("theme.json"), Path::new("src/theme.json")] {
        if let Ok(text) = std::fs::read_to_string(candidate) {
            if let Ok(raw) = serde_json::from_str::<RawTheme>(&text) {
                return Some(raw);
            }
        }
    }
    None
}

/// Load the theme from `theme.json` and build a Freya [`Theme`]. Falls back
/// to the built-in palette on any error.
///
/// We start from Freya's `dark_theme()`, which registers every built-in
/// component theme (button, input, scrollbar, ...), and then swap in our own
/// [`ColorsSheet`]. This keeps the component themes intact while letting the
/// app's palette come entirely from `theme.json`.
pub fn load_theme() -> Theme {
    let colors = match read_raw_theme() {
        Some(raw) => raw.colors.to_colors_sheet(),
        None => fallback_colors(),
    };

    let mut theme = dark_theme();
    theme.name = "obsidian";
    theme.colors = colors;
    theme
}

//! Theme system — trait, appearance enum, and global reactive state

use std::sync::OnceLock;

use floem_reactive::{RwSignal, SignalGet};
use serde::{Deserialize, Serialize};

use crate::palette::Palette;
use crate::system::is_system_dark_mode;

/// Trait for defining a custom theme.
///
/// Implement this to provide your own dark/light palettes.
/// The built-in default theme uses `Palette::dark()` and `Palette::light()`.
pub trait ThemeDef: Send + Sync + 'static {
    /// The dark mode palette for this theme.
    fn dark_palette(&self) -> Palette;
    /// The light mode palette for this theme.
    fn light_palette(&self) -> Palette;
    /// A human-readable name for this theme.
    fn name(&self) -> &str;
}

/// User-facing appearance preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Appearance {
    #[default]
    Dark,
    Light,
    System,
}

/// Resolved appearance (no `System` variant).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedAppearance {
    Dark,
    Light,
}

impl Appearance {
    /// Resolve `System` to the actual OS appearance.
    pub fn resolve(self) -> ResolvedAppearance {
        match self {
            Appearance::Dark => ResolvedAppearance::Dark,
            Appearance::Light => ResolvedAppearance::Light,
            Appearance::System => {
                if is_system_dark_mode() {
                    ResolvedAppearance::Dark
                } else {
                    ResolvedAppearance::Light
                }
            }
        }
    }
}

// -- Built-in default theme --------------------------------------------------

struct DefaultTheme;

impl ThemeDef for DefaultTheme {
    fn dark_palette(&self) -> Palette {
        Palette::dark()
    }
    fn light_palette(&self) -> Palette {
        Palette::light()
    }
    fn name(&self) -> &str {
        "Default"
    }
}

// -- Global state -------------------------------------------------------------

struct ThemeState {
    theme: Box<dyn ThemeDef>,
    appearance: RwSignal<Appearance>,
}

static THEME_STATE: OnceLock<ThemeState> = OnceLock::new();

/// Initialize the global theme with the built-in default palettes.
///
/// Returns the reactive `Appearance` signal. Call once at app startup.
///
/// # Panics
///
/// Panics if called outside a reactive runtime.
pub fn init(appearance: Appearance) -> RwSignal<Appearance> {
    init_with(DefaultTheme, appearance)
}

/// Initialize the global theme with a custom `ThemeDef`.
///
/// Returns the reactive `Appearance` signal. Call once at app startup.
///
/// # Panics
///
/// Panics if called outside a reactive runtime.
pub fn init_with(theme: impl ThemeDef, appearance: Appearance) -> RwSignal<Appearance> {
    let signal = RwSignal::new(appearance);
    let _ = THEME_STATE.set(ThemeState {
        theme: Box::new(theme),
        appearance: signal,
    });
    signal
}

/// Get the reactive `Appearance` signal.
///
/// Returns `None` if `init()` / `init_with()` has not been called yet.
pub fn appearance_signal() -> Option<RwSignal<Appearance>> {
    THEME_STATE.get().map(|s| s.appearance)
}

/// Get the current `Palette` based on the active appearance.
///
/// This reads the reactive `Appearance` signal, so calling it inside a
/// `.style(|s| ...)` closure creates a reactive dependency — the closure
/// re-runs when the appearance changes.
///
/// Falls back to `Palette::dark()` if `init()` has not been called.
pub fn palette() -> Palette {
    match THEME_STATE.get() {
        Some(state) => {
            let resolved = state.appearance.get().resolve();
            match resolved {
                ResolvedAppearance::Dark => state.theme.dark_palette(),
                ResolvedAppearance::Light => state.theme.light_palette(),
            }
        }
        None => Palette::dark(),
    }
}

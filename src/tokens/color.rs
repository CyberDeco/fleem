//! Semantic color tokens for theming

use floem::prelude::Color;

/// All semantic color slots for a theme palette.
///
/// Covers backgrounds, text, borders, and semantic status colors.
/// Each field is a concrete `Color` value — no indirection.
#[derive(Debug, Clone, Copy)]
pub struct ColorTokens {
    // Backgrounds
    /// Window/root background
    pub bg_base: Color,
    /// Card or panel background
    pub bg_surface: Color,
    /// Raised element background
    pub bg_elevated: Color,
    /// Hover state background
    pub bg_hover: Color,
    /// Selected/active state background
    pub bg_selected: Color,
    /// Text input background
    pub bg_input: Color,
    /// Disabled element background
    pub bg_disabled: Color,
    /// Modal/overlay backdrop
    pub bg_overlay: Color,

    // Text
    /// Primary body text
    pub text_primary: Color,
    /// Secondary/supporting text
    pub text_secondary: Color,
    /// De-emphasized/placeholder text
    pub text_muted: Color,
    /// Text on colored backgrounds (e.g. accent buttons)
    pub text_inverse: Color,
    /// Disabled text
    pub text_disabled: Color,
    /// Link text
    pub text_link: Color,

    // Borders
    /// Default border
    pub border: Color,
    /// Stronger/emphasized border
    pub border_strong: Color,
    /// Focus ring border
    pub border_focus: Color,

    // Semantic: Accent
    /// Primary accent color
    pub accent: Color,
    /// Accent hover state
    pub accent_hover: Color,
    /// Foreground on accent background
    pub accent_fg: Color,

    // Semantic: Success
    /// Success color
    pub success: Color,
    /// Success background (tinted)
    pub success_bg: Color,
    /// Foreground on success background
    pub success_fg: Color,

    // Semantic: Error
    /// Error/danger color
    pub error: Color,
    /// Error background (tinted)
    pub error_bg: Color,
    /// Foreground on error background
    pub error_fg: Color,

    // Semantic: Warning
    /// Warning color
    pub warning: Color,
    /// Warning background (tinted)
    pub warning_bg: Color,
    /// Foreground on warning background
    pub warning_fg: Color,

    // Semantic: Info
    /// Info color
    pub info: Color,
    /// Info background (tinted)
    pub info_bg: Color,
    /// Foreground on info background
    pub info_fg: Color,

    // Semantic: Neutral
    /// Neutral color (secondary actions)
    pub neutral: Color,
    /// Neutral hover state
    pub neutral_hover: Color,
    /// Foreground on neutral background
    pub neutral_fg: Color,
}

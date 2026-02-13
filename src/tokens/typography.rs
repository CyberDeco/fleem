//! Typography tokens for consistent text sizing

/// Font size tokens for a theme palette.
///
/// Values are in logical pixels (f32), matching floem's `font_size()`.
#[derive(Debug, Clone, Copy)]
pub struct TypographyTokens {
    /// Title text (18.0)
    pub font_title: f32,
    /// Section heading text (14.0)
    pub font_heading: f32,
    /// Body/status text (12.0)
    pub font_body: f32,
    /// Label text (11.0)
    pub font_label: f32,
    /// Small text (10.0)
    pub font_small: f32,
    /// Tiny text (9.0)
    pub font_tiny: f32,
    /// Monospace font family name
    pub font_mono: &'static str,
}

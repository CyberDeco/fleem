//! Spacing tokens for consistent layout

/// Spacing, radius, and dimension tokens for a theme palette.
///
/// All values are in logical pixels (f32).
#[derive(Debug, Clone, Copy)]
pub struct SpacingTokens {
    // Padding
    /// Extra-small padding (4.0)
    pub pad_xs: f32,
    /// Small padding (6.0)
    pub pad_sm: f32,
    /// Medium padding (8.0)
    pub pad_md: f32,
    /// Large padding (12.0)
    pub pad_lg: f32,
    /// Extra-large padding (16.0)
    pub pad_xl: f32,

    // Gaps
    /// Small gap between elements (8.0)
    pub gap_sm: f32,
    /// Medium gap (12.0)
    pub gap_md: f32,
    /// Large gap (16.0)
    pub gap_lg: f32,

    // Border radii
    /// Small radius (3.0)
    pub radius_sm: f32,
    /// Medium radius (4.0)
    pub radius_md: f32,
    /// Large radius (6.0)
    pub radius_lg: f32,
    /// Extra-large radius (8.0)
    pub radius_xl: f32,

    // Border widths
    /// Default border width (1.0)
    pub border_width: f32,
    /// Thick border width (2.0)
    pub border_width_thick: f32,

    // Dimensions
    /// Minimum width for text inputs (150.0)
    pub input_min_width: f32,
    /// Standard label width for form fields (90.0)
    pub label_width: f32,
    /// Progress bar track height (8.0)
    pub progress_height: f32,
}

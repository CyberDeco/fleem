//! Variant, Size, and Fill enums with palette-based color resolution

use floem::prelude::Color;

use crate::palette::Palette;

/// Semantic variant for UI elements.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Neutral,
    Ghost,
    Link,
}

/// Size scale for UI elements.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Size {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

/// Fill mode for UI elements.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fill {
    #[default]
    Filled,
    Outlined,
}

/// Resolved colors for a variant + fill combination.
#[derive(Debug, Clone, Copy)]
pub struct VariantColors {
    pub bg: Color,
    pub bg_hover: Color,
    pub fg: Color,
    pub border: Color,
}

impl Variant {
    /// Resolve this variant to concrete colors from the palette.
    pub fn resolve(self, fill: Fill, palette: &Palette) -> VariantColors {
        let c = &palette.colors;

        match fill {
            Fill::Filled => match self {
                Variant::Primary => VariantColors {
                    bg: c.accent,
                    bg_hover: c.accent_hover,
                    fg: c.accent_fg,
                    border: c.accent,
                },
                Variant::Secondary => VariantColors {
                    bg: c.bg_elevated,
                    bg_hover: c.bg_hover,
                    fg: c.text_primary,
                    border: c.border,
                },
                Variant::Success => VariantColors {
                    bg: c.success,
                    bg_hover: c.success,
                    fg: c.success_fg,
                    border: c.success,
                },
                Variant::Warning => VariantColors {
                    bg: c.warning,
                    bg_hover: c.warning,
                    fg: c.warning_fg,
                    border: c.warning,
                },
                Variant::Error => VariantColors {
                    bg: c.error,
                    bg_hover: c.error,
                    fg: c.error_fg,
                    border: c.error,
                },
                Variant::Info => VariantColors {
                    bg: c.info,
                    bg_hover: c.info,
                    fg: c.info_fg,
                    border: c.info,
                },
                Variant::Neutral => VariantColors {
                    bg: c.neutral,
                    bg_hover: c.neutral_hover,
                    fg: c.neutral_fg,
                    border: c.neutral,
                },
                Variant::Ghost => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.bg_hover,
                    fg: c.text_primary,
                    border: Color::TRANSPARENT,
                },
                Variant::Link => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: Color::TRANSPARENT,
                    fg: c.text_link,
                    border: Color::TRANSPARENT,
                },
            },
            Fill::Outlined => match self {
                Variant::Primary => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.accent,
                    fg: c.accent,
                    border: c.accent,
                },
                Variant::Secondary => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.bg_hover,
                    fg: c.text_primary,
                    border: c.border_strong,
                },
                Variant::Success => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.success_bg,
                    fg: c.success,
                    border: c.success,
                },
                Variant::Warning => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.warning_bg,
                    fg: c.warning,
                    border: c.warning,
                },
                Variant::Error => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.error_bg,
                    fg: c.error,
                    border: c.error,
                },
                Variant::Info => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.info_bg,
                    fg: c.info,
                    border: c.info,
                },
                Variant::Neutral => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.neutral,
                    fg: c.neutral_fg,
                    border: c.neutral,
                },
                Variant::Ghost => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: c.bg_hover,
                    fg: c.text_primary,
                    border: Color::TRANSPARENT,
                },
                Variant::Link => VariantColors {
                    bg: Color::TRANSPARENT,
                    bg_hover: Color::TRANSPARENT,
                    fg: c.text_link,
                    border: Color::TRANSPARENT,
                },
            },
        }
    }
}

impl Size {
    /// Font size for this size level.
    pub fn font_size(self, palette: &Palette) -> f32 {
        let t = &palette.typography;
        match self {
            Size::Large => t.font_heading,
            Size::Normal => t.font_body,
            Size::Small => t.font_label,
            Size::Tiny => t.font_tiny,
        }
    }

    /// Padding as `(vertical, horizontal)` for this size level.
    pub fn padding(self, palette: &Palette) -> (f32, f32) {
        let s = &palette.spacing;
        match self {
            Size::Large => (s.pad_lg, s.pad_xl),
            Size::Normal => (s.pad_md, s.pad_lg),
            Size::Small => (s.pad_sm, s.pad_md),
            Size::Tiny => (s.pad_xs, s.pad_sm),
        }
    }

    /// Border radius for this size level.
    pub fn radius(self, palette: &Palette) -> f32 {
        let s = &palette.spacing;
        match self {
            Size::Large => s.radius_xl,
            Size::Normal => s.radius_lg,
            Size::Small => s.radius_md,
            Size::Tiny => s.radius_sm,
        }
    }
}

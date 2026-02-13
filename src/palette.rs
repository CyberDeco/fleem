//! Palette — bundles all design tokens with built-in dark/light presets

use floem::prelude::Color;

use crate::tokens::{ColorTokens, SpacingTokens, TypographyTokens};

/// A complete set of design tokens for one appearance mode.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub colors: ColorTokens,
    pub typography: TypographyTokens,
    pub spacing: SpacingTokens,
}

impl Palette {
    /// Built-in dark palette. Values match MacPak's dark theme.
    pub const fn dark() -> Self {
        Self {
            colors: ColorTokens {
                // Backgrounds
                bg_base: Color::rgb8(30, 30, 30),
                bg_surface: Color::rgb8(38, 38, 38),
                bg_elevated: Color::rgb8(50, 50, 50),
                bg_hover: Color::rgb8(60, 60, 60),
                bg_selected: Color::rgb8(70, 70, 70),
                bg_input: Color::rgb8(45, 45, 45),
                bg_disabled: Color::rgb8(40, 40, 40),
                bg_overlay: Color::rgba8(0, 0, 0, 100),

                // Text
                text_primary: Color::WHITE,
                text_secondary: Color::rgb8(180, 180, 180),
                text_muted: Color::rgb8(128, 128, 128),
                text_inverse: Color::rgb8(30, 30, 30),
                text_disabled: Color::rgb8(90, 90, 90),
                text_link: Color::rgb8(100, 180, 255),

                // Borders
                border: Color::rgb8(60, 60, 60),
                border_strong: Color::rgb8(80, 80, 80),
                border_focus: Color::rgb8(66, 165, 245),

                // Accent
                accent: Color::rgb8(33, 150, 243),
                accent_hover: Color::rgb8(66, 165, 245),
                accent_fg: Color::WHITE,

                // Success
                success: Color::rgb8(46, 125, 50),
                success_bg: Color::rgb8(30, 60, 35),
                success_fg: Color::rgb8(129, 199, 132),

                // Error
                error: Color::rgb8(211, 47, 47),
                error_bg: Color::rgb8(60, 30, 30),
                error_fg: Color::rgb8(239, 154, 154),

                // Warning
                warning: Color::rgb8(255, 160, 0),
                warning_bg: Color::rgb8(60, 50, 30),
                warning_fg: Color::rgb8(255, 213, 79),

                // Info
                info: Color::rgb8(33, 150, 243),
                info_bg: Color::rgb8(30, 45, 60),
                info_fg: Color::rgb8(144, 202, 249),

                // Neutral
                neutral: Color::rgb8(80, 80, 80),
                neutral_hover: Color::rgb8(100, 100, 100),
                neutral_fg: Color::rgb8(200, 200, 200),
            },
            typography: TypographyTokens {
                font_title: 18.0,
                font_heading: 14.0,
                font_body: 12.0,
                font_label: 11.0,
                font_small: 10.0,
                font_tiny: 9.0,
                font_mono: "monospace",
            },
            spacing: SpacingTokens {
                pad_xs: 4.0,
                pad_sm: 6.0,
                pad_md: 8.0,
                pad_lg: 12.0,
                pad_xl: 16.0,

                gap_sm: 8.0,
                gap_md: 12.0,
                gap_lg: 16.0,

                radius_sm: 3.0,
                radius_md: 4.0,
                radius_lg: 6.0,
                radius_xl: 8.0,

                border_width: 1.0,
                border_width_thick: 2.0,

                input_min_width: 150.0,
                label_width: 90.0,
                progress_height: 8.0,
            },
        }
    }

    /// Built-in light palette. Values match MacPak's light theme.
    pub const fn light() -> Self {
        Self {
            colors: ColorTokens {
                // Backgrounds
                bg_base: Color::WHITE,
                bg_surface: Color::rgb8(250, 250, 250),
                bg_elevated: Color::rgb8(245, 245, 245),
                bg_hover: Color::rgb8(235, 235, 235),
                bg_selected: Color::rgb8(225, 225, 225),
                bg_input: Color::WHITE,
                bg_disabled: Color::rgb8(240, 240, 240),
                bg_overlay: Color::rgba8(0, 0, 0, 100),

                // Text
                text_primary: Color::rgb8(30, 30, 30),
                text_secondary: Color::rgb8(80, 80, 80),
                text_muted: Color::rgb8(128, 128, 128),
                text_inverse: Color::WHITE,
                text_disabled: Color::rgb8(180, 180, 180),
                text_link: Color::rgb8(25, 118, 210),

                // Borders
                border: Color::rgb8(220, 220, 220),
                border_strong: Color::rgb8(200, 200, 200),
                border_focus: Color::rgb8(25, 118, 210),

                // Accent
                accent: Color::rgb8(25, 118, 210),
                accent_hover: Color::rgb8(21, 101, 192),
                accent_fg: Color::WHITE,

                // Success
                success: Color::rgb8(46, 125, 50),
                success_bg: Color::rgb8(232, 245, 233),
                success_fg: Color::rgb8(46, 125, 50),

                // Error
                error: Color::rgb8(180, 30, 30),
                error_bg: Color::rgb8(255, 235, 235),
                error_fg: Color::rgb8(180, 30, 30),

                // Warning
                warning: Color::rgb8(230, 140, 0),
                warning_bg: Color::rgb8(255, 248, 225),
                warning_fg: Color::rgb8(230, 140, 0),

                // Info
                info: Color::rgb8(25, 118, 210),
                info_bg: Color::rgb8(227, 242, 253),
                info_fg: Color::rgb8(25, 118, 210),

                // Neutral
                neutral: Color::rgb8(224, 224, 224),
                neutral_hover: Color::rgb8(200, 200, 200),
                neutral_fg: Color::rgb8(55, 65, 81),
            },
            typography: TypographyTokens {
                font_title: 18.0,
                font_heading: 14.0,
                font_body: 12.0,
                font_label: 11.0,
                font_small: 10.0,
                font_tiny: 9.0,
                font_mono: "monospace",
            },
            spacing: SpacingTokens {
                pad_xs: 4.0,
                pad_sm: 6.0,
                pad_md: 8.0,
                pad_lg: 12.0,
                pad_xl: 16.0,

                gap_sm: 8.0,
                gap_md: 12.0,
                gap_lg: 16.0,

                radius_sm: 3.0,
                radius_md: 4.0,
                radius_lg: 6.0,
                radius_xl: 8.0,

                border_width: 1.0,
                border_width_thick: 2.0,

                input_min_width: 150.0,
                label_width: 90.0,
                progress_height: 8.0,
            },
        }
    }
}

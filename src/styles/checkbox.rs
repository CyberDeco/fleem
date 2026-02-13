//! Checkbox style recipe

use floem::style::Style;

use crate::palette::Palette;
use crate::variant::Variant;

/// Style recipe for checkbox containers (the row wrapping checkbox + label).
///
/// Provides consistent alignment and cursor. The variant controls the
/// accent color used for the checked state.
pub fn checkbox_style(palette: &Palette, variant: Variant) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    let accent = match variant {
        Variant::Primary => c.accent,
        Variant::Success => c.success,
        Variant::Warning => c.warning,
        Variant::Error => c.error,
        Variant::Info => c.info,
        _ => c.accent,
    };

    Style::new()
        .items_center()
        .gap(sp.pad_sm)
        .font_size(palette.typography.font_body)
        .color(c.text_primary)
        .cursor(floem::style::CursorStyle::Pointer)
        .focus(|s| s.border_color(accent))
        .disabled(|s| {
            s.color(c.text_disabled)
                .cursor(floem::style::CursorStyle::Default)
        })
}

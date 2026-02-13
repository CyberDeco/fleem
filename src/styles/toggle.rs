//! Toggle switch style recipe

use floem::style::Style;

use crate::palette::Palette;
use crate::variant::Variant;

/// Style recipe for toggle switch containers.
///
/// The variant controls the "on" accent color.
pub fn toggle_style(palette: &Palette, variant: Variant) -> Style {
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
        .gap(sp.pad_md)
        .font_size(palette.typography.font_body)
        .color(c.text_primary)
        .cursor(floem::style::CursorStyle::Pointer)
        .focus(|s| s.border_color(accent))
        .disabled(|s| {
            s.color(c.text_disabled)
                .cursor(floem::style::CursorStyle::Default)
        })
}

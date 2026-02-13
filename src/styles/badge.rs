//! Badge style recipes

use floem::style::Style;

use crate::palette::Palette;
use crate::variant::Variant;

/// Style recipe for variant-colored badges.
///
/// Compact inline element with tinted background and matching text.
pub fn badge_style(palette: &Palette, variant: Variant) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    let (bg, fg) = match variant {
        Variant::Primary => (c.accent, c.accent_fg),
        Variant::Success => (c.success_bg, c.success_fg),
        Variant::Warning => (c.warning_bg, c.warning_fg),
        Variant::Error => (c.error_bg, c.error_fg),
        Variant::Info => (c.info_bg, c.info_fg),
        _ => (c.bg_elevated, c.text_secondary),
    };

    Style::new()
        .padding_horiz(sp.pad_lg)
        .padding_vert(sp.pad_sm)
        .background(bg)
        .color(fg)
        .border_radius(sp.radius_md)
        .font_size(palette.typography.font_body)
}

/// Style recipe for a boolean status badge (success or error).
///
/// `ok = true` renders as success, `ok = false` renders as error.
pub fn status_badge_style(palette: &Palette, ok: bool) -> Style {
    if ok {
        badge_style(palette, Variant::Success)
    } else {
        badge_style(palette, Variant::Error)
    }
}

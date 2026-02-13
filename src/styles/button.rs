//! Button style recipe

use floem::style::Style;

use crate::palette::Palette;
use crate::variant::{Fill, Size, Variant};

/// Style recipe for buttons.
///
/// Applies background, foreground, border, padding, radius, and
/// hover/disabled/focus states based on the variant, size, and fill.
pub fn button_style(palette: &Palette, variant: Variant, size: Size, fill: Fill) -> Style {
    let vc = variant.resolve(fill, palette);
    let (pad_v, pad_h) = size.padding(palette);
    let radius = size.radius(palette);
    let font = size.font_size(palette);
    let c = &palette.colors;

    Style::new()
        .padding_vert(pad_v)
        .padding_horiz(pad_h)
        .background(vc.bg)
        .color(vc.fg)
        .border(palette.spacing.border_width)
        .border_color(vc.border)
        .border_radius(radius)
        .font_size(font)
        .cursor(floem::style::CursorStyle::Pointer)
        .hover(|s| s.background(vc.bg_hover))
        .disabled(|s| {
            s.background(c.bg_disabled)
                .color(c.text_disabled)
                .border_color(c.border)
                .cursor(floem::style::CursorStyle::Default)
        })
        .focus(|s| {
            s.border_color(c.border_focus)
                .border(palette.spacing.border_width_thick)
        })
}

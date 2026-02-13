//! Text input style recipe

use floem::style::Style;

use crate::palette::Palette;

/// Style recipe for text inputs.
///
/// Applies background, border, padding, min-width, and focus/disabled states.
pub fn input_style(palette: &Palette) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    Style::new()
        .padding_vert(sp.pad_sm)
        .padding_horiz(sp.pad_md)
        .background(c.bg_input)
        .color(c.text_primary)
        .border(sp.border_width)
        .border_color(c.border)
        .border_radius(sp.radius_md)
        .font_size(palette.typography.font_body)
        .min_width(sp.input_min_width)
        .focus(|s| s.border_color(c.border_focus))
        .disabled(|s| {
            s.background(c.bg_disabled)
                .color(c.text_disabled)
                .border_color(c.border)
        })
}

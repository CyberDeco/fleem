//! Card style recipe

use floem::style::Style;

use crate::palette::Palette;

/// Style recipe for card/panel containers.
///
/// Full width, surface background, border, rounded corners.
pub fn card_style(palette: &Palette) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    Style::new()
        .width_full()
        .padding(sp.pad_xl)
        .background(c.bg_surface)
        .border(sp.border_width)
        .border_color(c.border)
        .border_radius(sp.radius_xl)
}

//! Drop zone style recipe

use floem::style::Style;

use crate::palette::Palette;

/// Style recipe for drag-and-drop target zones.
///
/// Dashed border, centered content, subtle background.
pub fn drop_zone_style(palette: &Palette) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    Style::new()
        .flex_grow(1.0)
        .min_height(120.0)
        .padding(sp.pad_xl)
        .items_center()
        .justify_center()
        .background(c.bg_surface)
        .border(sp.border_width_thick)
        .border_color(c.border)
        .border_radius(sp.radius_xl)
}

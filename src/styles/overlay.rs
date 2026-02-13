//! Modal/overlay style recipes

use floem::style::{Display, Position, Style};

use crate::palette::Palette;

/// Style recipe for a modal backdrop.
///
/// Covers the entire parent (absolute positioned), semi-transparent background.
/// When `visible` is false, the element is hidden via `display: none`.
pub fn modal_backdrop_style(palette: &Palette, visible: bool) -> Style {
    let c = &palette.colors;

    if visible {
        Style::new()
            .position(Position::Absolute)
            .inset_top(0.0)
            .inset_left(0.0)
            .inset_bottom(0.0)
            .inset_right(0.0)
            .items_center()
            .justify_center()
            .background(c.bg_overlay)
            .z_index(100)
    } else {
        Style::new().display(Display::None)
    }
}

/// Style recipe for a modal card (the content box inside the backdrop).
pub fn modal_card_style(palette: &Palette) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    Style::new()
        .padding(sp.pad_xl * 1.5)
        .background(c.bg_surface)
        .border(sp.border_width)
        .border_color(c.border_strong)
        .border_radius(sp.radius_xl)
        .width(500.0)
}

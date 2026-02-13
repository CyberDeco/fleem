//! Progress bar style recipes

use floem::style::Style;

use crate::palette::Palette;

/// Style recipe for the progress bar track (background).
pub fn progress_track_style(palette: &Palette) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    Style::new()
        .width_full()
        .height(sp.progress_height)
        .background(c.border)
        .border_radius(sp.radius_md)
}

/// Style recipe for the progress bar fill.
///
/// `pct` is the progress percentage (0.0–100.0), used to set width.
pub fn progress_fill_style(palette: &Palette, pct: f64) -> Style {
    let c = &palette.colors;
    let sp = &palette.spacing;

    Style::new()
        .height_full()
        .width_pct(pct)
        .background(c.success)
        .border_radius(sp.radius_md)
}

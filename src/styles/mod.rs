//! Style recipe functions
//!
//! Each function takes a `&Palette` (plus relevant enums) and returns a `Style`.
//! No allocations, no trait methods — just plain functions.

mod badge;
mod button;
mod card;
mod checkbox;
mod drop_zone;
mod input;
mod overlay;
mod progress;
mod toggle;

pub use badge::{badge_style, status_badge_style};
pub use button::button_style;
pub use card::card_style;
pub use checkbox::checkbox_style;
pub use drop_zone::drop_zone_style;
pub use input::input_style;
pub use overlay::{modal_backdrop_style, modal_card_style};
pub use progress::{progress_fill_style, progress_track_style};
pub use toggle::toggle_style;

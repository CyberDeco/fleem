//! # fleem
//!
//! Design tokens and style recipes for floem 0.2 applications.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use fleem::{Appearance, Variant, Size, Fill, init, palette, styles};
//!
//! // Initialize once at startup:
//! let _appearance = init(Appearance::System);
//!
//! // Then use palette() + style recipes in your views.
//! ```

mod palette;
pub mod styles;
mod system;
mod theme;
pub mod tokens;
mod variant;

pub use palette::Palette;
pub use theme::{appearance_signal, init, init_with, palette, Appearance, ResolvedAppearance, ThemeDef};
pub use variant::{Fill, Size, Variant, VariantColors};

#[cfg(test)]
mod tests {
    use super::*;
    use floem::prelude::Color;

    #[test]
    fn dark_and_light_palettes_differ() {
        let dark = Palette::dark();
        let light = Palette::light();

        assert_ne!(dark.colors.bg_base, light.colors.bg_base);
        assert_ne!(dark.colors.text_primary, light.colors.text_primary);
        assert_ne!(dark.colors.border, light.colors.border);
    }

    #[test]
    fn all_variant_fill_combinations_resolve() {
        let palette = Palette::dark();
        let variants = [
            Variant::Primary,
            Variant::Secondary,
            Variant::Success,
            Variant::Warning,
            Variant::Error,
            Variant::Info,
            Variant::Neutral,
            Variant::Ghost,
            Variant::Link,
        ];
        let fills = [Fill::Filled, Fill::Outlined];

        for variant in &variants {
            for fill in &fills {
                let vc = variant.resolve(*fill, &palette);
                match variant {
                    Variant::Ghost | Variant::Link => {
                        assert_eq!(
                            vc.bg,
                            Color::TRANSPARENT,
                            "{variant:?} {fill:?} should have transparent bg"
                        );
                    }
                    _ => {
                        if *fill == Fill::Filled {
                            assert_ne!(
                                vc.bg,
                                Color::TRANSPARENT,
                                "{variant:?} Filled should have non-transparent bg"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn size_font_sizes_are_ordered() {
        let palette = Palette::dark();
        let tiny = Size::Tiny.font_size(&palette);
        let small = Size::Small.font_size(&palette);
        let normal = Size::Normal.font_size(&palette);
        let large = Size::Large.font_size(&palette);

        assert!(tiny < small, "Tiny ({tiny}) should be < Small ({small})");
        assert!(small < normal, "Small ({small}) should be < Normal ({normal})");
        assert!(normal < large, "Normal ({normal}) should be < Large ({large})");
    }

    #[test]
    fn size_padding_is_ordered() {
        let palette = Palette::dark();
        let (tv, th) = Size::Tiny.padding(&palette);
        let (sv, sh) = Size::Small.padding(&palette);
        let (nv, nh) = Size::Normal.padding(&palette);
        let (lv, lh) = Size::Large.padding(&palette);

        assert!(tv < sv && th < sh, "Tiny padding should be < Small");
        assert!(sv < nv && sh < nh, "Small padding should be < Normal");
        assert!(nv < lv && nh < lh, "Normal padding should be < Large");
    }

    #[test]
    fn appearance_serde_roundtrip() {
        let values = [Appearance::Dark, Appearance::Light, Appearance::System];
        for value in &values {
            let json = serde_json::to_string(value).unwrap();
            let back: Appearance = serde_json::from_str(&json).unwrap();
            assert_eq!(*value, back);
        }
    }

    #[test]
    fn style_recipes_dont_panic() {
        let palette = Palette::dark();
        let variants = [
            Variant::Primary,
            Variant::Secondary,
            Variant::Success,
            Variant::Warning,
            Variant::Error,
            Variant::Info,
            Variant::Neutral,
            Variant::Ghost,
            Variant::Link,
        ];
        let sizes = [Size::Large, Size::Normal, Size::Small, Size::Tiny];
        let fills = [Fill::Filled, Fill::Outlined];

        for variant in &variants {
            for size in &sizes {
                for fill in &fills {
                    let _ = styles::button_style(&palette, *variant, *size, *fill);
                }
            }
        }

        let _ = styles::input_style(&palette);
        let _ = styles::card_style(&palette);
        let _ = styles::drop_zone_style(&palette);
        let _ = styles::progress_track_style(&palette);
        let _ = styles::progress_fill_style(&palette, 50.0);
        let _ = styles::modal_backdrop_style(&palette, true);
        let _ = styles::modal_backdrop_style(&palette, false);
        let _ = styles::modal_card_style(&palette);

        for variant in &variants {
            let _ = styles::badge_style(&palette, *variant);
            let _ = styles::checkbox_style(&palette, *variant);
            let _ = styles::toggle_style(&palette, *variant);
        }

        let _ = styles::status_badge_style(&palette, true);
        let _ = styles::status_badge_style(&palette, false);
    }
}

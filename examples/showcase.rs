//! Showcase example — renders buttons in a few Variant × Size × Fill combinations.

use floem::event::EventListener;
use floem::prelude::*;

use fleem::styles::button_style;
use fleem::{Appearance, Fill, Size, Variant};

fn button_row(
    variant_name: &'static str,
    variant: Variant,
    fill: Fill,
) -> impl IntoView {
    let p = fleem::palette();
    h_stack((
        button(format!("{variant_name} Large"))
            .style(move |s| s.apply(button_style(&p, variant, Size::Large, fill))),
        button(format!("{variant_name} Normal"))
            .style(move |s| s.apply(button_style(&p, variant, Size::Normal, fill))),
        button(format!("{variant_name} Small"))
            .style(move |s| s.apply(button_style(&p, variant, Size::Small, fill))),
        button(format!("{variant_name} Tiny"))
            .style(move |s| s.apply(button_style(&p, variant, Size::Tiny, fill))),
    ))
    .style(|s| s.gap(8.0).items_center())
}

fn main() {
    let _appearance = fleem::init(Appearance::Dark);

    let view = v_stack((
        // Filled section
        label(|| "Filled".to_string())
            .style(|s| s.font_size(18.0).font_weight(floem::text::Weight::BOLD).margin_bottom(8.0)),
        button_row("Primary", Variant::Primary, Fill::Filled),
        button_row("Secondary", Variant::Secondary, Fill::Filled),
        button_row("Success", Variant::Success, Fill::Filled),
        button_row("Error", Variant::Error, Fill::Filled),
        button_row("Ghost", Variant::Ghost, Fill::Filled),
        // Outlined section
        label(|| "Outlined".to_string())
            .style(|s| s.font_size(18.0).font_weight(floem::text::Weight::BOLD).margin_top(16.0).margin_bottom(8.0)),
        button_row("Primary", Variant::Primary, Fill::Outlined),
        button_row("Secondary", Variant::Secondary, Fill::Outlined),
        button_row("Success", Variant::Success, Fill::Outlined),
        button_row("Error", Variant::Error, Fill::Outlined),
    ))
    .style(move |s| {
        let p = fleem::palette();
        s.padding(24.0)
            .gap(6.0)
            .background(p.colors.bg_base)
            .color(p.colors.text_primary)
            .width_full()
            .height_full()
    });

    floem::launch(move || {
        view.on_event_stop(EventListener::WindowClosed, |_| {
            floem::quit_app();
        })
    });
}

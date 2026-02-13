//! Theme switch example — toggle between Dark, Light, and System appearance live.

use floem::event::EventListener;
use floem::prelude::*;
use floem::text::Weight;

use fleem::styles::{button_style, card_style, input_style};
use fleem::{Appearance, Fill, Size, Variant};

fn main() {
    let appearance_signal = fleem::init(Appearance::Dark);

    let view = v_stack((
        // Title
        label(|| "Theme Switcher".to_string())
            .style(move |s| {
                let p = fleem::palette();
                s.font_size(p.typography.font_title).font_weight(Weight::BOLD)
            }),
        // Theme buttons
        h_stack((
            button("Dark")
                .action(move || appearance_signal.set(Appearance::Dark))
                .style(move |s| {
                    let p = fleem::palette();
                    s.apply(button_style(&p, Variant::Primary, Size::Normal, Fill::Filled))
                }),
            button("Light")
                .action(move || appearance_signal.set(Appearance::Light))
                .style(move |s| {
                    let p = fleem::palette();
                    s.apply(button_style(&p, Variant::Secondary, Size::Normal, Fill::Filled))
                }),
            button("System")
                .action(move || appearance_signal.set(Appearance::System))
                .style(move |s| {
                    let p = fleem::palette();
                    s.apply(button_style(&p, Variant::Neutral, Size::Normal, Fill::Outlined))
                }),
        ))
        .style(|s| s.gap(8.0)),
        // Current appearance label
        label(move || {
            let mode = appearance_signal.get();
            format!("Current: {mode:?}")
        })
        .style(move |s| {
            let p = fleem::palette();
            s.font_size(p.typography.font_body).color(p.colors.text_muted)
        }),
        // Sample card with content that reacts to theme
        container(
            v_stack((
                label(|| "Sample Card".to_string())
                    .style(move |s| {
                        let p = fleem::palette();
                        s.font_size(p.typography.font_heading).font_weight(Weight::BOLD)
                    }),
                label(|| "This card and its contents automatically update when the theme changes.".to_string())
                    .style(move |s| {
                        let p = fleem::palette();
                        s.font_size(p.typography.font_body).color(p.colors.text_secondary)
                    }),
                text_input(RwSignal::new("Type something here...".to_string()))
                    .style(move |s| {
                        let p = fleem::palette();
                        s.apply(input_style(&p))
                    }),
                h_stack((
                    button("Primary")
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(&p, Variant::Primary, Size::Normal, Fill::Filled))
                        }),
                    button("Success")
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(&p, Variant::Success, Size::Normal, Fill::Filled))
                        }),
                    button("Error")
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(&p, Variant::Error, Size::Normal, Fill::Outlined))
                        }),
                ))
                .style(|s| s.gap(8.0)),
            ))
            .style(|s| s.gap(12.0)),
        )
        .style(move |s| {
            let p = fleem::palette();
            s.apply(card_style(&p))
        }),
    ))
    .style(move |s| {
        let p = fleem::palette();
        s.padding(24.0)
            .gap(16.0)
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

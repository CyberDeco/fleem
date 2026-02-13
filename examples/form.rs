//! Form example — inputs, checkboxes, toggles, and labels.

use floem::event::EventListener;
use floem::prelude::*;
use floem::text::Weight;

use fleem::styles::{button_style, card_style, checkbox_style, input_style, toggle_style};
use fleem::{Appearance, Fill, Size, Variant};

fn labeled_row(label_text: &'static str, child: impl IntoView + 'static) -> impl IntoView {
    h_stack((
        label(move || label_text.to_string()).style(move |s| {
            let p = fleem::palette();
            s.width(p.spacing.label_width)
                .font_size(p.typography.font_label)
                .color(p.colors.text_secondary)
        }),
        child,
    ))
    .style(|s| s.items_center().gap(8.0))
}

fn main() {
    let _appearance = fleem::init(Appearance::Dark);

    let name = RwSignal::new("".to_string());
    let email = RwSignal::new("".to_string());
    let notify = RwSignal::new(true);
    let agree = RwSignal::new(false);

    let view = v_stack((
        label(|| "Form Example".to_string())
            .style(move |s| {
                let p = fleem::palette();
                s.font_size(p.typography.font_title).font_weight(Weight::BOLD)
            }),
        // Form card
        container(
            v_stack((
                label(|| "User Details".to_string())
                    .style(move |s| {
                        let p = fleem::palette();
                        s.font_size(p.typography.font_heading).font_weight(Weight::BOLD)
                    }),
                // Name input
                labeled_row(
                    "Name",
                    text_input(name).style(move |s| {
                        let p = fleem::palette();
                        s.apply(input_style(&p))
                    }),
                ),
                // Email input
                labeled_row(
                    "Email",
                    text_input(email).style(move |s| {
                        let p = fleem::palette();
                        s.apply(input_style(&p))
                    }),
                ),
                // Checkbox
                labeled_row(
                    "Terms",
                    h_stack((
                        checkbox(move || agree.get())
                            .on_update(move |v| agree.set(v))
                            .style(|s| s.cursor(floem::style::CursorStyle::Pointer)),
                        label(|| "I agree to the terms".to_string()),
                    ))
                    .on_click_stop(move |_| agree.update(|v| *v = !*v))
                    .style(move |s| {
                        let p = fleem::palette();
                        s.apply(checkbox_style(&p, Variant::Primary))
                    }),
                ),
                // Toggle
                labeled_row(
                    "Notifications",
                    h_stack((
                        checkbox(move || notify.get())
                            .on_update(move |v| notify.set(v))
                            .style(|s| s.cursor(floem::style::CursorStyle::Pointer)),
                        label(move || {
                            if notify.get() { "Enabled" } else { "Disabled" }.to_string()
                        }),
                    ))
                    .on_click_stop(move |_| notify.update(|v| *v = !*v))
                    .style(move |s| {
                        let p = fleem::palette();
                        s.apply(toggle_style(&p, Variant::Success))
                    }),
                ),
                // Submit button
                h_stack((
                    empty().style(move |s| {
                        let p = fleem::palette();
                        s.width(p.spacing.label_width)
                    }),
                    button("Submit")
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(&p, Variant::Primary, Size::Normal, Fill::Filled))
                        }),
                    button("Cancel")
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(&p, Variant::Ghost, Size::Normal, Fill::Filled))
                        }),
                ))
                .style(|s| s.gap(8.0).items_center()),
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

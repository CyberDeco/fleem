//! Progress example — animated progress bar with a modal overlay.

use floem::action::exec_after;
use floem::event::EventListener;
use floem::prelude::*;
use floem::text::Weight;
use std::time::Duration;

use fleem::styles::{
    button_style, card_style, modal_backdrop_style, modal_card_style, progress_fill_style,
    progress_track_style,
};
use fleem::{Appearance, Fill, Size, Variant};

fn main() {
    let _appearance = fleem::init(Appearance::Dark);

    let progress_pct = RwSignal::new(0.0_f64);
    let show_modal = RwSignal::new(false);
    let running = RwSignal::new(false);

    // Tick function that advances progress
    fn tick(pct: RwSignal<f64>, running: RwSignal<bool>) {
        exec_after(Duration::from_millis(50), move |_| {
            if !running.get_untracked() {
                return;
            }
            let current = pct.get_untracked();
            if current >= 100.0 {
                running.set(false);
            } else {
                pct.set(current + 1.0);
                tick(pct, running);
            }
        });
    }

    // Modal overlay — absolutely positioned, hidden via display:none when inactive
    let modal_overlay = dyn_container(
        move || show_modal.get(),
        move |visible| {
            if visible {
                container(
                    v_stack((
                        label(|| "Modal Title".to_string()).style(move |s| {
                            let p = fleem::palette();
                            s.font_size(p.typography.font_heading)
                                .font_weight(Weight::BOLD)
                        }),
                        label(|| {
                            "This is a modal dialog styled with fleem's overlay recipes."
                                .to_string()
                        })
                        .style(move |s| {
                            let p = fleem::palette();
                            s.font_size(p.typography.font_body)
                                .color(p.colors.text_secondary)
                        }),
                        h_stack((
                            button("Close")
                                .action(move || show_modal.set(false))
                                .style(move |s| {
                                    let p = fleem::palette();
                                    s.apply(button_style(
                                        &p,
                                        Variant::Primary,
                                        Size::Normal,
                                        Fill::Filled,
                                    ))
                                }),
                            button("Cancel")
                                .action(move || show_modal.set(false))
                                .style(move |s| {
                                    let p = fleem::palette();
                                    s.apply(button_style(
                                        &p,
                                        Variant::Ghost,
                                        Size::Normal,
                                        Fill::Filled,
                                    ))
                                }),
                        ))
                        .style(|s| s.gap(8.0)),
                    ))
                    .style(|s| s.gap(12.0)),
                )
                .style(move |s| {
                    let p = fleem::palette();
                    s.apply(modal_card_style(&p))
                })
                .into_any()
            } else {
                empty().into_any()
            }
        },
    )
    .style(move |s| {
        let p = fleem::palette();
        s.apply(modal_backdrop_style(&p, show_modal.get()))
    });

    let root = v_stack((
        label(|| "Progress & Overlay".to_string()).style(move |s| {
            let p = fleem::palette();
            s.font_size(p.typography.font_title).font_weight(Weight::BOLD)
        }),
        // Inline progress card
        container(
            v_stack((
                label(|| "Inline Progress".to_string()).style(move |s| {
                    let p = fleem::palette();
                    s.font_size(p.typography.font_heading).font_weight(Weight::BOLD)
                }),
                // Progress bar
                container(
                    container(empty()).style(move |s| {
                        let p = fleem::palette();
                        s.apply(progress_fill_style(&p, progress_pct.get()))
                    }),
                )
                .style(move |s| {
                    let p = fleem::palette();
                    s.apply(progress_track_style(&p))
                }),
                // Percentage label
                label(move || format!("{:.0}%", progress_pct.get())).style(move |s| {
                    let p = fleem::palette();
                    s.font_size(p.typography.font_body).color(p.colors.text_muted)
                }),
                // Controls
                h_stack((
                    button("Start")
                        .action(move || {
                            if !running.get_untracked() {
                                progress_pct.set(0.0);
                                running.set(true);
                                tick(progress_pct, running);
                            }
                        })
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(
                                &p,
                                Variant::Primary,
                                Size::Normal,
                                Fill::Filled,
                            ))
                        }),
                    button("Reset")
                        .action(move || {
                            running.set(false);
                            progress_pct.set(0.0);
                        })
                        .style(move |s| {
                            let p = fleem::palette();
                            s.apply(button_style(
                                &p,
                                Variant::Secondary,
                                Size::Normal,
                                Fill::Filled,
                            ))
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
        // Modal trigger card
        container(
            v_stack((
                label(|| "Modal Overlay".to_string()).style(move |s| {
                    let p = fleem::palette();
                    s.font_size(p.typography.font_heading).font_weight(Weight::BOLD)
                }),
                label(|| "Click the button to show a modal dialog.".to_string()).style(
                    move |s| {
                        let p = fleem::palette();
                        s.font_size(p.typography.font_body)
                            .color(p.colors.text_secondary)
                    },
                ),
                button("Show Modal")
                    .action(move || show_modal.set(true))
                    .style(move |s| {
                        let p = fleem::palette();
                        s.apply(button_style(
                            &p,
                            Variant::Primary,
                            Size::Normal,
                            Fill::Filled,
                        ))
                    }),
            ))
            .style(|s| s.gap(12.0)),
        )
        .style(move |s| {
            let p = fleem::palette();
            s.apply(card_style(&p))
        }),
        // Modal overlay — child of same v_stack, absolutely positioned over it
        modal_overlay,
    ))
    .style(move |s| {
        let p = fleem::palette();
        s.padding(24.0)
            .gap(16.0)
            .background(p.colors.bg_base)
            .color(p.colors.text_primary)
            .width_full()
            .height_full()
            .position(floem::style::Position::Relative)
    });

    floem::launch(move || {
        root.on_event_stop(EventListener::WindowClosed, |_| {
            floem::quit_app();
        })
    });
}

//! Cards example — card layouts with status badges in various variants.

use floem::event::EventListener;
use floem::prelude::*;
use floem::text::Weight;

use fleem::styles::{badge_style, card_style, status_badge_style};
use fleem::{Appearance, Variant};

fn status_card(
    title: &'static str,
    description: &'static str,
    variant: Variant,
    badge_text: &'static str,
) -> impl IntoView {
    container(
        v_stack((
            // Header row with title and badge
            h_stack((
                label(move || title.to_string()).style(move |s| {
                    let p = fleem::palette();
                    s.font_size(p.typography.font_heading).font_weight(Weight::BOLD)
                }),
                empty().style(|s| s.flex_grow(1.0)),
                label(move || badge_text.to_string()).style(move |s| {
                    let p = fleem::palette();
                    s.apply(badge_style(&p, variant))
                }),
            ))
            .style(|s| s.items_center().width_full()),
            // Description
            label(move || description.to_string()).style(move |s| {
                let p = fleem::palette();
                s.font_size(p.typography.font_body).color(p.colors.text_secondary)
            }),
        ))
        .style(|s| s.gap(8.0)),
    )
    .style(move |s| {
        let p = fleem::palette();
        s.apply(card_style(&p))
    })
}

fn main() {
    let _appearance = fleem::init(Appearance::Dark);

    let view = v_stack((
        label(|| "Cards & Badges".to_string())
            .style(move |s| {
                let p = fleem::palette();
                s.font_size(p.typography.font_title).font_weight(Weight::BOLD)
            }),
        // Status cards
        status_card(
            "Build Pipeline",
            "All 42 tests passed, coverage at 94%.",
            Variant::Success,
            "Passing",
        ),
        status_card(
            "API Gateway",
            "Response time degraded above 500ms threshold.",
            Variant::Warning,
            "Degraded",
        ),
        status_card(
            "Database Migration",
            "Migration #47 failed: column type mismatch.",
            Variant::Error,
            "Failed",
        ),
        status_card(
            "CDN Distribution",
            "Cache invalidation in progress across 12 regions.",
            Variant::Info,
            "Syncing",
        ),
        status_card(
            "Feature Flag",
            "Dark mode toggle is available for 30% of users.",
            Variant::Primary,
            "Canary",
        ),
        // Status badge row
        container(
            v_stack((
                label(|| "Status Badges".to_string()).style(move |s| {
                    let p = fleem::palette();
                    s.font_size(p.typography.font_heading).font_weight(Weight::BOLD)
                }),
                h_stack((
                    label(|| "OK".to_string()).style(move |s| {
                        let p = fleem::palette();
                        s.apply(status_badge_style(&p, true))
                    }),
                    label(|| "Error".to_string()).style(move |s| {
                        let p = fleem::palette();
                        s.apply(status_badge_style(&p, false))
                    }),
                    label(|| "Neutral".to_string()).style(move |s| {
                        let p = fleem::palette();
                        s.apply(badge_style(&p, Variant::Neutral))
                    }),
                    label(|| "Ghost".to_string()).style(move |s| {
                        let p = fleem::palette();
                        s.apply(badge_style(&p, Variant::Ghost))
                    }),
                ))
                .style(|s| s.gap(8.0)),
            ))
            .style(|s| s.gap(8.0)),
        )
        .style(move |s| {
            let p = fleem::palette();
            s.apply(card_style(&p))
        }),
    ))
    .style(move |s| {
        let p = fleem::palette();
        s.padding(24.0)
            .gap(12.0)
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

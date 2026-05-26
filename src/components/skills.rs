use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::reveal::observe_reveal;
use crate::skills::all;

#[component]
pub fn Skills() -> impl IntoView {
    // IntersectionObserver target — flips `data-revealed="1"` on the
    // `.skills` container when it enters the viewport, which then kicks
    // off the staggered chip-in keyframe animation.
    let section_ref = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        if let Some(el) = section_ref.get() {
            observe_reveal(el.unchecked_into());
        }
    });

    // Build groups + chips imperatively so we can assign each chip a
    // monotonic CSS variable `--i` for the cascade timing. Doing it in
    // a loop avoids the FnMut/borrow-checker hassle of nested .map().
    let groups = all();
    let mut idx = 0_usize;
    let mut group_views = Vec::with_capacity(groups.len());

    for group in groups {
        let label = group.label;
        let mut chip_views = Vec::with_capacity(group.items.len());

        for skill in group.items {
            let i = idx;
            idx += 1;

            let class = if skill.primary {
                "chip chip--primary"
            } else {
                "chip"
            };

            chip_views.push(view! {
                <span class=class style=format!("--i: {i}")>
                    {skill.name}
                    {skill.primary.then(|| view! { <span class="chip__star">" ★"</span> })}
                </span>
            });
        }

        group_views.push(view! {
            <div class="skills__group">
                <h3 class="skills__label">{label}</h3>
                <div class="skills__chips">{chip_views}</div>
            </div>
        });
    }

    view! {
        <section class="section" id="skills">
            <header class="section__head">
                <span class="section__kicker">"02 — Toolkit"</span>
                <h2 class="section__title">"What I work with"</h2>
                <p class="section__sub">
                    "Daily-driver tools, frameworks, and patterns — Python-first."
                </p>
            </header>

            <div class="skills" node_ref=section_ref>
                {group_views}
            </div>
        </section>
    }
}

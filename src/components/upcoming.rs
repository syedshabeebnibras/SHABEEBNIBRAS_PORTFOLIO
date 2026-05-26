use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::reveal::observe_reveal;
use crate::projects::{all, Status};

#[component]
pub fn Upcoming() -> impl IntoView {
    let upcoming: Vec<_> = all()
        .into_iter()
        .filter(|p| matches!(p.status, Status::Upcoming))
        .collect();

    let section_ref = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        if let Some(el) = section_ref.get() {
            observe_reveal(el.unchecked_into());
        }
    });

    view! {
        <section class="section section--upcoming" id="upcoming">
            <header class="section__head">
                <span class="section__kicker">"05 — In flight"</span>
                <h2 class="section__title">"What's next"</h2>
                <p class="section__sub">"committed. quietly in progress."</p>
            </header>

            <div class="timeline" node_ref=section_ref>
                {upcoming.into_iter().enumerate().map(|(i, p)| {
                    let stack = p.stack;
                    view! {
                        <article
                            class="lane"
                            style=format!("--index: {i}")
                        >
                            <div class="lane__when">{p.stamp}</div>
                            <div class="lane__rail" />
                            <div class="lane__card">
                                <h3>{p.name}</h3>
                                <p class="lane__tag">{p.tagline}</p>
                                <p class="lane__desc">{p.description}</p>
                                <ul class="card__stack">
                                    {stack
                                        .iter()
                                        .map(|s| view! { <li>{*s}</li> })
                                        .collect::<Vec<_>>()}
                                </ul>
                            </div>
                        </article>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

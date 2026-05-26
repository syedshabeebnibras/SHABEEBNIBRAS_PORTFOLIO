use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlVideoElement;

use crate::components::reveal::observe_reveal;
use crate::projects::Project;

#[component]
pub fn ProjectCard(project: Project, index: usize) -> impl IntoView {
    let card_ref = NodeRef::<leptos::html::Article>::new();
    Effect::new(move |_| {
        if let Some(el) = card_ref.get() {
            observe_reveal(el.unchecked_into());
        }
    });

    let video_ref = NodeRef::<leptos::html::Video>::new();

    // `loop` is a Rust reserved keyword and the view! macro won't accept it
    // as a bare attribute. Set the video properties imperatively once mounted.
    Effect::new(move |_| {
        if let Some(v) = video_ref.get() {
            let v: HtmlVideoElement = v.unchecked_into();
            v.set_loop(true);
            v.set_muted(true);
            v.set_autoplay(false);
            let _ = v.set_attribute("playsinline", "true");
        }
    });

    let on_enter = move |_| {
        if let Some(v) = video_ref.get() {
            let v: HtmlVideoElement = v.unchecked_into();
            let _ = v.play();
        }
    };
    let on_leave = move |_| {
        if let Some(v) = video_ref.get() {
            let v: HtmlVideoElement = v.unchecked_into();
            let _ = v.pause();
            v.set_current_time(0.0);
        }
    };

    let status_color = project.status.color_var();
    let status_label = project.status.label();
    let id_str = format!("p-{}", project.id);

    // pull all &'static fields out — these are Copy, so reusing them in the
    // view is fine even after Option fields are consumed by .map().
    let name = project.name;
    let tagline = project.tagline;
    let description = project.description;
    let stamp = project.stamp;
    let stack = project.stack;
    let video_src = project.video_src;
    let poster = project.poster.unwrap_or("");
    let link = project.link;
    let repo = project.repo;

    // Three media modes:
    //   1. video_src present       → <video> hover-plays (default product loop)
    //   2. poster only             → <img> static banner with ken-burns hover
    //   3. neither                 → collapsed media row (role-style cards)
    let has_poster_only = video_src.is_none() && project.poster.is_some();
    let card_class = if video_src.is_some() {
        "card"
    } else if has_poster_only {
        "card card--poster"
    } else {
        "card card--no-video"
    };

    view! {
        <article
            class=card_class
            id=id_str
            node_ref=card_ref
            on:mouseenter=on_enter
            on:mouseleave=on_leave
            style=format!("--accent: {status_color}; --index: {index}")
        >
            <div class="card__media">
                {video_src.map(|src| view! {
                    <video
                        node_ref=video_ref
                        class="card__video"
                        src=src
                        poster=poster
                        preload="metadata"
                    />
                })}
                {(has_poster_only).then(|| view! {
                    <img
                        class="card__poster"
                        src=poster
                        alt=format!("{name} preview")
                        loading="lazy"
                    />
                })}
                <div class="card__status">
                    <span class="card__status-dot"></span>
                    <span>{status_label}</span>
                </div>
                <div class="card__stamp">{stamp}</div>
            </div>

            <div class="card__body">
                <header class="card__head">
                    <h3 class="card__name">{name}</h3>
                    <span class="card__index">{format!("{:02}", index + 1)}</span>
                </header>

                <p class="card__tagline">{tagline}</p>
                <p class="card__desc">{description}</p>

                <ul class="card__stack" aria-label="stack">
                    {stack
                        .iter()
                        .map(|s| view! { <li>{*s}</li> })
                        .collect::<Vec<_>>()}
                </ul>

                <footer class="card__links">
                    {link.map(|l| view! {
                        <a class="card__link" href=l target="_blank" rel="noreferrer">
                            "visit  ↗"
                        </a>
                    })}
                    {repo.map(|r| view! {
                        <a class="card__link card__link--ghost" href=r target="_blank" rel="noreferrer">
                            "source  ↗"
                        </a>
                    })}
                </footer>
            </div>
        </article>
    }
}

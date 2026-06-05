use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::motion::{start_hero_field, wire_kinetic_name};

/// Inject LinkedIn's profile.js into the document body after WASM has mounted.
fn load_linkedin_badge_script() {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let Ok(script) = document.create_element("script") else { return };
    let _ = script.set_attribute("src", "https://platform.linkedin.com/badges/js/profile.js");
    let _ = script.set_attribute("type", "text/javascript");
    let script: web_sys::HtmlScriptElement = script.unchecked_into();
    script.set_async(true);
    script.set_defer(true);
    if let Some(body) = document.body() {
        let _ = body.append_child(&script);
    }
}

/// Build per-glyph spans grouped into `.word` wrappers (so the name only
/// wraps between words on narrow screens, never mid-word). Each letter's
/// `.char` carries a monotonic `--ci` for the staggered entrance wave; the
/// inner `.char__in` is the target of the Rust cursor-physics loop. A shared
/// `counter` threads the index across multiple calls (name + surname).
fn push_words(out: &mut Vec<AnyView>, text: &str, counter: &mut usize) {
    let words: Vec<&str> = text.split(' ').collect();
    for (wi, word) in words.iter().enumerate() {
        if wi > 0 {
            out.push(view! { <span class="char--space">" "</span> }.into_any());
        }
        if word.is_empty() {
            continue;
        }
        let mut inner: Vec<AnyView> = Vec::new();
        for c in word.chars() {
            let ci = *counter;
            *counter += 1;
            inner.push(
                view! {
                    <span class="char" style=format!("--ci:{ci}")>
                        <span class="char__in">{c.to_string()}</span>
                    </span>
                }
                .into_any(),
            );
        }
        out.push(view! { <span class="word">{inner}</span> }.into_any());
    }
}

#[component]
pub fn Hero() -> impl IntoView {
    let name_ref = NodeRef::<leptos::html::H1>::new();
    let field_ref = NodeRef::<leptos::html::Canvas>::new();

    // load the LinkedIn badge widget once on mount
    Effect::new(move |_| {
        load_linkedin_badge_script();
    });

    // wire the per-glyph magnetic/idle physics once the <h1> is in the DOM
    Effect::new(move |_| {
        if let Some(el) = name_ref.get() {
            wire_kinetic_name(el.unchecked_into());
        }
    });

    // start the warm particle field on the backdrop canvas
    Effect::new(move |_| {
        if let Some(c) = field_ref.get() {
            start_hero_field(c.unchecked_into());
        }
    });

    view! {
        <section class="hero" id="top">
            <canvas class="hero__field" node_ref=field_ref aria-hidden="true"></canvas>

            <p class="hero__meta">
                <span class="hero__meta-shimmer">"AI Engineer"</span>
                " · Chicago"
            </p>

            <h1 class="hero__name" tabindex="0" node_ref=name_ref>
                {
                    let mut counter = 0usize;
                    let mut first: Vec<AnyView> = Vec::new();
                    push_words(&mut first, "syed shabeeb ", &mut counter);
                    let mut sur: Vec<AnyView> = Vec::new();
                    push_words(&mut sur, "nibras", &mut counter);
                    view! {
                        {first}
                        <span class="surname">{sur}</span>
                    }
                }

                // hover popover with the LinkedIn badge widget
                <span class="name-popover" aria-hidden="true">
                    <span class="name-popover__hint">"linkedin ↓"</span>
                    <div
                        class="badge-base LI-profile-badge"
                        data-locale="en_US"
                        data-size="large"
                        data-theme="dark"
                        data-type="VERTICAL"
                        data-vanity="syed-shabeeb"
                        data-version="v1"
                    >
                        <a
                            class="badge-base__link LI-simple-link"
                            href="https://www.linkedin.com/in/syed-shabeeb?trk=profile-badge"
                        >
                            "Syed Shabeeb Nibras"
                        </a>
                    </div>
                </span>
            </h1>

            <p class="hero__lede">
                "I build production "
                <em>"llm systems"</em>
                " — retrieval pipelines, multi-agent orchestration, and the quiet "
                "observability that keeps them honest."
            </p>

            <p class="hero__lede--sub">
                "MS Computer Science, DePaul · ex-Salesforce Developer @ Wipro · published ML researcher"
            </p>

            <div class="hero__cta">
                <a class="btn btn--primary" href="#work">
                    <span>"view work"</span>
                    <span class="btn__arrow">"→"</span>
                </a>
                <a class="btn" href="#upcoming">"upcoming"</a>
                <a class="btn" href="#contact">"contact"</a>
            </div>
        </section>
    }
}

use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Inject LinkedIn's profile.js into the document body after WASM has mounted.
/// The script walks the DOM for `.LI-profile-badge` divs and replaces each
/// with the rendered badge iframe. Loaded once per page — async + defer.
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

#[component]
pub fn Hero() -> impl IntoView {
    // The popover below contains the only `.LI-profile-badge` div on the page —
    // the footer's linkedin card is a plain anchor with a static logo. So we
    // load the script exactly once, when this component mounts.
    Effect::new(move |_| {
        load_linkedin_badge_script();
    });

    view! {
        <section class="hero" id="top">
            <p class="hero__meta">"AI Engineer · Chicago"</p>

            <h1 class="hero__name" tabindex="0">
                "syed shabeeb "
                <span class="surname">"nibras"</span>

                // Hover popover with the LinkedIn badge widget — large vertical
                // dark variant. LinkedIn's profile.js (loaded above) finds this
                // div and injects the rendered badge iframe in its place.
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
                "MS Computer Science Graduate · Prior Salesforce Dev @wipro"
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

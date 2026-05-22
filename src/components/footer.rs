use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <section class="section section--contact" id="contact">
            <header class="section__head">
                <span class="section__kicker">"05 — Get in touch"</span>
                <h2 class="section__title">"Let's talk"</h2>
                <p class="section__sub">"always happy to chat about agents, evals, and quiet systems."</p>
            </header>

            <div class="contact-grid">
                <a
                    class="contact-card contact-card--hero contact-card--email"
                    href="mailto:syedshabeebn@gmail.com"
                    aria-label="email syed shabeeb nibras"
                >
                    <div class="contact-card__bg" aria-hidden="true"></div>
                    <span class="contact-card__handle">"@syedshabeebnibras"</span>
                    <div class="contact-card__hero-body">
                        <h3 class="contact-card__title">"email"</h3>
                        <span class="contact-card__hero-line">"syedshabeebn@gmail.com  ↗"</span>
                        <img
                            class="contact-card__hero-badge"
                            src="https://img.shields.io/badge/Gmail-EA4335.svg?style=for-the-badge&logo=Gmail&logoColor=white"
                            alt="Gmail"
                            loading="lazy"
                        />
                    </div>
                </a>

                <a
                    class="contact-card contact-card--hero contact-card--github"
                    href="https://github.com/syedshabeebnibras"
                    target="_blank"
                    rel="noreferrer"
                    aria-label="open github profile in a new tab"
                >
                    <div class="contact-card__bg" aria-hidden="true"></div>
                    <span class="contact-card__handle">"@syedshabeebnibras"</span>
                    <div class="contact-card__hero-body">
                        <h3 class="contact-card__title">"github"</h3>
                        <span class="contact-card__hero-line">"github.com/syedshabeebnibras  ↗"</span>
                        <img
                            class="contact-card__hero-badge"
                            src="https://img.shields.io/badge/GitHub-181717.svg?style=for-the-badge&logo=GitHub&logoColor=white"
                            alt="GitHub"
                            loading="lazy"
                        />
                    </div>
                </a>

                <a
                    class="contact-card contact-card--hero contact-card--linkedin"
                    href="https://www.linkedin.com/in/syed-shabeeb"
                    target="_blank"
                    rel="noreferrer"
                    aria-label="open linkedin profile in a new tab"
                >
                    <div class="contact-card__bg" aria-hidden="true"></div>
                    <span class="contact-card__handle">"@syedshabeebnibras"</span>
                    <div class="contact-card__hero-body">
                        <h3 class="contact-card__title">"linkedin"</h3>
                        <span class="contact-card__hero-line">"linkedin.com/in/syed-shabeeb  ↗"</span>
                        <img
                            class="contact-card__hero-badge contact-card__hero-badge--linkedin"
                            src="https://upload.wikimedia.org/wikipedia/commons/0/01/LinkedIn_Logo.svg"
                            alt="LinkedIn"
                            loading="lazy"
                        />
                    </div>
                </a>
            </div>

            <footer class="root-footer">
                <span class="small muted">
                    "hand-built in rust · wasm · leptos"
                </span>
                <span class="small muted">"© 2026"</span>
            </footer>
        </section>
    }
}

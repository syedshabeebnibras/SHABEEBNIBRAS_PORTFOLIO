use leptos::prelude::*;

use crate::components::project_card::ProjectCard;
use crate::projects::{all, Status};

#[component]
pub fn Volunteering() -> impl IntoView {
    let projects: Vec<_> = all()
        .into_iter()
        .filter(|p| matches!(p.status, Status::Volunteer))
        .collect();
    let count = projects.len();

    view! {
        <section class="section" id="volunteering">
            <header class="section__head">
                <span class="section__kicker">"04 — Volunteering"</span>
                <h2 class="section__title">"Giving back"</h2>
                <p class="section__sub">
                    {format!(
                        "{count} active engagement — nonprofit & community work, same craft as paid work.",
                        count = count
                    )}
                </p>
            </header>

            <div class="grid">
                {projects
                    .into_iter()
                    .enumerate()
                    .map(|(i, p)| view! { <ProjectCard project=p index=i /> })
                    .collect::<Vec<_>>()}
            </div>
        </section>
    }
}

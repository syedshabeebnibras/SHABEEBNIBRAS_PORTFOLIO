use leptos::prelude::*;

use crate::components::project_card::ProjectCard;
use crate::projects::{all, Status};

#[component]
pub fn ProjectsGrid() -> impl IntoView {
    // Selected work = Shipped + Live only. Volunteer and Upcoming have their own sections.
    let projects: Vec<_> = all()
        .into_iter()
        .filter(|p| matches!(p.status, Status::Shipped | Status::Live))
        .collect();
    let count = projects.len();

    view! {
        <section class="section" id="work">
            <header class="section__head">
                <span class="section__kicker">"02 — Selected work"</span>
                <h2 class="section__title">"Things I've shipped"</h2>
                <p class="section__sub">
                    {format!("{count} projects. hover any card for a quiet preview.")}
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

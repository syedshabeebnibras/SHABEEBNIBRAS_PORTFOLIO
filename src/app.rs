use leptos::prelude::*;

use crate::components::{
    experience::Experience, footer::Footer, hero::Hero, projects_grid::ProjectsGrid,
    status_bar::StatusBar, upcoming::Upcoming, volunteering::Volunteering,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <StatusBar />
        <main class="root">
            <Hero />
            <Experience />
            <ProjectsGrid />
            <Volunteering />
            <Upcoming />
            <Footer />
        </main>
    }
}

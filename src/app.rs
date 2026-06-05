use leptos::prelude::*;

use crate::components::motion::wire_tilt_all;
use crate::components::{
    experience::Experience, footer::Footer, hero::Hero, projects_grid::ProjectsGrid,
    skills::Skills, status_bar::StatusBar, upcoming::Upcoming, volunteering::Volunteering,
};

#[component]
pub fn App() -> impl IntoView {
    // After the whole page mounts, attach the 3D cursor-tilt loop to every
    // project card (Work + Volunteering). One pass covers them all.
    Effect::new(move |_| {
        wire_tilt_all();
    });

    view! {
        <StatusBar />
        <main class="root">
            <Hero />
            <Experience />
            <Skills />
            <ProjectsGrid />
            <Volunteering />
            <Upcoming />
            <Footer />
        </main>
    }
}

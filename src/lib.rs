mod components;
mod contexts;
mod data;
mod models;
mod pages;
mod services;
use crate::{
    contexts::{project_context::ProjectContext, repository_context::RepositoryContext},
    pages::{not_found::NotFound, projects_view::ProjectsView, timeline_view::TimelineView},
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());
    let projects = RwSignal::new(data::projects_data::PROJECTS.to_vec());

    provide_context(ProjectContext { projects });
    provide_context(RepositoryContext::default());
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <ConfigProvider theme>
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=ProjectsView />
                    <Route path=path!("/timeline") view=TimelineView />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}

mod components;
mod contexts;
mod data;
mod models;
mod pages;
use crate::{
    contexts::project_context::ProjectContext,
    pages::{home::Home, not_found::NotFound},
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
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <ConfigProvider theme>
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}

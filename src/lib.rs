use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use thaw::*;

mod components;
mod pages;

use crate::pages::{home::Home, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = RwSignal::new(Theme::dark());

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

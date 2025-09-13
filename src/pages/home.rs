use crate::components::{footer::Footer, header::Header, projects::Projects};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div class="container">
                <Header />
                <Projects />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}

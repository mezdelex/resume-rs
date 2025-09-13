use crate::components::counter_btn::MyButton;
use leptos::prelude::*;
use thaw::Button;

use crate::components::sidebar::Sidebar;

#[component]
pub fn Home() -> impl IntoView {
    let open = RwSignal::new(false);

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

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <Sidebar open />
                <Button on_click=move |_| { open.set(true) } />

                <div class="buttons">
                    <MyButton />
                    <MyButton increment=5 />
                </div>

            </div>
        </ErrorBoundary>
    }
}

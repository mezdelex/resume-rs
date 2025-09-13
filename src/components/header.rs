use crate::components::sidebar::Sidebar;
use leptos::prelude::{component, view, IntoView, RwSignal, Set};
use thaw::Button;

#[component]
pub fn Header() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Button on_click=move |_| { open.set(true) } />
        <Sidebar open />
    }
}

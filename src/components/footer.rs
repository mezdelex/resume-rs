use leptos::prelude::{component, view, IntoView};
use thaw::Flex;

#[component]
pub fn Footer() -> impl IntoView {
    view! { <Flex class="footer">To Vim or not to Vim</Flex> }
}

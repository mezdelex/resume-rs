use leptos::prelude::{component, view, IntoView};
use thaw::{Flex, Label};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <Flex style="margin-bottom: 2rem">
            <Label>To Vim or not to Vim</Label>
        </Flex>
    }
}

use leptos::prelude::{component, signal, view, AddAnyAttr, IntoView};
use thaw::Button;

#[component]
pub fn MyButton(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);

    view! { <Button on:click=move |_| { set_count(count() + increment) }>"Click me: " {count}</Button> }
}

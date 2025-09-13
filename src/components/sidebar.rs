use leptos::prelude::{component, view, IntoView, RwSignal};
use leptos_router::components::A;
use thaw::{DrawerBody, DrawerPosition, Flex, OverlayDrawer};

#[component]
pub fn Sidebar(#[prop(default = RwSignal::new(false))] open: RwSignal<bool>) -> impl IntoView {
    view! {
        <OverlayDrawer open position=DrawerPosition::Right>
            <DrawerBody>
                <Flex vertical=true>
                    <A href="/">Home</A>
                    <A href="/error">Error</A>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}

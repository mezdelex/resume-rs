use icondata::{FaLaptopCodeSolid, MdiChartTimeline};
use leptos::prelude::{component, view, IntoView, RwSignal};
use leptos_icons::Icon;
use leptos_router::components::A;
use thaw::{DrawerBody, DrawerPosition, Flex, FlexAlign, FlexJustify, OverlayDrawer};

#[component]
pub fn Sidebar(#[prop(default = RwSignal::new(false))] open: RwSignal<bool>) -> impl IntoView {
    view! {
        <OverlayDrawer open position=DrawerPosition::Right>
            <DrawerBody>
                <Flex vertical=true>
                    <A href="/">
                        <Flex class="link" align=FlexAlign::Center justify=FlexJustify::Start>
                            <Icon icon=FaLaptopCodeSolid />
                            Projects
                        </Flex>
                    </A>
                    <A href="/timeline">
                        <Flex class="link" align=FlexAlign::Center justify=FlexJustify::Start>
                            <Icon icon=MdiChartTimeline />
                            Timeline
                        </Flex>
                    </A>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}

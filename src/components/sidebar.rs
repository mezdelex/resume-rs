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
                    <Flex class="link" align=FlexAlign::Start justify=FlexJustify::Start>
                        <Icon icon=FaLaptopCodeSolid style="color: orange" />
                        <A href="/">Projects</A>
                    </Flex>
                    <Flex class="link" align=FlexAlign::Start justify=FlexJustify::Start>
                        <Icon icon=MdiChartTimeline style="color: orange" />
                        <A href="/timeline">Timeline</A>
                    </Flex>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}

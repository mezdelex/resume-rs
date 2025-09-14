use crate::contexts::project_context::ProjectContext;
use leptos::prelude::{component, use_context, view, For, IntoView};
use thaw::{
    Button, Card, CardFooter, CardHeader, CardHeaderDescription, CardPreview, Flex, FlexAlign,
    FlexGap, FlexJustify, Text,
};

#[component]
pub fn Projects() -> impl IntoView {
    let projects = use_context::<ProjectContext>()
        .expect("Missing ProjectContext.")
        .projects;

    view! {
        <Flex
            align=FlexAlign::FlexStart
            justify=FlexJustify::SpaceAround
            gap=FlexGap::Large
            style="width: 80vw;flex-wrap: wrap"
        >
            <For
                each=projects
                key=|project| project.id.clone()
                children=move |project| {
                    view! {
                        <Flex style="width: 30rem">
                            <Card class="card">
                                <Flex vertical=true justify=FlexJustify::SpaceAround>
                                    <CardPreview class="card-preview">
                                        <img src=project.image />
                                    </CardPreview>
                                    <CardHeader class="card-header">
                                        <Text style="margin-inline: auto">
                                            {project.name}- {project.pushed_at}
                                        </Text>
                                        <CardHeaderDescription slot>
                                            {project.description}
                                        </CardHeaderDescription>
                                    </CardHeader>
                                    <Flex justify=FlexJustify::SpaceAround>
                                        <CardFooter>
                                            <Button>Repo</Button>
                                            <Button>Link</Button>
                                        </CardFooter>
                                    </Flex>
                                </Flex>
                            </Card>
                        </Flex>
                    }
                }
            />
        </Flex>
    }
}

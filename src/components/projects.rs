use crate::{
    contexts::{project_context::ProjectContext, repository_context::RepositoryContext},
    services::projects_service::ProjectsService,
};
use chrono::{DateTime, Duration, Local, Utc};
use icondata::{BsCloudArrowUpFill, BsGithub, BsLink};
use leptos::prelude::{component, use_context, view, Effect, For, Get, IntoView, Show};
use leptos_icons::Icon;
use leptos_router::components::A;
use thaw::{
    Card, CardFooter, CardHeader, CardPreview, Flex, FlexAlign, FlexGap, FlexJustify, Spinner, Text,
};

pub fn get_last_update(pushed_at: String) -> String {
    let parsed: DateTime<Utc> = match pushed_at.parse() {
        Ok(dt) => dt,
        Err(_) => {
            return if pushed_at.is_empty() {
                "Couldn't fetch date".to_string()
            } else {
                pushed_at
            }
        }
    };

    let duration = Utc::now() - parsed;
    if duration < Duration::days(1) {
        if duration < Duration::hours(1) {
            format!("{} minute(s) ago", duration.num_minutes())
        } else {
            format!("{} hour(s) ago", duration.num_hours())
        }
    } else {
        parsed.with_timezone(&Local).format("%Y-%m-%d").to_string()
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = use_context::<ProjectContext>()
        .expect("Missing ProjectContext.")
        .projects;
    let repository_context =
        use_context::<RepositoryContext>().expect("Missing RepositoryContext.");

    Effect::new(move |_| {
        if repository_context.finished.get() {
            ProjectsService::sort_projects(projects, repository_context.repos);
        }
    });

    view! {
        <Flex
            class="projects-container inner-container"
            align=FlexAlign::FlexStart
            gap=FlexGap::Size(40)
            justify=FlexJustify::SpaceAround
        >
            <Show
                when=move || { repository_context.finished.get() }
                fallback=|| view! { <Spinner /> }
            >
                <For
                    each=projects
                    key=|project| project.id.clone()
                    children=move |project| {
                        view! {
                            <Card>
                                <Flex vertical=true>
                                    <CardHeader>
                                        <Flex
                                            class="last-update"
                                            align=FlexAlign::Center
                                            gap=FlexGap::Size(3)
                                        >
                                            Last update:
                                            <Icon icon=BsCloudArrowUpFill />
                                            {get_last_update(project.pushed_at)}
                                        </Flex>
                                    </CardHeader>
                                    <CardPreview class="card-preview">
                                        <img src=project.image />
                                    </CardPreview>
                                    <Text class="project-name">{project.name}</Text>
                                </Flex>
                                <Flex
                                    class="card-lower"
                                    justify=FlexJustify::SpaceBetween
                                    vertical=true
                                >
                                    <Text class="project-description">{project.description}</Text>
                                    <CardFooter>
                                        <Flex class="card-footer" justify=FlexJustify::SpaceAround>
                                            <A href=project.repo target="_blank">
                                                <Flex
                                                    class="link"
                                                    align=FlexAlign::FlexEnd
                                                    gap=FlexGap::Small
                                                >
                                                    <Icon icon=BsGithub />
                                                    Repo
                                                </Flex>
                                            </A>
                                            <A href=project.app target="_blank">
                                                <Flex
                                                    class="link"
                                                    align=FlexAlign::FlexEnd
                                                    gap=FlexGap::Small
                                                >
                                                    <Icon icon=BsLink />
                                                    Link
                                                </Flex>
                                            </A>
                                        </Flex>
                                    </CardFooter>
                                </Flex>
                            </Card>
                        }
                    }
                />
            </Show>
        </Flex>
    }
}

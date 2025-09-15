use crate::{
    contexts::{project_context::ProjectContext, repository_context::RepositoryContext},
    services::projects_service::ProjectsService,
};
use chrono::{DateTime, Duration, Local, Utc};
use icondata::{AiGithubFilled, AiLinkOutlined, MdiUpdate};
use leptos::prelude::{component, use_context, view, Effect, For, IntoView, With};
use leptos_icons::Icon;
use leptos_router::components::A;
use thaw::{
    Card, CardFooter, CardHeader, CardPreview, Flex, FlexAlign, FlexGap, FlexJustify, Text,
};

pub fn get_last_update(pushed_at: String) -> String {
    let parsed: DateTime<Utc> = match pushed_at.parse() {
        Ok(dt) => dt,
        Err(_) => return "Invalid date".to_string(),
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
        if repository_context.finished.with(|&v| v) {
            ProjectsService::sort_projects(projects, repository_context.repos);
        }
    });

    view! {
        <Flex
            align=FlexAlign::FlexStart
            class="projects-container"
            gap=FlexGap::Size(40)
            justify=FlexJustify::SpaceAround
        >
            <For
                each=projects
                key=|project| project.id.clone()
                children=move |project| {
                    view! {
                        <Card>
                            <Flex
                                class="card-container"
                                justify=FlexJustify::SpaceBetween
                                vertical=true
                            >
                                <Flex vertical=true>
                                    <CardHeader>
                                        <Flex class="last-update" gap=FlexGap::Size(2)>
                                            Last update
                                            <Icon icon=MdiUpdate />
                                            :
                                            {get_last_update(project.pushed_at)}
                                        </Flex>
                                    </CardHeader>
                                    <CardPreview class="card-preview">
                                        <img src=project.image />
                                    </CardPreview>
                                </Flex>
                                <Text class="project-name">{project.name}</Text>
                                <Text class="project-description">{project.description}</Text>
                                <CardFooter>
                                    <Flex class="card-footer" justify=FlexJustify::SpaceAround>
                                        <A href=project.repo target="_">
                                            <Flex class="link hover-effect">
                                                <Icon icon=AiGithubFilled />
                                                Repo
                                            </Flex>
                                        </A>
                                        <A href=project.app target="_">
                                            <Flex class="link hover-effect">
                                                <Icon icon=AiLinkOutlined />
                                                App
                                            </Flex>
                                        </A>
                                    </Flex>
                                </CardFooter>
                            </Flex>
                        </Card>
                    }
                }
            />
        </Flex>
    }
}

use crate::{
    contexts::{project_context::ProjectContext, repository_context::RepositoryContext},
    services::projects_service::ProjectsService,
};
use chrono::{DateTime, Duration, Local, Utc};
use leptos::prelude::{component, use_context, view, Effect, For, IntoView, With};
use leptos_router::components::A;
use thaw::{
    Card, CardFooter, CardHeader, CardHeaderDescription, CardPreview, Flex, FlexAlign, FlexGap,
    FlexJustify, Text,
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
    let repos = use_context::<RepositoryContext>()
        .expect("Missing RepositoryContext.")
        .repos;

    Effect::new(move |_| {
        if repos.with(|r| !r.is_empty()) {
            ProjectsService::sort_projects(projects, repos);
        }
    });

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
                                            {project.name}-{get_last_update(project.pushed_at)}
                                        </Text>
                                        <CardHeaderDescription slot>
                                            {project.description}
                                        </CardHeaderDescription>
                                    </CardHeader>
                                    <CardFooter class="card-footer">
                                        <A href=project.repo target="_">
                                            Repo
                                        </A>
                                        <A href=project.app target="_">
                                            App
                                        </A>
                                    </CardFooter>
                                </Flex>
                            </Card>
                        </Flex>
                    }
                }
            />
        </Flex>
    }
}

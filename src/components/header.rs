use crate::{
    components::sidebar::Sidebar, contexts::repository_context::RepositoryContext,
    services::github_service::GithubService,
};
use icondata::FaBarsSolid;
use leptos::{
    prelude::{
        component, use_context, view, ClassAttribute, Effect, IntoView, RwSignal, Set, With,
    },
    reactive::spawn_local,
};
use leptos_router::components::A;
use thaw::{Button, Flex, FlexAlign, FlexJustify, Text};

#[component]
pub fn Header() -> impl IntoView {
    let open = RwSignal::new(false);
    let repository_context =
        use_context::<RepositoryContext>().expect("Missing RepositoryContext.");

    Effect::new(move |_| {
        if repository_context.link.with(|s| s.is_empty()) {
            spawn_local(async move {
                GithubService::get_repos(repository_context.repos).await;
                GithubService::get_updated_repo(repository_context.repos, repository_context.repo)
                    .await;
                GithubService::get_last_commit(
                    repository_context.repo,
                    repository_context.date,
                    repository_context.message,
                    repository_context.sha,
                    repository_context.link,
                )
                .await;
                repository_context.finished.set(true);
            });
        }
    });

    view! {
        <Flex class="inner-container" justify=FlexJustify::FlexEnd>
            <Button icon=FaBarsSolid on_click=move |_| { open.set(true) } />
        </Flex>
        <Flex vertical=true align=FlexAlign::Center gap=thaw::FlexGap::Medium>
            <img class="selfie" src="./images/alejandro.png" />
            <Flex>
                <Text class="header-text">
                    Last activity: <Text>{repository_context.date}</Text>
                </Text>
                <Text class="header-text">
                    Commit: <A href=repository_context.link target="_">
                        Link
                    </A>
                </Text>
            </Flex>
            <Text class="commit-message header-text">
                Message: <Text>{repository_context.message}</Text>
            </Text>
            <Sidebar open />
        </Flex>
    }
}

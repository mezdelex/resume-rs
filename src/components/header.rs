use crate::{
    components::sidebar::Sidebar, contexts::repository_context::RepositoryContext,
    services::github_service,
};
use leptos::{
    prelude::{
        component, use_context, view, ClassAttribute, Effect, IntoView, RwSignal,
        Set, With,
    },
    reactive::spawn_local,
};
use leptos_router::components::A;
use thaw::{Button, Flex, FlexAlign, Text};

#[component]
pub fn Header() -> impl IntoView {
    let open = RwSignal::new(false);
    let repository_context =
        use_context::<RepositoryContext>().expect("Missing RepositoryContext.");

    Effect::new(move |_| {
        if repository_context.link.with(|s| s.is_empty()) {
            spawn_local(async move {
                github_service::GithubService::get_repos(repository_context.repos).await;
                github_service::GithubService::get_updated_repo(
                    repository_context.repos,
                    repository_context.repo,
                )
                .await;
                github_service::GithubService::get_last_commit(
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
        <Flex
            vertical=true
            align=FlexAlign::Center
            gap=thaw::FlexGap::Medium
            style="margin-top: 2rem"
        >
            <img class="selfie" src="./images/alejandro.png" />
            <Flex>
                <Text>Last activity: {repository_context.date}</Text>
                <Text>
                    Commit: <A href=repository_context.link target="_">
                        Link
                    </A>
                </Text>
            </Flex>
            <Text style="max-width: 20rem;text-align: center">
                Message: {repository_context.message}
            </Text>
            <Button on_click=move |_| { open.set(true) } />
            <Sidebar open />
        </Flex>
    }
}

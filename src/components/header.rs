use crate::{
    contexts::repository_context::RepositoryContext, services::github_service::GithubService,
};
use icondata::{AiGithubFilled, AiLinkedinFilled, AiRedditCircleFilled};
use leptos::{
    prelude::{component, use_context, view, ClassAttribute, Effect, Get, IntoView, Set, Show},
    reactive::spawn_local,
};
use leptos_icons::Icon;
use leptos_router::components::A;
use thaw::{Flex, FlexAlign, FlexGap, Skeleton, SkeletonItem, Text};

#[component]
pub fn Header() -> impl IntoView {
    let repository_context =
        use_context::<RepositoryContext>().expect("Missing RepositoryContext.");

    Effect::new(move |_| {
        if repository_context.link.get().is_empty() {
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
        <Flex class="header" align=FlexAlign::Center gap=thaw::FlexGap::Medium vertical=true>
            <img class="selfie" src="./images/alejandro.png" />
            <Show
                when=move || { repository_context.finished.get() }
                fallback=|| {
                    view! {
                        <Skeleton class="full-width">
                            <Flex gap=FlexGap::Small vertical=true>
                                <SkeletonItem />
                                <SkeletonItem />
                                <SkeletonItem />
                            </Flex>
                        </Skeleton>
                    }
                }
            >
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
                <Flex class="social" gap=FlexGap::Small>
                    <A href="https://linkedin.com/in/mezdelex" target="_">
                        <Icon icon=AiLinkedinFilled />
                    </A>
                    <A href="https://github.com/mezdelex" target="_">
                        <Icon icon=AiGithubFilled />
                    </A>
                    <A href="https://reddit.com/u/mezdelex" target="_">
                        <Icon icon=AiRedditCircleFilled />
                    </A>
                </Flex>
            </Show>
        </Flex>
    }
}

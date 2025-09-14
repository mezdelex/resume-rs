use crate::models::{commit::Commit, repository::Repository};
use leptos::prelude::{GetUntracked, RwSignal, Set};
use web_sys::console::error_1;

pub struct GithubService;

impl GithubService {
    pub async fn get_repos(repos: RwSignal<Vec<Repository>>) {
        match reqwest::get("https://api.github.com/users/mezdelex/repos?per_page=100").await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Repository>>().await {
                        Ok(data) => {
                            repos.set(data);
                        }
                        Err(e) => {
                            error_1(&format!("Failed to parse repositories: {:?}", e).into());
                        }
                    }
                } else {
                    error_1(
                        &format!("Failed to fetch repositories: Status {}", response.status())
                            .into(),
                    );
                }
            }
            Err(e) => {
                error_1(&format!("Failed to send request: {:?}", e).into());
            }
        }
    }
    pub async fn get_updated_repo(repos: RwSignal<Vec<Repository>>, repo: RwSignal<String>) {
        let current_repos = repos.get_untracked();
        if !current_repos.is_empty() {
            if let Some(latest_repo) = current_repos.iter().max_by_key(|r| &r.pushed_at) {
                repo.set(latest_repo.name.clone());
            }
        }
    }
    pub async fn get_last_commit(
        repo: RwSignal<String>,
        date: RwSignal<String>,
        message: RwSignal<String>,
        sha: RwSignal<String>,
        link: RwSignal<String>,
    ) {
        let updated_repo = repo.get_untracked();
        if updated_repo.is_empty() {
            error_1(&format!("Failed to get last commit: repository name is empty.").into());
            return;
        }

        match reqwest::get(format!(
            "https://api.github.com/repos/mezdelex/{}/commits",
            updated_repo
        ))
        .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Commit>>().await {
                        Ok(data) => {
                            if let Some(first_commit) = data.first() {
                                date.set(first_commit.commit.author.date[0..10].to_string());
                                message.set(first_commit.commit.message.clone());
                                sha.set(first_commit.sha.clone());
                                link.set(format!(
                                    "https://github.com/mezdelex/{}/commit/{}",
                                    updated_repo, first_commit.sha
                                ));
                            } else {
                                error_1(
                                    &format!(
                                        "Failed to get last commit: No commits found for {}",
                                        updated_repo
                                    )
                                    .into(),
                                );
                            }
                        }
                        Err(e) => {
                            error_1(&format!("Failed to parse commit data: {:?}", e).into());
                        }
                    }
                } else {
                    error_1(
                        &format!(
                            "Failed to fetch last commit: Status {} for {}",
                            response.status(),
                            updated_repo
                        )
                        .into(),
                    );
                }
            }
            Err(e) => {
                error_1(
                    &format!(
                        "Failed to send request for last commit ({}): {:?}",
                        updated_repo, e
                    )
                    .into(),
                );
            }
        }
    }
}

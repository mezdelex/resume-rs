use crate::models::repository::Repository;
use leptos::prelude::RwSignal;

#[derive(Clone)]
pub struct RepositoryContext {
    pub repos: RwSignal<Vec<Repository>>,
    pub date: RwSignal<String>,
    pub message: RwSignal<String>,
    pub repo: RwSignal<String>,
    pub sha: RwSignal<String>,
    pub link: RwSignal<String>,
    pub finished: RwSignal<bool>,
}

impl Default for RepositoryContext {
    fn default() -> Self {
        Self {
            repos: RwSignal::default(),
            date: RwSignal::default(),
            message: RwSignal::default(),
            repo: RwSignal::default(),
            sha: RwSignal::default(),
            link: RwSignal::default(),
            finished: RwSignal::default(),
        }
    }
}

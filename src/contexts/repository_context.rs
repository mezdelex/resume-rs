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
            repos: RwSignal::new(Vec::<Repository>::new()),
            date: RwSignal::new(String::new()),
            message: RwSignal::new(String::new()),
            repo: RwSignal::new(String::new()),
            sha: RwSignal::new(String::new()),
            link: RwSignal::new(String::new()),
            finished: RwSignal::new(false),
        }
    }
}

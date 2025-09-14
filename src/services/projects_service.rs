use crate::{
    data,
    models::{project::Project, repository::Repository},
};
use leptos::prelude::{GetUntracked, RwSignal, Set};

pub struct ProjectsService;

impl ProjectsService {
    pub fn sort_projects(projects: RwSignal<Vec<Project>>, repos: RwSignal<Vec<Repository>>) {
        let current_repos = repos.get_untracked();
        let projects_data = data::projects_data::PROJECTS.to_vec();

        if !current_repos.is_empty() {
            let updated_projects: Vec<Project> = projects_data
                .into_iter()
                .map(|mut p| {
                    if !p.id.is_empty() {
                        if let Some(repo) = current_repos.iter().find(|r| r.name == p.id) {
                            p.pushed_at = repo.pushed_at.clone();
                        }
                    }
                    p
                })
                .collect();

            let mut sorted_projects = updated_projects;
            sorted_projects.sort_by(|a, b| b.pushed_at.cmp(&a.pushed_at));

            projects.set(sorted_projects);
        } else {
            projects.set(projects_data);
        }
    }
}

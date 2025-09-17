use crate::{
    data::projects_data::PROJECTS,
    models::{project::Project, repository::Repository},
};
use leptos::prelude::{GetUntracked, RwSignal, Set};

pub struct ProjectsService;

impl ProjectsService {
    pub fn sort_projects(projects: RwSignal<Vec<Project>>, repos: RwSignal<Vec<Repository>>) {
        let current_repos = repos.get_untracked();
        let projects_data = PROJECTS.to_vec();

        if !current_repos.is_empty() {
            let mut updated_projects: Vec<Project> = projects_data
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
            updated_projects.sort_by(|a, b| b.pushed_at.cmp(&a.pushed_at));

            projects.set(updated_projects);
        } else {
            projects.set(projects_data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{project::Project, repository::Repository};
    use chrono::{Duration, Utc};

    fn mock_repository(name: String, pushed_at: String) -> Repository {
        Repository { name, pushed_at }
    }

    #[test]
    fn test_sort_projects_with_non_empty_repos() {
        let repos_signal = RwSignal::new(vec![
            mock_repository(
                "resume-rs".to_owned(),
                (Utc::now() - Duration::days(5)).to_rfc3339(),
            ),
            mock_repository(
                "unpack".to_owned(),
                (Utc::now() - Duration::days(1)).to_rfc3339(),
            ),
        ]);

        let projects_signal: RwSignal<Vec<Project>> = RwSignal::new(vec![]);
        ProjectsService::sort_projects(projects_signal, repos_signal);
        let sorted_projects = projects_signal.get_untracked();

        assert!(
            !sorted_projects.is_empty(),
            "sorted_projects shouldn't be empty."
        );

        let unpack_index = sorted_projects.iter().position(|p| p.id == "unpack");
        let resume_rs_index = sorted_projects.iter().position(|p| p.id == "resume-rs");

        if let (Some(u_idx), Some(r_idx)) = (unpack_index, resume_rs_index) {
            assert!(u_idx < r_idx, "'unpack' should go before 'resume-rs'");
        } else {
            panic!("Couldn't find 'unpack' or 'resume-rs' among projects data.");
        }
    }

    #[test]
    fn test_sort_projects_with_empty_repos() {
        let repos_signal = RwSignal::new(vec![]);
        let projects_signal: RwSignal<Vec<Project>> = RwSignal::new(vec![]);

        ProjectsService::sort_projects(projects_signal, repos_signal);
        let default_projects = projects_signal.get_untracked();

        assert!(
            !default_projects.is_empty(),
            "default_projects should have default data."
        );
    }
}

use crate::models::project::Project;
use leptos::prelude::RwSignal;

#[derive(Clone)]
pub struct ProjectContext {
    pub projects: RwSignal<Vec<Project>>,
}

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Repository {
    pub name: String,
    pub pushed_at: String,
}

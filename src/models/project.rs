#[derive(Clone, PartialEq)]
pub struct Project {
    pub id: String,
    pub pushed_at: String,
    pub name: String,
    pub image: String,
    pub repo: String,
    pub app: String,
    pub description: String,
}

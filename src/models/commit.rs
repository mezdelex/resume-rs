use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct CommitDetails {
    pub author: CommitAuthor,
    pub message: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct CommitAuthor {
    pub date: String,
    pub name: String,
}

use serde::Deserialize;

use super::common::GitHubTimeStamp;

#[derive(Deserialize)]
pub struct Author {
    pub date: GitHubTimeStamp,
}

#[derive(Deserialize)]
pub struct Commit {
    pub message: String,
    pub author: Author,
}

#[derive(Deserialize)]
pub struct CommitRecord {
    pub commit: Commit,
}

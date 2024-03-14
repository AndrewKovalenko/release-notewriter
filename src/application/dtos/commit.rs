use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct Author {
    pub date: DateTime<Utc>,
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

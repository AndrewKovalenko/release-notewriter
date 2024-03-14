use serde::Deserialize;

pub type GitHubTimeStamp = String;

#[derive(Deserialize)]
pub struct Release {
    pub published_at: GitHubTimeStamp,
}

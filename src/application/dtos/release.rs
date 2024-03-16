use serde::Deserialize;

use super::common::GitHubTimeStamp;

#[derive(Deserialize)]
pub struct Release {
    pub published_at: GitHubTimeStamp,
}

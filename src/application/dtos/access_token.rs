use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepositoryAccessToken {
    pub token: String,
}

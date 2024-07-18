use serde::Deserialize;

#[derive(Deserialize)]
pub struct Installation {
    pub access_token_url: String,
}

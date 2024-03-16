use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repository {
    pub description: String,
}

use crate::application::dtos::llm_request::GptMessage;
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub enum ObjectType {
    ChatCompletion,
}
impl<'de> Deserialize<'de> for ObjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = s.as_str();
        match value {
            "chat.completion" => Ok(ObjectType::ChatCompletion),
            _ => Err(anyhow::anyhow!("unknown object type {}", value)).map_err(de::Error::custom),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub index: i64,
    pub message: GptMessage,
}

#[derive(Serialize, Deserialize)]
pub struct GptResponse {
    pub id: String,
    pub object: ObjectType,
    pub created: i64,
    pub choices: Vec<Choice>,
}

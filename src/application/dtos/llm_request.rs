const GPT_3_5_TURBO_NAME: &str = "gpt-3.5-turbo";
const SYSTEM_ROLE_NAME: &str = "system";
const ASSISTANT_ROLE_NAME: &str = "assistant";
const USER_ROLE_NAME: &str = "user";

pub enum ModelVersion {
    Gpt3_5_Turbo,
}

impl fmt::Display for ModelVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelVersion::Gpt3_5_Turbo => write!(f, GPT_3_5_TURBO_NAME),
        }
    }
}

pub enum GptRole {
    System,
    Assistant,
    User,
}

impl fmt::Display for GptRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GptRole::System => write!(f, SYSTEM_ROLE_NAME),
            GptRole::Assistant => write!(f, ASSISTANT_ROLE_NAME),
            GptRole::User => write!(f, USER_ROLE_NAME),
        }
    }
}

pub struct GptMessage {
    pub role: GptRole,
    pub content: String,
}

pub struct LLM_Request {
    pub model: ModelVersion,
    pub messages: Vec<GptMessage>,
}

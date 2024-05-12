use super::system::get_env_value;
use crate::application::dtos::llm_request::LlmRequest;

const GPT_COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn get_release_notes(model_request: LlmRequest) -> String {
    let gpt_access_token = get_env_value("GPT_ACCESS_TOKEN").unwrap();
    let request_body = serde_json::to_string(&model_request).unwrap();

    let http_client = reqwest::Client::new();
    let gpt_response = http_client
        .post(GPT_COMPLETIONS_URL)
        .header("Authorization", format!("Bearer {gpt_access_token}"))
        .header(reqwest::header::ACCEPT, "*/*")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(request_body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    gpt_response
}

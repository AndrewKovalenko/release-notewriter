use super::dtos::llm_request::{GptMessage, GptRole, LLM_Request, ModelVersion};
use crate::repositories::github::Repository;

pub fn build_llm_request(repository_description: String, commits: Vec<String>) -> LLM_Request {
    let system_prompt_text = format!(
        include_str!("../../prompt_templates/system_prompt.tmpl"),
        repository_description = repository_description
    );

    let messages = vec![GptMessage {
        role: GptRole::System,
        content: system_prompt_text,
    }];

    LLM_Request {
        model: ModelVersion::Gpt3_5_Turbo,
        messages,
    }
}

pub async fn generate_notes_since_latest_release(repository_url: &str) -> Vec<String> {
    let repository = Repository::new(repository_url);
    let latest_release = repository.latest_release().await;
    let last_release_timestamp = if let Some(last_release) = latest_release {
        Some(last_release.published_at)
    } else {
        None
    };

    let mut commits = repository
        .commits(last_release_timestamp)
        .await
        .iter()
        .map(|record| record.commit.message.clone())
        .collect::<Vec<String>>();

    let repository_information = repository.description().await;

    commits.push(repository_information.description);
    commits
}

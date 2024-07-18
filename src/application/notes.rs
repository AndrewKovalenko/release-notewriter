use super::dtos::llm_request::{GptMessage, GptRole, LlmRequest, ModelVersion};
use crate::repositories::{github::Repository, gpt};

pub fn build_llm_request(repository_description: String, commits: Vec<String>) -> LlmRequest {
    let system_prompt_text = format!(
        include_str!("../../prompt_templates/system_prompt.tmpl"),
        repository_description = repository_description
    );

    let commit_messages = commits.iter().fold(String::new(), |result, commit| {
        format!("{result} {commit}").to_string()
    });

    let changes_prompt_text = format!(
        include_str!("../../prompt_templates/changes_prompt.tmpl"),
        changes = commit_messages
    );

    let messages = vec![
        GptMessage {
            role: GptRole::System,
            content: system_prompt_text,
        },
        GptMessage {
            role: GptRole::User,
            content: changes_prompt_text,
        },
        GptMessage {
            role: GptRole::User,
            content: String::from(include_str!("../../prompt_templates/ask_prompt.tmpl")),
        },
    ];

    LlmRequest {
        model: ModelVersion::Gpt3_5Turbo,
        messages,
    }
}

pub async fn generate_notes_since_latest_release(account: String, repo_name: String) -> String {
    let repository = Repository::new(account, repo_name).await;
    let latest_release = repository.latest_release().await;
    let last_release_timestamp = if let Some(last_release) = latest_release {
        Some(last_release.published_at)
    } else {
        None
    };

    let commits = repository
        .commits(last_release_timestamp)
        .await
        .iter()
        .map(|record| record.commit.message.clone())
        .collect::<Vec<String>>();

    let repository_information = repository.description().await;

    let llm_request = build_llm_request(repository_information.description, commits);
    gpt::get_release_notes(llm_request).await
}

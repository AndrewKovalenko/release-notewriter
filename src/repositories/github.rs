use anyhow::Result;

use crate::application::dtos::common::GitHubTimeStamp;
use crate::application::dtos::{
    access_token::RepositoryAccessToken, commit::CommitRecord, installation::Installation,
    release::Release, repository::Repository as GitHubRepositoryInformation,
};
use crate::config::{GITHUB_APP_ID, GITHUB_JWT_TTL, GITHUB_PRIVATE_KEY_FILE};
use crate::services::jwt::generate_github_access_jwt;
use std::env;

// const APP_AUTH_TOKEN: &str = "ghs_2bfqNh2rReCtw5MVfXffTREXTkc7Ka2u8Odn";

async fn get_app_access_token(
    repo_owner: &str,
    repo_name: &str,
    jwt_token: &str,
) -> Result<String> {
    let get_installations_url = format!(
        "https://api.github.com/repos/{}/{}/installation",
        repo_owner, repo_name
    );
    let http_client = reqwest::Client::new();

    let installation_response = http_client
        .get(get_installations_url)
        .header("Authorization:", format!("Bearer {jwt_token}"))
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let installation = serde_json::from_str::<Installation>(installation_response.as_str())?;

    let repository_access_token_response = http_client
        .post(installation.access_token_url)
        .header("Authorization:", format!("Bearer {jwt_token}"))
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let access_token =
        serde_json::from_str::<RepositoryAccessToken>(repository_access_token_response.as_str())?;

    Ok(access_token.token)
}

pub struct Repository {
    owner: String,
    repo_name: String,
    access_token: String,
}

impl Repository {
    pub async fn new(owner: String, repo_name: String) -> Self {
        let app_id = env::var(GITHUB_APP_ID).unwrap();
        let private_secret_path = env::var(GITHUB_PRIVATE_KEY_FILE).unwrap();

        let jwt_token =
            generate_github_access_jwt(app_id, private_secret_path, GITHUB_JWT_TTL).unwrap();
        let access_token = get_app_access_token(&owner, &repo_name, &jwt_token)
            .await
            .unwrap();

        Repository {
            owner,
            repo_name,
            access_token,
        }
    }

    pub async fn description(&self) -> GitHubRepositoryInformation {
        let get_repo_url = format!(
            "https://api.github.com/repos/{}/{}",
            self.owner, self.repo_name
        );

        let http_client = reqwest::Client::new();
        let repo_information_response = http_client
            .get(get_repo_url)
            .header("Authorization", self.access_token.as_str())
            .header(reqwest::header::USER_AGENT, "Release-Noter")
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        serde_json::from_str(&repo_information_response).unwrap()
    }

    pub async fn commits(&self, since: Option<GitHubTimeStamp>) -> Vec<CommitRecord> {
        let get_commits_url = if let Some(timestamp) = since {
            format!(
                "https://api.github.com/repos/{}/{}/commits?since={}",
                self.owner, self.repo_name, timestamp
            )
        } else {
            format!(
                "https://api.github.com/repos/{}/{}/commits",
                self.owner, self.repo_name
            )
        };

        let http_client = reqwest::Client::new();
        let commits_response = http_client
            .get(get_commits_url)
            .header("Authorization", self.access_token.as_str())
            .header(reqwest::header::USER_AGENT, "Release-Noter")
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        serde_json::from_str::<Vec<CommitRecord>>(&commits_response).unwrap()
    }

    pub async fn latest_release(&self) -> Option<Release> {
        let latest_release_api_call = format!(
            " https://api.github.com/repos/{}/{}/releases/latest",
            self.owner, self.repo_name
        );

        let http_client = reqwest::Client::new();
        let latest_release_response = http_client
            .get(latest_release_api_call)
            .header("Authorization", self.access_token.as_str())
            .header(reqwest::header::USER_AGENT, "Release-Noter")
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        if let Ok(latest_release) =
            serde_json::from_str::<Release>(latest_release_response.as_str())
        {
            return Some(latest_release);
        }

        None
    }
}

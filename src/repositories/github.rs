use crate::application::dtos::common::GitHubTimeStamp;
use crate::application::dtos::{
    commit::CommitRecord, release::Release, repository::Repository as GitHubRepositoryInformation,
};

const APP_AUTH_TOKEN: &str = "ghs_2bfqNh2rReCtw5MVfXffTREXTkc7Ka2u8Odn";

fn parse_url(url: &str) -> (String, String) {
    (String::from("AndrewKovalenko"), String::from("lotogen"))
}

pub struct Repository {
    owner: String,
    repo_name: String,
}

impl Repository {
    pub fn new(url: &str) -> Self {
        let (owner, repo_name) = parse_url(url);
        Repository { owner, repo_name }
    }

    pub async fn description(&self) -> GitHubRepositoryInformation {
        let get_repo_url = format!(
            "https://api.github.com/repos/{}/{}",
            self.owner, self.repo_name
        );

        let http_client = reqwest::Client::new();
        let repo_information_response = http_client
            .get(get_repo_url)
            .header("Authorization", APP_AUTH_TOKEN)
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
            .header("Authorization", APP_AUTH_TOKEN)
            .header(reqwest::header::USER_AGENT, "Release-Noter")
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let commits = serde_json::from_str::<Vec<CommitRecord>>(&commits_response).unwrap();
        return commits;
    }

    pub async fn latest_release(&self) -> Option<Release> {
        let latest_release_api_call = format!(
            " https://api.github.com/repos/{}/{}/releases/latest",
            self.owner, self.repo_name
        );

        let http_client = reqwest::Client::new();
        let latest_release_response = http_client
            .get(latest_release_api_call)
            .header("Authorization", APP_AUTH_TOKEN)
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

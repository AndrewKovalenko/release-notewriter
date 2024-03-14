use crate::{
    application::dtos::release::{GitHubTimeStamp, Release},
    config,
    services::jwt,
};
use chrono::{self, DateTime, Utc};

use super::system;

const APP_AUTH_TOKEN: &str = "ghs_2bfqNh2rReCtw5MVfXffTREXTkc7Ka2u8Odn";

fn parse_url(url: &str) -> (String, String) {
    (String::from("AndrewKovalenko"), String::from("lotogen"))
}

pub struct Repository<'a> {
    url: &'a str,
}

impl<'a> Repository<'a> {
    pub fn new(url: &'a str) -> Self {
        Repository { url }
    }

    pub async fn get_commits(&self, since: Option<GitHubTimeStamp>) -> String {
        let (owner, repo) = parse_url(self.url);
        let get_commits_url = if let Some(timestamp) = since {
            format!(
                "https://api.github.com/repos/{}/{}/commits?since={}",
                owner, repo, timestamp
            )
        } else {
            format!("https://api.github.com/repos/{}/{}/commits", owner, repo)
        };

        let http_client = reqwest::Client::new();
        let commits = http_client
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

        return commits;
    }

    pub async fn get_latest_release(&self) -> Option<Release> {
        let (owner, repo) = parse_url(self.url);
        let latest_release_api_call =
            format!(" https://api.github.com/repos/{owner}/{repo}/releases/latest");

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

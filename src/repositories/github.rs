use crate::application::dtos::release::Release;
use chrono::{self, DateTime, Utc};

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

    pub fn get_latest_release(&self) -> Release {
        return Release {
            timestamp: chrono::offset::Utc::now(),
        };
    }

    pub async fn get_commit(&self, since: DateTime<Utc>, until: DateTime<Utc>) -> String {
        let (owner, repo) = parse_url(self.url);
        let get_commits_url = format!("https://api.github.com/repos/{}/{}/commits", owner, repo);
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
}

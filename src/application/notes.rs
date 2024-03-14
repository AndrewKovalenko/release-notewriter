use crate::repositories::github::Repository;

pub async fn generate_notes_since_latest_release(repository_url: &str) -> Vec<String> {
    let repository = Repository::new(repository_url);
    let latest_release = repository.get_latest_release().await;
    let last_release_timestamp = if let Some(last_release) = latest_release {
        Some(last_release.published_at)
    } else {
        None
    };

    let commits = repository.get_commits(last_release_timestamp).await;
    return vec![commits];
}

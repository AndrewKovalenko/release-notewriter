use crate::data_access::github::Repository;

pub async fn generate_notes_since_latest_release(repository_url: &str) -> Vec<String> {
    let repository = Repository::new(repository_url);
    // let latest_release = repository.get_latest_release();
    let commits = repository
        .get_commit(chrono::offset::Utc::now(), chrono::offset::Utc::now())
        .await;
    return vec![commits];
}

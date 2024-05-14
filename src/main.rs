use release_notewriter::application::notes::generate_notes_since_latest_release;
use release_notewriter::config::TEST_REPO;
use release_notewriter::repositories::system;

#[tokio::main]
async fn main() {
    system::load_env_file();
    let release_notes = generate_notes_since_latest_release(TEST_REPO).await;

    println!("{release_notes}")
}

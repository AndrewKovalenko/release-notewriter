use release_notewriter::application::notes::generate_notes_since_latest_release;

const TEST_REPO: &str = "https://github.com/AndrewKovalenko/lotogen";

#[tokio::main]
async fn main() {
    let release_notes = generate_notes_since_latest_release(TEST_REPO).await;

    release_notes.iter().for_each(|line| println!("{line}"));
}

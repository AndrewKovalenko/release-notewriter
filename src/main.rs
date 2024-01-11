use release_notewriter::cmd::notes::generate_notes_since_latest_release;

const TEST_REPO: &str = "https://github.com/AndrewKovalenko/lotogen";

fn main() {
    let release_notes = generate_notes_since_latest_release(TEST_REPO);

    release_notes.iter().for_each(|line| println!("{line}"));
}

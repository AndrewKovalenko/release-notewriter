use release_notewriter::application::notes::generate_notes_since_latest_release;
use release_notewriter::config;
use release_notewriter::repositories::system;

use warp;
use std::env;

#[tokio::main]
async fn main() {
    system::load_env_file();
    let args = env::args().collect();
    let server_address = if let Some(address) = parse_server_adderss(args[0]) {
    address
} else {
    config::SERVER_ADDRESS
};

    let release_notes = generate_notes_since_latest_release(config::TEST_REPO).await;

    println!("{release_notes}")



    let get_repo_release_notes = warp::path!();

    warp::serve(get_repo_release_notes)
.run(server_address, server_port)
    .await
}

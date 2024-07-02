use release_notewriter::application::notes::generate_notes_since_latest_release;
use release_notewriter::config;
use release_notewriter::repositories::system;

use axum::{extract::Path, response::Json, routing::get, Router};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct ReleaseNotesResponse {
    pub release_notes: String,
}

async fn release_notes(
    Path((account, repo_name)): Path<(String, String)>,
) -> Json<ReleaseNotesResponse> {
    let release_notes = generate_notes_since_latest_release(account, repo_name).await;

    let response = ReleaseNotesResponse { release_notes };
    Json(response)
}

#[tokio::main]
async fn main() {
    system::load_env_file();
    let args = env::args().collect::<Vec<String>>();
    let (server_address, server_port) = if args.len() == 2 {
        let server_address = args[0].to_string();
        let server_port = args[1]
            .clone()
            .parse::<u16>()
            .unwrap_or(config::SERVER_PORT);
        (server_address, server_port)
    } else {
        (config::SERVER_ADDRESS.to_string(), config::SERVER_PORT)
    };

    let app = Router::new().route("/releasenotes/:account/:repo_name", get(release_notes));
    let listener = tokio::net::TcpListener::bind(format!("{server_address}:{server_port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

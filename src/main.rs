use release_notewriter::application::notes::generate_notes_since_latest_release;
use release_notewriter::config;
use release_notewriter::repositories::system;
use release_notewriter::utils::argument_parser;

use std::convert::Infallible;
use std::{collections::HashMap, env, u16};
use warp::{self, Filter};

async fn get_release_notes_handler(
    _: HashMap<String, String>,
) -> Result<impl warp::Reply, Infallible> {
    let release_notes = generate_notes_since_latest_release(config::TEST_REPO).await;
    //let repo_name = query_params.get("repo_name").unwrap();

    Ok(warp::reply::json(&release_notes))
}

#[tokio::main]
async fn main() {
    system::load_env_file();
    let args = env::args().collect::<Vec<String>>();
    let (server_address, server_port) = if args.len() == 2 {
        let server_address = argument_parser::parse_server_adderss(args[0].clone())
            .unwrap_or(config::SERVER_ADDRESS);
        let server_port = args[1]
            .clone()
            .parse::<u16>()
            .unwrap_or(config::SERVER_PORT);
        (server_address, server_port)
    } else {
        (config::SERVER_ADDRESS, config::SERVER_PORT)
    };

    let get_repo_release_notes = warp::get()
        .and(warp::path("releasenotes"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_release_notes_handler);

    warp::serve(get_repo_release_notes)
        .run((server_address, server_port))
        .await
}

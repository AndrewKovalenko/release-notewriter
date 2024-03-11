use std::collections::HashMap;

use chrono::Duration;
use jsonwebtoken;
use serde::Serialize;

const ONE_MINUTE: i64 = 60;

#[derive(Serialize, Debug)]
struct GitHubClaims {
    iat: i64,
    exp: i64,
    iss: String,
}

pub fn generate_github_access_jwt(
    app_id: &str,
    private_key_file_path: &str,
    ttl_sec: i64,
) -> String {
    let jwt_headers = HashMap::from([("typ", "JWT"), ("alg", "RS256")]);

    let now = chrono::offset::Local::now();
    let one_minute_ago = now - Duration::seconds(ONE_MINUTE);
    let jwt_expiration_time = now + Duration::seconds(ttl_sec);

    let claims = GitHubClaims {
        iat: one_minute_ago.timestamp(),
        exp: jwt_expiration_time.timestamp(),
        iss: String::from(app_id),
    };
}

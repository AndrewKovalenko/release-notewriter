use std::fs;

use chrono::Duration;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
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
    let jwt_header = Header::new(Algorithm::RS256);

    let now = chrono::offset::Local::now();
    let one_minute_ago = now - Duration::seconds(ONE_MINUTE);
    let jwt_expiration_time = now + Duration::seconds(ttl_sec);

    let claims = GitHubClaims {
        iat: one_minute_ago.timestamp(),
        exp: jwt_expiration_time.timestamp(),
        iss: String::from(app_id),
    };

    let private_key = fs::read_to_string(private_key_file_path).unwrap();

    let jwt_token = encode(
        &jwt_header,
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_ref()).unwrap(),
    );

    jwt_token.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;
    use std::env;

    #[test]
    fn sandbox() {
        let app_id = env::var(GITHUB_APP_ID);
        let private_secret_path = env::var(GITHUB_PRIVATE_KEY_FILE);

        let jwt_token = generate_github_access_jwt(GITHUB_APP_ID, GITHUB_PRIVATE_KEY_FILE, 600);
        println!("JWT: {}", jwt_token);
    }
}

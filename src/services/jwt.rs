use anyhow::{bail, Context, Result};
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
    app_id: String,
    private_key_file_path: String,
    ttl_sec: i64,
) -> Result<String> {
    let jwt_header = Header::new(Algorithm::RS256);

    let now = chrono::offset::Local::now();
    let one_minute_ago = now - Duration::seconds(ONE_MINUTE);
    let jwt_expiration_time = now + Duration::seconds(ttl_sec);

    let claims = GitHubClaims {
        iat: one_minute_ago.timestamp(),
        exp: jwt_expiration_time.timestamp(),
        iss: app_id,
    };

    let private_key = read_private_secret(private_key_file_path.as_str())?;

    let jwt_token = encode(
        &jwt_header,
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_ref()).unwrap(),
    )
    .context("Unable to generate a JWT token based on claims and headers provided");

    jwt_token
}

fn read_private_secret(file_path: &str) -> Result<String> {
    let secret_reading_result = fs::read_to_string(file_path);

    if secret_reading_result.is_err() {
        bail!("Unable to read the file at {file_path}")
    }

    Ok(secret_reading_result.unwrap())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::*;
    use std::env;

    #[test]
    fn sandbox() {
        let app_id = env::var(GITHUB_APP_ID).unwrap();
        let private_secret_path = env::var(GITHUB_PRIVATE_KEY_FILE).unwrap();

        let jwt_token = generate_github_access_jwt(app_id, private_secret_path, 600);
        assert!(jwt_token.is_ok());
    }
}

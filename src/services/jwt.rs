use std::fs;

use anyhow::bail;
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
fn read_private_key(file_path: &str) -> anyhow::Result<String> {
    fs::read_to_string(file_path).or(bail!("unable to read secret form file: {file_path}"))
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

    let private_key = read_private_key(private_key_file_path).unwrap();

    let jwt_token = encode(
        &jwt_header,
        &claims,
        &EncodingKey::from_secret(private_key.as_ref()),
    );

    jwt_token.unwrap()
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{Algorithm, Header};

    #[test]
    fn sandbox() {
        let jwt_header = Header::new(Algorithm::RS256);

        println!("Type: {}", jwt_header.typ.unwrap());
        println!("Claims: {}", cla);
    }
}

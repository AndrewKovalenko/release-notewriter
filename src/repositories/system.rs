use anyhow;
use dotenv::dotenv;
use std::env;

pub fn load_env_file() {
    dotenv().ok();
}

pub fn get_env_value(variable_name: &str) -> anyhow::Result<String> {
    match env::var(variable_name) {
        Ok(value) => Ok(value),
        Err(_) => anyhow::bail!("Unable to read system variable {}", variable_name),
    }
}

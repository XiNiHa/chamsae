use std::path::PathBuf;

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::try_from_env().expect("failed to parse config from environment variables")
});

fn default_listen_addr() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_database_host() -> String {
    "localhost".to_string()
}

fn default_database_port() -> u16 {
    5432
}

fn default_database_user() -> String {
    "postgres".to_string()
}

fn default_database_password() -> String {
    "chamsae".to_string()
}

fn default_database_database() -> String {
    "postgres".to_string()
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub domain: String,

    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    #[serde(default = "default_database_host")]
    pub database_host: String,
    #[serde(default = "default_database_port")]
    pub database_port: u16,
    #[serde(default = "default_database_user")]
    pub database_user: String,
    #[serde(default = "default_database_password")]
    pub database_password: String,
    #[serde(default = "default_database_database")]
    pub database_database: String,

    /// Handle of the owner of this instance
    pub user_handle: String,
    /// Password bcrypt hash of the owner user of this instance
    pub user_password_bcrypt: String,

    #[serde(skip)]
    pub user_id: Option<Url>,
    #[serde(skip)]
    pub inbox_url: Option<Url>,

    /// Public key PEM file path for the owner user of this instance
    pub user_public_key_path: PathBuf,
    /// Private key PEM file path for the owner user of this instance
    pub user_private_key_path: PathBuf,

    #[serde(skip)]
    pub user_public_key: String,
    #[serde(skip)]
    pub user_private_key: String,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        let mut config: Config =
            envy::from_env().context("failed to parse config fro environment variables")?;
        let user_id = Url::parse(&format!("https://{}/ap/user", config.domain))
            .context("failed to construct ID URL")?;
        let inbox_url = Url::parse(&format!("https://{}/ap/inbox", config.domain))
            .context("failed to construct inbox URL")?;
        let user_public_key = std::fs::read_to_string(&config.user_public_key_path)
            .context("failed to read public key file")?;
        let user_private_key = std::fs::read_to_string(&config.user_private_key_path)
            .context("failed to read private key file")?;
        config.user_id = Some(user_id);
        config.inbox_url = Some(inbox_url);
        config.user_public_key = user_public_key;
        config.user_private_key = user_private_key;
        Ok(config)
    }
}

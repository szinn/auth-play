use config::{Config, Environment};
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct AuthPlayDatabaseConfig {
    /// (required) Fully qualified URL for accessing Postgres server.
    /// e.g. postgres://user:password@host/database
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPlayHttpConfig {
    /// (required) Port to serve http traffic.
    pub port: u16,
    /// (required) Secret key for encrypting cookies.
    pub secret_key: String,
    /// (required) Google OAuth client id.
    pub google_oauth_client_id: String,
    /// (required) Google OAuth client secret.
    pub google_oauth_client_secret: String,
    /// (required) Google OAuth redirect URL.
    pub google_oauth_redirect_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPlayConfig {
    pub database: AuthPlayDatabaseConfig,
    pub http: AuthPlayHttpConfig,
}

impl AuthPlayConfig {
    pub fn load() -> Result<AuthPlayConfig, Error> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("AUTH_PLAY").try_parsing(true).separator("__"))
            .build()?;

        let config: AuthPlayConfig = config.try_deserialize()?;

        Ok(config)
    }
}

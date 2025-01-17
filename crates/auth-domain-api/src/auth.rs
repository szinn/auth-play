use async_trait::async_trait;
use auth_domain_models::auth::{NewUser, User};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_sha: String,
}

#[async_trait]
pub trait AuthApi: Send + Sync {
    async fn register(&self, user: &NewUser) -> Result<User, Error>;
    async fn authenticate(&self, email: &str, password: &str) -> Result<UserInfo, Error>;
    async fn get_user(&self, id: i64) -> Result<UserInfo, Error>;
}

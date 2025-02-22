use async_trait::async_trait;
use auth_domain_models::auth::{NewSession, NewUser, Session, SessionId, User};
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

    async fn create_session(&self, new_session: &NewSession) -> Result<Session, Error>;
    async fn save_session(&self, session: &Session) -> Result<(), Error>;
    async fn load_session(&self, id: &SessionId) -> Result<Option<Session>, Error>;
    async fn delete_session(&self, id: &SessionId) -> Result<(), Error>;
}

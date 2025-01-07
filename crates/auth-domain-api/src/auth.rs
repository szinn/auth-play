use async_trait::async_trait;
use auth_domain_models::auth::{NewUser, User};

use crate::Error;

#[async_trait]
pub trait AuthApi: Send + Sync {
    async fn register(&self, user: &NewUser) -> Result<User, Error>;
}

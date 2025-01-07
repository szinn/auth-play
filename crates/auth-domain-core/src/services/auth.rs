use std::sync::Arc;

use async_trait::async_trait;
use auth_db::{adapters::UserAdapter, Repository};
use auth_domain_api::{AuthApi, Error};
use auth_domain_models::auth::{NewUser, User};
use auth_utils::arcbox::ArcBox;

#[derive(Clone)]
pub(crate) struct AuthService {
    repository: Arc<Repository>,
    user_adapter: ArcBox<dyn UserAdapter>,
}

impl AuthService {
    pub(crate) fn new(repository: Arc<Repository>, user_adapter: ArcBox<dyn UserAdapter>) -> Self {
        Self { repository, user_adapter }
    }
}

#[async_trait]
impl AuthApi for AuthService {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn register(&self, new_user: &NewUser) -> Result<User, Error> {
        let adapter = self.user_adapter.clone();
        let new_user = NewUser {
            name: new_user.name.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
        };

        let result: Result<User, auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.add_user(tx, &new_user).await }))
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use auth_db::{adapters::UserAdapter, Repository};
use auth_domain_api::{AuthApi, Error, UserInfo};
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

    #[tracing::instrument(level = "trace", skip(self, password))]
    async fn authenticate(&self, email: &str, password: &str) -> Result<UserInfo, Error> {
        let adapter = self.user_adapter.clone();
        let email = email.to_string();
        let password = password.to_string();

        let result: Result<User, auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.authenticate_user(tx, &email, &password).await }))
            .await;
        match result {
            Ok(user) => Ok(UserInfo {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
                password_sha: user.password_sha.clone(),
            }),
            Err(_) => Err(Error::NotFound),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_user(&self, id: i64) -> Result<UserInfo, Error> {
        let adapter = self.user_adapter.clone();

        let result: Result<User, auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.get_user_by_id(tx, id).await }))
            .await;
        tracing::info!("get_user result: {:?}", result);
        match result {
            Ok(user) => Ok(UserInfo {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
                password_sha: user.password_sha.clone(),
            }),
            Err(_) => Err(Error::NotFound),
        }
    }
}

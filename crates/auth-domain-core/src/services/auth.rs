use std::sync::Arc;

use async_trait::async_trait;
use auth_db::{
    adapters::{SessionAdapter, UserAdapter},
    Repository, RepositoryAdapters,
};
use auth_domain_api::{AuthApi, Error, UserInfo};
use auth_domain_models::auth::{NewSession, NewUser, Session, SessionId, User};
use auth_utils::arcbox::ArcBox;

#[derive(Clone)]
pub(crate) struct AuthService {
    repository: Arc<Repository>,
    user_adapter: ArcBox<dyn UserAdapter>,
    session_adapter: ArcBox<dyn SessionAdapter>,
}

impl AuthService {
    pub(crate) fn new(repository_adapters: Arc<RepositoryAdapters>) -> Self {
        Self {
            repository: repository_adapters.repository.clone(),
            user_adapter: repository_adapters.user_adapter.clone(),
            session_adapter: repository_adapters.session_adapter.clone(),
        }
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
    async fn create_session(&self, new_session: &NewSession) -> Result<Session, Error> {
        let adapter = self.session_adapter.clone();
        let new_session = new_session.clone();

        let result: Result<Session, auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.create_session(tx, &new_session).await }))
            .await;

        match result {
            Ok(session) => Ok(session),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn save_session(&self, session: &Session) -> Result<(), Error> {
        let adapter = self.session_adapter.clone();
        let session = session.clone();

        let result: Result<Session, auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.save_session(tx, &session).await }))
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn load_session(&self, id: &SessionId) -> Result<Option<Session>, Error> {
        let adapter = self.session_adapter.clone();
        let id = id.clone();

        let result: Result<Option<Session>, auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.load_session(tx, &id).await }))
            .await;

        match result {
            Ok(session) => Ok(session),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn delete_session(&self, id: &SessionId) -> Result<(), Error> {
        let adapter = self.session_adapter.clone();
        let id = id.clone();

        let result: Result<(), auth_db::Error> = self
            .repository
            .transaction(|tx| Box::pin(async move { adapter.delete_session(tx, &id).await }))
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }
}

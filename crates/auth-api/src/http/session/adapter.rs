use async_trait::async_trait;
use auth_domain_api::AuthApi;
use auth_utils::arcbox::ArcBox;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};
use tower_sessions::{
    session::{Id, Record},
    session_store, MemoryStore, SessionStore,
};

use crate::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_sha: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_sha.as_bytes() // We use the password sha as the auth
                                     // hash--what this means
                                     // is when the user changes their password the
                                     // auth session becomes invalid.
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub _next: Option<String>,
}

#[derive(Clone)]
pub(crate) struct SessionAdapter {
    pub auth_api: ArcBox<dyn AuthApi>,
    store: MemoryStore,
}

impl SessionAdapter {
    pub(crate) fn new(auth_api: ArcBox<dyn AuthApi>) -> Self {
        let store = MemoryStore::default();

        Self { auth_api, store }
    }
}

impl std::fmt::Debug for SessionAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionAdapter").finish()
    }
}

#[async_trait]
impl SessionStore for SessionAdapter {
    #[tracing::instrument(level = "trace")]
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        self.store.create(record).await
    }

    #[tracing::instrument(level = "trace")]
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        self.store.save(record).await
    }

    #[tracing::instrument(level = "trace")]
    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        self.store.load(session_id).await
    }

    #[tracing::instrument(level = "trace")]
    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        self.store.delete(session_id).await
    }
}

#[async_trait]
impl AuthnBackend for SessionAdapter {
    type User = User;
    type Credentials = Credentials;
    type Error = ApiError;

    #[tracing::instrument(level = "trace", skip(self, creds))]
    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let result = self.auth_api.authenticate(&creds.email, &creds.password).await;
        tracing::info!("Got {:?}", result);
        match result {
            Ok(user_info) => Ok(Some(Self::User {
                id: user_info.id,
                name: user_info.name.clone(),
                email: user_info.email.clone(),
                password_sha: user_info.password_sha.clone(),
            })),
            Err(_) => Err(Self::Error::UserNotFound(creds.email.clone())),
        }
    }

    #[tracing::instrument(level = "trace")]
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user_info = self.auth_api.get_user(*user_id).await?;

        Ok(Some(Self::User {
            id: user_info.id,
            name: user_info.name.clone(),
            email: user_info.email.clone(),
            password_sha: user_info.password_sha.clone(),
        }))
    }
}

pub type AuthSession = axum_login::AuthSession<SessionAdapter>;

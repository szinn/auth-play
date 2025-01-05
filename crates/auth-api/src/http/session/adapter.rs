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
    id: i64,
    pub username: String,
    password: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Clone)]
pub(crate) struct SessionAdapter {
    auth_api: ArcBox<dyn AuthApi>,
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

    #[tracing::instrument(level = "trace")]
    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        Ok(None)
    }

    #[tracing::instrument(level = "trace")]
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(None)
    }
}

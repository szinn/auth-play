use async_trait::async_trait;
use auth_domain_api::AuthApi;
use auth_domain_models::auth::{NewSession, Session};
use auth_utils::{arcbox, arcbox::ArcBox};
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tower_sessions::{
    session::{Id, Record},
    session_store, SessionStore,
};
use uuid::Uuid;

use crate::{http::Configuration, ApiError};

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
    pub config: ArcBox<Configuration>,
    pub auth_api: ArcBox<dyn AuthApi>,
}

impl SessionAdapter {
    pub(crate) fn new(config: &Configuration, auth_api: ArcBox<dyn AuthApi>) -> Self {
        let config = config.clone();
        Self {
            config: arcbox!(config),
            auth_api,
        }
    }

    fn to_uuid(id: i128) -> Uuid {
        let bytes = id.to_le_bytes();

        Uuid::from_bytes_le(bytes)
    }

    fn from_uuid(uuid: &Uuid) -> i128 {
        let bytes = uuid.to_bytes_le();

        i128::from_le_bytes(bytes)
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
        let data = rmp_serde::to_vec(record);
        if data.is_err() {
            return Err(session_store::Error::Encode("create session record".to_string()));
        }

        let new_session = NewSession {
            data: data.unwrap(),
            expiry: DateTime::<Utc>::from_timestamp(record.expiry_date.unix_timestamp(), 0).unwrap(),
        };

        match self.auth_api.create_session(&new_session).await {
            Ok(session) => {
                record.id = Id(SessionAdapter::from_uuid(&session.id));
                Ok(())
            }
            Err(_) => Err(session_store::Error::Encode("create session record".to_string())),
        }
    }

    #[tracing::instrument(level = "trace")]
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let data = rmp_serde::to_vec(record);
        if data.is_err() {
            return Err(session_store::Error::Encode("save session record".to_string()));
        }

        let session = Session {
            id: SessionAdapter::to_uuid(record.id.0),
            data: data.unwrap(),
            expiry: DateTime::<Utc>::from_timestamp(record.expiry_date.unix_timestamp(), 0).unwrap(),
        };

        match self.auth_api.save_session(&session).await {
            Ok(_) => Ok(()),
            Err(_) => Err(session_store::Error::Encode("save session record".to_string())),
        }
    }

    #[tracing::instrument(level = "trace")]
    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let uuid = SessionAdapter::to_uuid(session_id.0);

        match self.auth_api.load_session(&uuid).await {
            Ok(Some(session)) => {
                let mut record: Record = rmp_serde::from_slice(&session.data).unwrap();
                record.id = Id(SessionAdapter::from_uuid(&session.id));

                Ok(Some(record))
            }
            Ok(None) => Ok(None),
            Err(_) => Ok(None),
        }
    }

    #[tracing::instrument(level = "trace")]
    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let uuid = SessionAdapter::to_uuid(session_id.0);

        match self.auth_api.delete_session(&uuid).await {
            Ok(_) => Ok(()),
            Err(_) => Err(session_store::Error::Backend("delete session error".to_string())),
        }
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

    #[tracing::instrument(level = "trace", skip(self))]
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

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use super::SessionAdapter;

    #[test]
    fn id_uuid() {
        let id = -1;
        let uuid = SessionAdapter::to_uuid(id);
        assert_eq!(id, SessionAdapter::from_uuid(&uuid));

        let id = 1;
        let uuid = SessionAdapter::to_uuid(id);
        assert_eq!(id, SessionAdapter::from_uuid(&uuid));

        let uuid = Uuid::now_v7();
        let id = SessionAdapter::from_uuid(&uuid);
        assert_eq!(uuid, SessionAdapter::to_uuid(id));
    }
}

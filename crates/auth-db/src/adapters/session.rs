use async_trait::async_trait;
use auth_domain_models::auth::{NewSession, Session, SessionId};
use chrono::{TimeZone, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    entities::{prelude, sessions},
    Error,
};

#[async_trait]
pub trait SessionAdapter: Send + Sync {
    async fn create_session(&self, tx: &mut DatabaseTransaction, new_session: &NewSession) -> Result<Session, Error>;
    async fn save_session(&self, tx: &mut DatabaseTransaction, session: &Session) -> Result<Session, Error>;
    async fn load_session(&self, tx: &mut DatabaseTransaction, session_id: &SessionId) -> Result<Option<Session>, Error>;
    async fn delete_session(&self, tx: &mut DatabaseTransaction, session_id: &SessionId) -> Result<(), Error>;
}

pub(crate) struct SessionAdapterImpl {}

impl SessionAdapterImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn from_model(model: sessions::Model) -> Session {
        Session {
            id: model.uuid,
            data: model.data,
            expiry: Utc.from_local_datetime(&model.expiry).unwrap(),
        }
    }
    async fn id_exists(&self, tx: &mut DatabaseTransaction, id: &SessionId) -> Result<bool, Error> {
        let model = prelude::Sessions::find().filter(sessions::Column::Uuid.eq(*id)).one(tx).await?;

        Ok(model.is_some())
    }
}

#[async_trait]
impl SessionAdapter for SessionAdapterImpl {
    #[tracing::instrument(level = "trace", skip(self, tx, new_session))]
    async fn create_session(&self, tx: &mut DatabaseTransaction, new_session: &NewSession) -> Result<Session, Error> {
        let mut session_id = Uuid::now_v7();

        while self.id_exists(tx, &session_id).await? {
            session_id = Uuid::now_v7();
        }

        let new_session = sessions::ActiveModel {
            uuid: Set(session_id),
            data: Set(new_session.data.clone()),
            expiry: Set(new_session.expiry.naive_utc()),
            ..Default::default()
        };
        let session_model = new_session.insert(tx).await?;

        Ok(SessionAdapterImpl::from_model(session_model))
    }

    #[tracing::instrument(level = "trace", skip(self, tx, session))]
    async fn save_session(&self, tx: &mut DatabaseTransaction, session: &Session) -> Result<Session, Error> {
        let model = prelude::Sessions::find().filter(sessions::Column::Uuid.eq(session.id)).one(tx).await?;
        if model.is_none() {
            return Err(Error::NotFound);
        }
        let model = model.unwrap();

        let mut active_session: sessions::ActiveModel = model.into();

        active_session.data = Set(session.data.clone());
        active_session.expiry = Set(session.expiry.naive_utc());

        let model = active_session.update(tx).await?;

        Ok(SessionAdapterImpl::from_model(model))
    }

    #[tracing::instrument(level = "trace", skip(self, tx))]
    async fn load_session(&self, tx: &mut DatabaseTransaction, session_id: &SessionId) -> Result<Option<Session>, Error> {
        let model = prelude::Sessions::find()
            .filter(sessions::Column::Uuid.eq(*session_id).and(sessions::Column::Expiry.gt(Utc::now())))
            .one(tx)
            .await?;

        match model {
            Some(session) => Ok(Some(SessionAdapterImpl::from_model(session))),
            None => Ok(None),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, tx))]
    async fn delete_session(&self, tx: &mut DatabaseTransaction, session_id: &SessionId) -> Result<(), Error> {
        let model = prelude::Sessions::find().filter(sessions::Column::Uuid.eq(*session_id)).one(tx).await?;

        match model {
            Some(session) => {
                let active_session: sessions::ActiveModel = session.into();
                active_session.delete(tx).await?;

                Ok(())
            }
            None => Ok(()),
        }
    }
}

use async_trait::async_trait;
use auth_domain_models::auth::{NewUser, User};
use auth_utils::argon2::{check_password, hash_password, hash_string};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set};

use crate::{
    entities::{prelude, users},
    Error,
};

#[async_trait]
pub trait UserAdapter: Send + Sync {
    async fn add_user(&self, tx: &mut DatabaseTransaction, user: &NewUser) -> Result<User, Error>;
    async fn get_user(&self, tx: &mut DatabaseTransaction, email: &str) -> Result<User, Error>;
    async fn get_user_by_id(&self, tx: &mut DatabaseTransaction, id: i64) -> Result<User, Error>;
    async fn authenticate_user(&self, tx: &mut DatabaseTransaction, email: &str, password: &str) -> Result<User, Error>;
}

pub(crate) struct UserAdapterImpl {}

impl UserAdapterImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn from_model(model: users::Model) -> User {
        User {
            id: model.id,
            name: model.name,
            email: model.email,
            password_sha: hash_string(&model.password),
        }
    }
}

#[async_trait]
impl UserAdapter for UserAdapterImpl {
    #[tracing::instrument(level = "trace", skip(self, tx, new_user))]
    async fn add_user(&self, tx: &mut DatabaseTransaction, new_user: &NewUser) -> Result<User, Error> {
        let hashed_password = match hash_password(&new_user.password) {
            Ok(hash) => hash,
            Err(_) => return Err(Error::Message("System error".to_string())),
        };

        let new_user = users::ActiveModel {
            name: Set(new_user.name.clone()),
            email: Set(new_user.email.clone()),
            password: Set(hashed_password),
            ..Default::default()
        };
        let user_model = new_user.insert(tx).await?;

        Ok(UserAdapterImpl::from_model(user_model))
    }

    #[tracing::instrument(level = "trace", skip(self, tx))]
    async fn get_user(&self, tx: &mut DatabaseTransaction, email: &str) -> Result<User, Error> {
        let model = prelude::Users::find().filter(users::Column::Email.eq(email)).one(tx).await?;

        match model {
            Some(user) => Ok(UserAdapterImpl::from_model(user)),
            None => Err(Error::NotFound),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, tx))]
    async fn get_user_by_id(&self, tx: &mut DatabaseTransaction, id: i64) -> Result<User, Error> {
        let model = prelude::Users::find().filter(users::Column::Id.eq(id)).one(tx).await?;

        match model {
            Some(user) => Ok(UserAdapterImpl::from_model(user)),
            None => Err(Error::NotFound),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, tx, password))]
    async fn authenticate_user(&self, tx: &mut DatabaseTransaction, email: &str, password: &str) -> Result<User, Error> {
        let model = prelude::Users::find().filter(users::Column::Email.eq(email)).one(tx).await?;

        match model {
            Some(user) => match check_password(password, &user.password) {
                Ok(is_valid) => {
                    if is_valid {
                        Ok(UserAdapterImpl::from_model(user))
                    } else {
                        Err(Error::InvalidPassword)
                    }
                }
                Err(_) => Err(Error::InvalidPassword),
            },
            None => Err(Error::NotFound),
        }
    }
}

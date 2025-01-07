use async_trait::async_trait;
use auth_domain_models::auth::{NewUser, User};
use auth_utils::argon2::hash_string;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};

use crate::{entities::users, Error};

#[async_trait]
pub trait UserAdapter: Send + Sync {
    async fn add_user(&self, tx: &mut DatabaseTransaction, user: &NewUser) -> Result<User, Error>;
}

pub(crate) struct UserAdapterImpl {}

impl UserAdapterImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn from_model(model: users::Model) -> User {
        User {
            name: model.name,
            email: model.email,
        }
    }
}

#[async_trait]
impl UserAdapter for UserAdapterImpl {
    #[tracing::instrument(level = "trace", skip(self, tx, new_user))]
    async fn add_user(&self, tx: &mut DatabaseTransaction, new_user: &NewUser) -> Result<User, Error> {
        let hashed_password = match hash_string(&new_user.password) {
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
}

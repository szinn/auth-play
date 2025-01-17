mod auth;
mod health;

mod error;

use auth_utils::arcbox::ArcBox;

pub use error::Error;

pub use auth::{AuthApi, UserInfo};
pub use health::HealthApi;

pub struct AuthDomainApi {
    pub auth_api: ArcBox<dyn AuthApi>,
    pub health_api: ArcBox<dyn HealthApi>,
}

mod error;
mod health;

use auth_utils::arcbox::ArcBox;

pub use error::Error;
pub use health::HealthApi;

pub struct AuthApi {
    pub health_api: ArcBox<dyn HealthApi>,
}

use auth_domain_api::{AuthApi, HealthApi};
use auth_utils::{arcbox, arcbox::ArcBox};
use error::Error;
use health::HealthService;

mod error;
mod health;

#[tracing::instrument(level = "trace")]
pub async fn create_auth() -> Result<AuthApi, Error> {
    let health_service = HealthService::new();

    let health_api: ArcBox<dyn HealthApi> = arcbox!(health_service);

    Ok(AuthApi { health_api })
}

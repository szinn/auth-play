use auth::AuthService;
use auth_domain_api::{AuthApi, AuthDomainApi, HealthApi};
use auth_utils::{arcbox, arcbox::ArcBox};
use error::Error;
use health::HealthService;

mod error;

mod auth;
mod health;

#[tracing::instrument(level = "trace")]
pub async fn create_auth() -> Result<AuthDomainApi, Error> {
    let auth_service = AuthService::new();
    let health_service = HealthService::new();

    let auth_api: ArcBox<dyn AuthApi> = arcbox!(auth_service);
    let health_api: ArcBox<dyn HealthApi> = arcbox!(health_service);

    Ok(AuthDomainApi { auth_api, health_api })
}

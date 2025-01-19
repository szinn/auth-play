use std::sync::Arc;

use auth::AuthService;
use auth_db::RepositoryAdapters;
use auth_domain_api::{AuthApi, AuthDomainApi, HealthApi};
use auth_utils::{arcbox, arcbox::ArcBox};
use health::HealthService;

mod error;
mod services;

pub use error::*;
pub(crate) use services::*;

#[tracing::instrument(level = "trace", skip(repository_adapters))]
pub async fn create_auth(repository_adapters: Arc<RepositoryAdapters>) -> Result<AuthDomainApi, Error> {
    let auth_service = AuthService::new(repository_adapters.clone());
    let health_service = HealthService::new();

    let auth_api: ArcBox<dyn AuthApi> = arcbox!(auth_service);
    let health_api: ArcBox<dyn HealthApi> = arcbox!(health_service);

    Ok(AuthDomainApi { auth_api, health_api })
}

use async_trait::async_trait;
use auth_domain_api::HealthApi;

#[derive(Clone)]
pub(crate) struct HealthService {}

impl HealthService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl HealthApi for HealthService {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn is_healthy(&self) -> bool {
        true
    }
}

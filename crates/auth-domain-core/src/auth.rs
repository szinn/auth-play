use async_trait::async_trait;
use auth_domain_api::AuthApi;

#[derive(Clone)]
pub(crate) struct AuthService {}

impl AuthService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthApi for AuthService {}

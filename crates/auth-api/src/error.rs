#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad port - {}", _0)]
    BadPort(u16),

    #[error(transparent)]
    DomainError(#[from] auth_domain_api::Error),
}

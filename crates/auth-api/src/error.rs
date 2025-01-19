#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad port - {}", _0)]
    BadPort(u16),

    #[error("Encode/decode error")]
    EncodeDecodeError,

    #[error(transparent)]
    DomainError(#[from] auth_domain_api::Error),

    #[error("Not found - {}", _0)]
    UserNotFound(String),
}

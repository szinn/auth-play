#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad port - {}", _0)]
    BadPort(u16),
}

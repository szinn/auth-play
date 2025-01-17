#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("Not found")]
    NotFound,

    #[error("Invalid email or password")]
    InvalidPassword,

    #[error(transparent)]
    DatabaseError(#[from] auth_db::Error),
}

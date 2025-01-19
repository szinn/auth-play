use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_sha: String,
}

pub type SessionId = Uuid;

#[derive(Debug, Clone)]
pub struct NewSession {
    pub data: Vec<u8>,
    pub expiry: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub data: Vec<u8>,
    pub expiry: DateTime<Utc>,
}

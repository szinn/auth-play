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

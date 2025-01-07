use argon2::{
    password_hash::{errors::Result, rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash_string(string: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let hash = argon2.hash_password(string.as_bytes(), &salt)?.to_string();

    Ok(hash)
}

pub fn check_hash(string: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(Argon2::default().verify_password(string.as_bytes(), &parsed_hash).is_ok())
}

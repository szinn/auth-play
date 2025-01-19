use argon2::{
    password_hash::{errors::Result, rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};

pub fn hash_password(string: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let hash = argon2.hash_password(string.as_bytes(), &salt)?.to_string();

    Ok(hash)
}

pub fn check_password(string: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(Argon2::default().verify_password(string.as_bytes(), &parsed_hash).is_ok())
}

pub fn hash_string(string: &str) -> String {
    let mut hasher = Shake128::default();
    hasher.update(string.as_bytes());
    let mut reader = hasher.finalize_xof();
    let mut buf = [0u8; 10];
    reader.read(&mut buf);

    base16::encode_lower(&buf)
}

#[cfg(test)]
mod test {
    use crate::argon2::check_password;

    use super::{hash_password, hash_string};

    #[test]
    fn basic() {
        let password = "FooBar";
        let hash_password = hash_password(password).unwrap();
        let hash1 = hash_string(&hash_password);
        let hash2 = hash_string(&hash_password);

        assert_eq!(hash1, hash2);

        assert!(check_password(password, &hash_password).unwrap());
    }
}

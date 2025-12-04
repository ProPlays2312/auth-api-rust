use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use anyhow::anyhow;

// Turns a plain password into a secure hash
pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

// Verifies a plain password against a stored hash
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, anyhow::Error> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| anyhow!(e.to_string()))?;

    let ok = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(ok)
}
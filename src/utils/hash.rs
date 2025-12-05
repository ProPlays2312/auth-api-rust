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

    let params = argon2::Params::new(4096, 3, 1, Some(4096))
        .map_err(|e| anyhow!(e.to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e.to_string()))?
        .to_string();
    // let argon2 = Argon2::;
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
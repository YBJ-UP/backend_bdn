use argon2::{Argon2, PasswordHasher, password_hash::phc::SaltString};

use crate::model::errors::ApiError;

pub fn hash_password(pw: &str) -> Result<String, ApiError> {
    let sal = SaltString::generate();

    let hashed = Argon2::default()
        .hash_password_with_salt(pw.as_bytes(), sal.as_bytes())
        .map_err(|_e| ApiError::CreatedToken)?;

    Ok(hashed.to_string())
}

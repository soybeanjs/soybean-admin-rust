use std::error::Error;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub struct SecureUtil;

impl SecureUtil {
    pub fn hash_password(password: &[u8]) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt)?.to_string();
        Ok(password_hash)
    }

    pub fn verify_password(password: &[u8], password_hash: &str) -> Result<bool, Box<dyn Error>> {
        let parsed_hash = PasswordHash::new(password_hash)?;

        match Argon2::default().verify_password(password, &parsed_hash) {
            Ok(_) => Ok(true),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verification() {
        let password = b"example_password";
        let password_hash = SecureUtil::hash_password(password)
            .expect("Failed to hash password, check the input and environment setup");

        assert!(
            SecureUtil::verify_password(password, &password_hash).is_ok(),
            "Password verification should succeed for the correct password"
        );

        let wrong_password = b"wrong_password";
        assert!(
            SecureUtil::verify_password(wrong_password, &password_hash).is_err(),
            "Password verification should fail for the wrong password"
        );
    }
}
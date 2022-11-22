use std::sync::Arc;

use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;

#[derive(Clone)]
pub struct Password {
    inner: Arc<Inner>,
}

struct Inner {
    salt: SaltString,
}

impl Password {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                salt: SaltString::generate(&mut OsRng),
            }),
        }
    }

    pub fn hash(&self, password: &str) -> Result<PasswordHash, pbkdf2::password_hash::Error> {
        pbkdf2::Pbkdf2.hash_password(password.as_bytes(), &self.inner.salt)
    }

    pub fn verify(
        &self,
        password: &str,
        hashed_password: &str,
    ) -> Result<(), pbkdf2::password_hash::Error> {
        pbkdf2::Pbkdf2.verify_password(password.as_bytes(), &PasswordHash::new(hashed_password)?)
    }
}

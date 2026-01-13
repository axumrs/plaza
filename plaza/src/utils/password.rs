use crate::{Error, Result};

pub fn hash(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(Error::from)
}

pub fn verify(password: &str, hashed_password: &str) -> Result<bool> {
    bcrypt::verify(password, hashed_password).map_err(Error::from)
}

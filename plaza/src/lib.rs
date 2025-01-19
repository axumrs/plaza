pub mod config;
mod err;
pub mod mail;
pub mod turnstile;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

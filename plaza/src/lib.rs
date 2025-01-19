pub mod activation_code;
pub mod config;
mod err;
pub mod mail;
pub mod turnstile;
mod types;
pub mod user;
pub mod util;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub use types::*;

pub mod config;
mod err;
pub mod mail;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub mod activation_code;
pub mod config;
mod err;
pub mod jwt;
pub mod login_log;
pub mod mail;
pub mod mid;
pub mod resp;
pub mod router;
mod state;
pub mod turnstile;
mod types;
pub mod user;
pub mod util;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub use state::{AppState, ArcAppState};
pub use types::*;

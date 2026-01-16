pub mod api_resp;
pub mod captcha;
mod err;
pub mod jwt;
pub mod pb;
pub mod rds;
pub mod types;
pub mod user;
pub mod utils;
pub mod valid_code;

pub use err::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;

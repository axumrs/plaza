pub mod api_resp;
pub mod captcha;
mod err;
pub mod pb;
pub mod types;
pub mod user;
pub mod utils;

pub use err::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;

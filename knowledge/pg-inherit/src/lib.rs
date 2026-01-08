mod err;
pub mod form;
pub mod handler;
pub mod model;

pub use err::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;

pub struct AppState {
    pub pool: sqlx::PgPool,
}
pub type ArcAppState = std::sync::Arc<AppState>;

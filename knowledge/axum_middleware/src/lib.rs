use axum::response::IntoResponse;

pub mod mid;

pub struct Error(anyhow::Error);

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Error(e.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.0.to_string().into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct AppState {
    pub pool: sqlx::PgPool,
}

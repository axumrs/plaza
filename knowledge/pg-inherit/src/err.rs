use axum::response::IntoResponse;

pub struct Error(anyhow::Error);

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Error(err.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        format!("出错了：{}", self.0).into_response()
    }
}

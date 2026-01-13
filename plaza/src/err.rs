#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("数据库错误")]
    DataBase(#[from] sqlx::Error),

    #[error("密码哈希错误")]
    Password(#[from] bcrypt::BcryptError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self:?}");

        self.to_string().into_response()
    }
}

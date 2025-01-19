use axum::response::IntoResponse;

/// 错误
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// 未找到
    #[error("{0}")]
    NotFound(String),

    /// 配置文件错误
    #[error("配置文件错误: {0}")]
    ConfigError(#[from] ::config::ConfigError),

    #[error("{0}")]
    LettreError(#[from] lettre::error::Error),

    #[error("{0}")]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),

    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),

    /// 其它错误，来源于`anyhow::Error`
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// 错误码
    pub fn code(&self) -> i32 {
        match self {
            _ => -1,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.to_string().into_response()
    }
}

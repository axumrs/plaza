use axum::response::IntoResponse;

use crate::resp;

/// 错误
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// 未找到
    #[error("{0}")]
    NotFound(String),

    /// 配置文件错误
    #[error("配置文件错误: {0}")]
    ConfigError(#[from] ::config::ConfigError),

    #[error("构建邮件失败：{0}")]
    LettreError(#[from] lettre::error::Error),

    #[error("发送邮件失败：{0}")]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),

    #[error("发送HTTP请求失败：{0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("数据库错误：{0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("密码哈希失败：{0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("日期时间解析失败：{0}")]
    ChronoError(#[from] chrono::ParseError),

    #[error("转换为字符串失败：{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("验证失败：{0}")]
    ValidateError(#[from] validator::ValidationErrors),

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
        resp::Response::err(self).to_json().into_response()
    }
}

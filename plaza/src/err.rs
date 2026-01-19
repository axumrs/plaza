#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("数据库错误")]
    DataBase(#[from] sqlx::Error),

    #[error("密码哈希错误")]
    Password(#[from] bcrypt::BcryptError),

    #[error("gRPC错误")]
    Grpc(#[from] tonic::Status),

    #[error("验证错误: {0}")]
    Validate(#[from] validator::ValidationErrors),

    #[error("HTTP请求错误")]
    Reqwest(#[from] reqwest::Error),

    #[error("Redis错误")]
    Redis(#[from] redis::RedisError),

    #[error("序列化错误")]
    Serde(#[from] serde_json::Error),

    #[error("JWT错误")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("邮件错误")]
    Email(#[from] lettre::error::Error),

    #[error("邮件发送错误")]
    EmailSend(#[from] lettre::transport::smtp::Error),

    #[error("邮件地址错误")]
    EmailAddress(#[from] lettre::address::AddressError),

    #[error("IO错误")]
    Io(#[from] std::io::Error),

    #[error("ETCD错误")]
    Etcd(#[from] etcd_client::Error),

    #[error("{0}")]
    Custom(&'static str),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn code(&self) -> i32 {
        -1
    }

    pub fn msg(&self) -> String {
        self.to_string()
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self:?}");

        self.to_string().into_response()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HttpClient {
    /// IP地址
    pub ip: String,
    /// 地理位置
    pub loc: String,
    /// 用户代理
    pub user_agent: String,
}

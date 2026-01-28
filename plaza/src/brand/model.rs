use crate::types;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 品牌
#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Brand {
    /// 品牌 ID (CHAR(20))
    pub id: String,

    /// 品牌名称
    pub name: String,

    /// 品牌 LOGO URL
    pub logo: String,

    /// 创建时间
    /// 使用 chrono::DateTime<Utc> 对应 PostgreSQL 的 TIMESTAMPTZ
    pub created_at: types::Timestamp,
}

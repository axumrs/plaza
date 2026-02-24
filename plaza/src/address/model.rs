use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::types;

/// 收货地址实体 (对应 address 表)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Address {
    /// 地址唯一ID (CHAR(20))
    pub id: String,

    /// 关联的用户ID
    pub user_id: String,

    /// 是否为默认地址
    pub is_default: bool,

    /// 收货人姓名
    pub consignee: String,

    /// 电话号码
    pub phone: String,

    /// 详细地址 (街道、门牌号等)
    pub address: String,

    /// 省份/直辖市
    pub province: String,

    /// 城市
    pub city: String,

    /// 县/区
    pub county: String,

    /// 邮编 (需符合6位校验)
    pub post_code: String,

    /// 创建时间
    /// 使用 chrono::DateTime<Utc> 对应 PostgreSQL 的 TIMESTAMPTZ
    pub created_at: types::Timestamp,
}

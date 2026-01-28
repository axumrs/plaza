use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::{audit, types};

/// 店铺类型枚举
#[derive(Debug, Default, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "shop_kind")]
pub enum ShopKind {
    #[default]
    Standard,
    OfficialFlagship,
    Flagship,
    MallFlagship,
    Specialty,
    Franchise,
}

/// 店铺主模型
#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Shop {
    /// 唯一标识店铺的ID (CHAR(20))
    pub id: String,
    /// 对应的商家ID
    pub merchant_id: String,
    /// 主营类目ID
    pub category_id: String,
    /// 保证金 (存储为分，使用 i64 避免浮点误差)
    pub deposit: i64,
    /// 店铺名称
    pub name: String,
    /// 店铺类型
    pub kind: ShopKind,
    /// 店铺描述
    pub description: String,
    /// 店铺创建时间
    pub created_at: types::Timestamp,
    /// 店铺审核状态
    pub status: audit::model::AuditStatus,
    /// 店铺元数据 (授权信息等)
    /// sqlx::types::Json 会自动处理 JSONB 序列化
    pub meta: sqlx::types::Json<ShopMeta>,
    /// 是否平台自营
    pub is_platform_self_operated: bool,
}

/// 店铺元数据 (存储在 Shop 的 meta 字段中)
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ShopMeta {
    /// 授权方
    pub licensor: String,
    /// 品牌名称
    pub brand_name: String,
    /// 被授权方
    pub licensee: String,
    /// 授权期限 (DateRange)
    pub expiry_date: types::DateRange,
    /// 授权日期
    pub authorization_date: types::Timestamp,
    /// 证明文件
    pub proof: Vec<String>,
}

/// 店铺审核记录
#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct ShopAudit {
    pub id: String,
    pub merchant_id: String,
    pub shop_id: String,
    pub auditor_id: String,
    pub audit_status: audit::model::AuditStatus,
    pub audit_comments: String,
    pub audit_date: types::Timestamp,
}

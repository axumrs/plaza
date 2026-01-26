use serde::{Deserialize, Serialize};

use crate::{audit, types};

/// 商家类型
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "merchant_kind")]
pub enum MerchantKind {
    #[default]
    /// 企业
    Enterprise,
    /// 个体户
    SoleProprietorship,
    /// 个人
    Individual,
}

/// 商家
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Merchant {
    pub id: String,
    /// 商户名称
    pub name: String,
    /// 简称
    pub short_name: String,
    /// 状态
    pub status: audit::model::AuditStatus,
    /// 类型
    pub kind: MerchantKind,
    /// 创建时间
    pub created_at: types::Timestamp,
    /// 元数据
    pub meta: sqlx::types::Json<MerchantMeta>,
}

/// 商家元数据
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MerchantMeta {
    /// 营业执照(照片)
    pub business_license: Option<String>,
    /// 证件注册号/统一社会信用代码
    pub business_license_number: Option<String>,
    /// 商户名称
    pub name: Option<String>,
    /// 商户简称
    pub short_name: Option<String>,
    /// 经营者/法定代表人姓名
    pub legal_representative_name: Option<String>,
    /// 营业期限
    pub business_license_expiry_date: Option<types::Timestamp>,
    /// 注册地址
    pub registered_address: Option<String>,
    /// 经营者/法人身份证/护照照片（正反面）
    pub legal_representative_id_pic: Option<(String, String)>,
    /// 法人身份证姓名
    pub legal_representative_id_name: Option<String>,
    /// 法人身份证号
    pub legal_representative_id_number: Option<String>,
    /// 法人身份证有效期
    pub legal_representative_id_expiry_date: Option<(types::Timestamp, types::Timestamp)>,
    /// 法人身份证地址
    pub legal_representative_id_address: Option<String>,
    /// 账户地址(web3钱包地址)
    pub account_address: Option<String>,
    /// 超级管理员类型,是否是法人(经营者/法人, 经办人)
    pub super_admin_is_legal_representative: Option<bool>,
    /// 超级管理员(如果是经办人)身份证/护照照片(正反面)
    pub super_admin_id_pic: Option<(String, String)>,
    /// 超级管理员(如果是经办人)身份证有效期
    pub super_admin_id_expiry_date: Option<(types::Timestamp, types::Timestamp)>,
    /// 超级管理员身份证姓名
    pub super_admin_id_name: Option<String>,
    /// 超级管理员身份证号
    pub super_admin_id_number: Option<String>,
    /// 超级管理员手机号码
    pub super_admin_phone: Option<String>,
    /// 超级管理员邮箱
    pub super_admin_email: Option<String>,
    /// 特殊资质（照片，0~5张）
    pub special_qualification_pics: Option<Vec<String>>,
    /// 补充材料（照片，0~5张）
    pub supplementary_material_pics: Option<Vec<String>>,
    /// 补充说明（0~500个字）
    pub supplementary_explain: Option<String>,
}

/// 商家账号
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct MerchantAccount {
    pub id: String,
    /// 对应的商家ID
    pub merchant_id: String,
    /// 邮箱
    pub email: String,
    /// 密码
    pub password: String,
    /// 昵称
    pub nickname: String,
    /// 是否超级用户
    pub is_super: bool,
    /// 权限
    pub permission: i64,
    /// 创建时间
    pub created_at: types::Timestamp,
}

/// 商家审核
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct MerchantAudit {
    pub id: String,
    /// 对应的商家ID
    pub merchant_id: String,
    /// 审核员的ID
    pub auditor_id: String,
    /// 商家审核状态
    pub audit_status: audit::model::AuditStatus,
    /// 审核的备注或意见
    pub audit_comments: String,
    /// 审核的时间
    pub audit_date: types::Timestamp,
}

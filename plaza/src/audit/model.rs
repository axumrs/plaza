use serde::{Deserialize, Serialize};

/// 审核状态
#[derive(Debug, Default, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "audit_status")]
pub enum AuditStatus {
    #[default]
    /// 未审核
    Pending,
    /// 审核通过
    Approved,
    /// 拒绝
    Rejected,
}

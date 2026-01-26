use crate::pb;

use super::model::AuditStatus;

impl From<pb::audit::AuditStatus> for AuditStatus {
    fn from(v: pb::audit::AuditStatus) -> Self {
        match v {
            pb::audit::AuditStatus::Pending => AuditStatus::Pending,
            pb::audit::AuditStatus::Approved => AuditStatus::Approved,
            pb::audit::AuditStatus::Rejected => AuditStatus::Rejected,
        }
    }
}

impl Into<pb::audit::AuditStatus> for AuditStatus {
    fn into(self) -> pb::audit::AuditStatus {
        match self {
            AuditStatus::Pending => pb::audit::AuditStatus::Pending,
            AuditStatus::Approved => pb::audit::AuditStatus::Approved,
            AuditStatus::Rejected => pb::audit::AuditStatus::Rejected,
        }
    }
}

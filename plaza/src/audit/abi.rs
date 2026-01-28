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

impl From<i32> for AuditStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => AuditStatus::Pending,
            1 => AuditStatus::Approved,
            2 => AuditStatus::Rejected,
            _ => AuditStatus::default(),
        }
    }
}

impl Into<i32> for AuditStatus {
    fn into(self) -> i32 {
        match self {
            AuditStatus::Pending => 0,
            AuditStatus::Approved => 1,
            AuditStatus::Rejected => 2,
        }
    }
}

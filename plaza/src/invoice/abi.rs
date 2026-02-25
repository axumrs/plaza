use super::model::{Invoice, InvoiceKind, InvoiceStatus};
use crate::{pb::invoice as pb, types};

impl From<pb::InvoiceStatus> for InvoiceStatus {
    fn from(v: pb::InvoiceStatus) -> Self {
        match v {
            pb::InvoiceStatus::Unpaid => InvoiceStatus::Unpaid,
            pb::InvoiceStatus::Paid => InvoiceStatus::Paid,
            pb::InvoiceStatus::Overdue => InvoiceStatus::Overdue,
            pb::InvoiceStatus::Cancelled => InvoiceStatus::Cancelled,
            pb::InvoiceStatus::Refunded => InvoiceStatus::Refunded,
        }
    }
}

impl Into<pb::InvoiceStatus> for InvoiceStatus {
    fn into(self) -> pb::InvoiceStatus {
        match self {
            InvoiceStatus::Unpaid => pb::InvoiceStatus::Unpaid,
            InvoiceStatus::Paid => pb::InvoiceStatus::Paid,
            InvoiceStatus::Overdue => pb::InvoiceStatus::Overdue,
            InvoiceStatus::Cancelled => pb::InvoiceStatus::Cancelled,
            InvoiceStatus::Refunded => pb::InvoiceStatus::Refunded,
        }
    }
}

impl From<i32> for InvoiceStatus {
    fn from(v: i32) -> Self {
        match v {
            0 => InvoiceStatus::Unpaid,
            1 => InvoiceStatus::Paid,
            2 => InvoiceStatus::Overdue,
            3 => InvoiceStatus::Cancelled,
            4 => InvoiceStatus::Refunded,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for InvoiceStatus {
    fn into(self) -> i32 {
        match self {
            InvoiceStatus::Unpaid => 0,
            InvoiceStatus::Paid => 1,
            InvoiceStatus::Overdue => 2,
            InvoiceStatus::Cancelled => 3,
            InvoiceStatus::Refunded => 4,
        }
    }
}

impl From<pb::InvoiceKind> for InvoiceKind {
    fn from(value: pb::InvoiceKind) -> Self {
        match value {
            pb::InvoiceKind::OrderPayment => InvoiceKind::OrderPayment,
            pb::InvoiceKind::OrderRefund => InvoiceKind::OrderRefund,
            pb::InvoiceKind::Withdrawal => InvoiceKind::Withdrawal,
            pb::InvoiceKind::Recharge => InvoiceKind::Recharge,
            pb::InvoiceKind::Commission => InvoiceKind::Commission,
            pb::InvoiceKind::PlatformService => InvoiceKind::PlatformService,
        }
    }
}

impl Into<pb::InvoiceKind> for InvoiceKind {
    fn into(self) -> pb::InvoiceKind {
        match self {
            InvoiceKind::OrderPayment => pb::InvoiceKind::OrderPayment,
            InvoiceKind::OrderRefund => pb::InvoiceKind::OrderRefund,
            InvoiceKind::Withdrawal => pb::InvoiceKind::Withdrawal,
            InvoiceKind::Recharge => pb::InvoiceKind::Recharge,
            InvoiceKind::Commission => pb::InvoiceKind::Commission,
            InvoiceKind::PlatformService => pb::InvoiceKind::PlatformService,
        }
    }
}

impl From<i32> for InvoiceKind {
    fn from(value: i32) -> Self {
        match value {
            0 => InvoiceKind::OrderPayment,
            1 => InvoiceKind::OrderRefund,
            2 => InvoiceKind::Withdrawal,
            3 => InvoiceKind::Recharge,
            4 => InvoiceKind::Commission,
            5 => InvoiceKind::PlatformService,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for InvoiceKind {
    fn into(self) -> i32 {
        match self {
            InvoiceKind::OrderPayment => 0,
            InvoiceKind::OrderRefund => 1,
            InvoiceKind::Withdrawal => 2,
            InvoiceKind::Recharge => 3,
            InvoiceKind::Commission => 4,
            InvoiceKind::PlatformService => 5,
        }
    }
}

impl From<pb::Invoice> for Invoice {
    fn from(v: pb::Invoice) -> Self {
        Self {
            id: v.id,
            order_id: v.order_id,
            user_id: v.user_id,
            shop_id: v.shop_id,
            amount: v.amount,
            pay_amount: v.pay_amount,
            status: v.status.into(),
            kind: v.kind.into(),
            pay_tx_id: v.pay_tx_id,
            remark: v.remark,
            due_date: types::prost2chrono(&v.due_date),
            paid_at: types::prost2chrono(&v.paid_at),
            created_at: types::prost2chrono(&v.created_at),
            updated_at: types::prost2chrono(&v.updated_at),
        }
    }
}

impl Into<pb::Invoice> for Invoice {
    fn into(self) -> pb::Invoice {
        pb::Invoice {
            id: self.id,
            order_id: self.order_id,
            user_id: self.user_id,
            shop_id: self.shop_id,
            amount: self.amount,
            pay_amount: self.pay_amount,
            status: self.status.into(),
            kind: self.kind.into(),
            pay_tx_id: self.pay_tx_id,
            remark: self.remark,
            due_date: types::chrono2prost(self.due_date),
            paid_at: types::chrono2prost(self.paid_at),
            created_at: types::chrono2prost(self.created_at),
            updated_at: types::chrono2prost(self.updated_at),
        }
    }
}

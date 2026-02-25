use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::types;

/// 账单状态：处理从创建到核销的全生命周期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "invoice_status", rename_all = "PascalCase")]
pub enum InvoiceStatus {
    #[default]
    Unpaid, // 待支付
    Paid,      // 已支付
    Overdue,   // 已逾期
    Cancelled, // 已取消
    Refunded,  // 已退款
}

/// 账单类型：区分资金流向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "invoice_kind", rename_all = "PascalCase")]
pub enum InvoiceKind {
    #[default]
    OrderPayment, // 订单支付
    OrderRefund,     // 订单退款
    Withdrawal,      // 提现
    Recharge,        // 充值
    Commission,      // 佣金/分成
    PlatformService, // 平台服务费
}

/// 账单实体 (对应 invoices 表)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Invoice {
    /// 账单唯一 ID (CHAR(20))
    pub id: String,

    /// 关联的业务 ID (通常是订单 ID)
    pub order_id: String,

    /// 买家 ID
    pub user_id: String,

    /// 卖家店铺 ID
    pub shop_id: String,

    /// 应付金额 (单位：分，对应 BIGINT)
    pub amount: i64,

    /// 实付金额 (单位：分，对应 BIGINT)
    pub pay_amount: i64,

    /// 账单状态
    pub status: InvoiceStatus,

    /// 账单类型
    pub kind: InvoiceKind,

    /// 支付交易号
    pub pay_tx_id: String,

    /// 备注
    pub remark: String,

    /// 截止支付日期
    pub due_date: types::Timestamp,

    /// 支付时间
    pub paid_at: types::Timestamp,

    /// 创建时间
    pub created_at: types::Timestamp,

    /// 更新时间
    pub updated_at: types::Timestamp,
}

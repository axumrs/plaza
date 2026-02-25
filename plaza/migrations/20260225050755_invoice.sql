-- 账单状态：处理从创建到核销的全生命周期
CREATE TYPE "invoice_status" AS ENUM (
    'Unpaid',      -- 待支付
    'Paid',        -- 已支付
    'Overdue',     -- 已逾期
    'Cancelled',   -- 已取消
    'Refunded'     -- 已退款
);

-- 账单类型：区分资金流向
CREATE TYPE "invoice_kind" AS ENUM (
    'OrderPayment',    -- 订单支付
    'OrderRefund',     -- 订单退款
    'Withdrawal',      -- 提现
    'Recharge',        -- 充值
    'Commission',      -- 佣金/分成
    'PlatformService'  -- 平台服务费
);

CREATE TABLE IF NOT EXISTS "invoices" (
    -- 账单唯一 ID (CHAR(20))
    "id" CHAR(20) PRIMARY KEY,
    -- 关联的业务 ID (通常是订单 ID)
    "order_id" CHAR(20) NOT NULL,
    -- 买家 ID
    "user_id" CHAR(20) NOT NULL,
    -- 卖家店铺 ID
    "shop_id" CHAR(20) NOT NULL,
    -- 金额相关 (单位：分)
    "amount" BIGINT CHECK("amount" >= 0) NOT NULL DEFAULT 0, -- 应付金额
    "pay_amount" BIGINT CHECK("pay_amount" >= 0) NOT NULL DEFAULT 0, -- 实付金额
    -- 状态与类型
    "status" invoice_status NOT NULL DEFAULT 'Unpaid',
    "kind" invoice_kind NOT NULL DEFAULT 'OrderPayment',
    -- 支付交易号
    "pay_tx_id" TEXT NOT NULL DEFAULT '',
    -- 备注
    "remark" TEXT NOT NULL DEFAULT '',
    -- 时间戳
    "due_date" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 00:00:00+0',                -- 截止支付日期
    "paid_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 00:00:00+0',                 -- 支付时间
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
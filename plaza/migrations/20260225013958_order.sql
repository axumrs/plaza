-- 订单状态流转
CREATE TYPE "order_status" AS ENUM (
    'PendingPayment',   -- 待付款
    'PendingShipment',  -- 待发货
    'Shipped',          -- 已发货
    'Completed',        -- 已完成
    'Cancelled',        -- 已取消
    'Refunded'          -- 已退款
);

-- 支付方式
CREATE TYPE "payment_method" AS ENUM (
    'Web3',             -- Web3
    'WeChatPay',        -- 微信支付
    'Alipay',           -- 支付宝
    'Balance',          -- 余额支付
    'CreditCard'        -- 信用卡
);

-- 订单快照中的简易商品信息（用于减少多表关联查询）
CREATE TYPE "order_item_snap" AS (
    -- 商品信息
    "goods_id" CHAR(20), -- 商品ID
	"shop_id" CHAR(20) , -- 店铺
	"is_vir" BOOLEAN  , -- 是否虚拟商品
	"category_id" CHAR(20) , -- 分类
    "name" VARCHAR , -- 名称
	"images" VARCHAR[]  , -- 图片
	"status" goods_status  , -- 状态
	"detail" TEXT  , -- 详情
	"comment_need_audit" BOOLEAN  , -- 评论需审核
	"service_guarantee" VARCHAR[]  , -- 服务保障
	"tags" VARCHAR[]  , -- 标签
	"arguments" goods_argument[]  , -- 参数
	"sku_meta" goods_sku_meta[]  , -- SKU
	"fare" INTEGER , -- 运费
	"recommendations" VARCHAR[]  , -- 商品推荐

    -- SKU
    "sku_id" CHAR(20)  , -- SKU ID
    "sku_arr" VARCHAR[]  , -- SKU(数组)

    -- 组合信息
    "goods_full_name" VARCHAR  , -- 商品名称（含规格）
    "goods_image" VARCHAR  , -- 商品图片
    "price" BIGINT  , -- 价格
    "real_price" BIGINT  -- 实际价格
);

CREATE TABLE IF NOT EXISTS "orders" (
    "id" CHAR(20) PRIMARY KEY,         -- 订单号（建议使用雪花算法或KSUID）
    "user_id" CHAR(20) NOT NULL,       -- 买家ID
    "shop_id" CHAR(20) NOT NULL,       -- 卖家店铺ID
    
    -- 金额相关 (单位：分)
    "total_amount" BIGINT CHECK("total_amount" >= 0) NOT NULL,    -- 订单总金额 (商品原价累计)
    "pay_amount" BIGINT CHECK("pay_amount" >= 0 ) NOT NULL,      -- 实付金额 (扣除优惠、运费后)
    "freight_amount" BIGINT CHECK("freight_amount" >= 0) NOT NULL DEFAULT 0, -- 运费
    
    -- 状态相关
    "status" order_status NOT NULL DEFAULT 'PendingPayment',
    "pay_method" payment_method NOT NULL DEFAULT 'Web3',       -- 支付方式
    
    -- 收货信息快照 (防止用户修改地址后历史订单发生变化)
    "consignee_name" TEXT NOT NULL DEFAULT '',
    "consignee_phone" TEXT NOT NULL DEFAULT '',
    "consignee_address" TEXT NOT NULL DEFAULT '',

    -- 商品快照
    "items" order_item_snap[] NOT NULL DEFAULT '{}',

    -- 购物车快照
    "cart_items" cart_item[] NOT NULL DEFAULT '{}',
    
    -- 时间相关
    "pay_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 00:00:00+0',              -- 支付时间
    "ship_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 00:00:00+0',             -- 发货时间
    "completed_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 00:00:00+0',        -- 完成时间
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
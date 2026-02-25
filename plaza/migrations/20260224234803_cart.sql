CREATE TYPE "cart_item" AS (
    "goods_id" CHAR(20), -- 商品ID
    "sku_id" CHAR(20), -- sku ID
    "quantity" INTEGER, -- 数量
    "price" BIGINT,  -- 单价
    "real_price" BIGINT, -- 实际价格
    "goods_full_name" TEXT, -- 商品名称(含SKU)
    "goods_image" TEXT -- 商品图片
);

CREATE TABLE IF NOT EXISTS "carts" (
    "user_id" CHAR(20) NOT NULL,       -- 用户ID
    "shop_id" CHAR(20) NOT NULL,       -- 店铺ID（按店铺分组展示）
    "items" cart_item[] NOT NULL DEFAULT '{}',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("user_id", "shop_id")
);
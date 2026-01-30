CREATE TYPE "goods_status" AS ENUM ('Available', 'Unavailable'); -- 商品状态

CREATE TABLE IF NOT EXISTS "goods"( -- 商品
	"id" CHAR(20) PRIMARY KEY,
	"shop_id" CHAR(20) NOT NULL, -- 店铺
	"is_vir" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否虚拟商品
	"category_id" CHAR(20) NOT NULL, -- 分类
    "name" VARCHAR NOT NULL, -- 名称
	"images" VARCHAR[] NOT NULL DEFAULT '{}', -- 图片
	"status" goods_status NOT NULL DEFAULT 'Available' , -- 状态
	"detail" TEXT NOT NULL DEFAULT '', -- 详情
	"comment_need_audit" BOOLEAN NOT NULL DEFAULT FALSE, -- 评论需审核
	"service_guarantee" VARCHAR[] NOT NULL DEFAULT '{}', -- 服务保障
	"tags" VARCHAR[] NOT NULL DEFAULT '{}', -- 标签
	"arguments" JSONB NOT NULL DEFAULT '[]', -- 参数
	"has_sku" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否有SKU（多规格）
	"stock" BIGINT CHECK ("stock" >= 0) NOT NULL DEFAULT 0 , -- 库存总计
	"sales" BIGINT CHECK ("sales" >= 0) NOT NULL DEFAULT 0, -- 销量总计
	"sku" JSONB NOT NULL DEFAULT '[]', -- SKU
	"fare" BIGINT CHECK ( "fare" >= 0 ) NOT NULL DEFAULT 0, -- 运费
	"recommendations" VARCHAR[] NOT NULL DEFAULT '{}', -- 商品推荐
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "idx_goods_sku" ON "goods" USING GIN ("sku");

CREATE TABLE IF NOT EXISTS "goods_comments" ( -- 商品评论
    "id" CHAR(20) PRIMARY KEY,
    "goods_id" CHAR(20) NOT NULL, -- 商品
    "goods_full_name" VARCHAR NOT NULL, -- 商品名称（含规格）
    "is_self" BOOLEAN NOT NULL DEFAULT FALSE,  -- 是否商家自评
    "user_id" CHAR(20) NOT NULL, -- 用户（商家自评时，为空）
    "user_avatar" VARCHAR NOT NULL, -- 用户头像（商家自评时，可以杜撰）
    "user_nickname" VARCHAR NOT NULL, -- 用户昵称（商家自评时，可以杜撰）
    "content" TEXT NOT NULL, -- 评论内容
    "goods_star" INT NOT NULL, -- 商品评分
    "service_star" INT NOT NULL, -- 服务评分
    "images" VARCHAR[] NOT NULL DEFAULT '{}', -- 图片
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 好评是商品分数和服务分数都是5分的评论
-- 中评是商品分数和服务分数为3-4的评论
-- 差评是商品分数和服务分数为1-2的评论
-- 好评率=好评的评论数/总条数*100%

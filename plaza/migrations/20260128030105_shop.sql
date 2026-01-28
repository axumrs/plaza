-- 店铺类型
CREATE TYPE shop_kind AS ENUM (
    'Standard',         -- 普通店
    'OfficialFlagship', -- 官方旗舰店
    'Flagship',         -- 旗舰店
    'MallFlagship',         -- 卖场旗舰店
    'Specialty',        -- 专卖店
    'Franchise'         -- 专营店
);


-- 店铺
CREATE TABLE IF NOT EXISTS "shops" (
    "id" CHAR(20) PRIMARY KEY,            -- 唯一标识店铺的ID
    "merchant_id" CHAR(20) NOT NULL,             -- 对应的商家ID
	"category_id" CHAR(20) NOT NULL,             -- 主营类目ID
	"deposit" BIGINT CHECK("deposit" > 0) NOT NULL DEFAULT 0, -- 保证金
    "name" VARCHAR(100) NOT NULL,         -- 店铺的名称
	"kind" shop_kind NOT NULL DEFAULT 'Standard', -- 店铺类型
    "description" VARCHAR(255) NOT NULL DEFAULT '',                        -- 店铺的描述
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 店铺创建时间
    "status" audit_status NOT NULL DEFAULT 'Pending', -- 店铺状态
    "meta" JSONB NOT NULL DEFAULT '{}' -- 元数据
	"is_platform_self_operated" BOOLEAN NOT NULL DEFAULT FALSE -- 是否平台自营
);

-- 店铺审核
CREATE TABLE IF NOT EXISTS "shop_audits" (
    "id" CHAR(20) PRIMARY KEY,           -- 唯一标识审核记录的ID
    "merchant_id" CHAR(20) NOT NULL,             -- 对应的商家ID
	"shop_id" CHAR(20) NOT NULL,             -- 对应的店铺ID
    "auditor_id" CHAR(20) NOT NULL,                     -- 审核员的ID
    "audit_status" audit_status NOT NULL DEFAULT 'Pending',         -- 审核状态
    "audit_comments" VARCHAR(255) NOT NULL DEFAULT '',                     -- 审核的备注或意见
    "audit_date" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP -- 审核的时间
);
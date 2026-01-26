-- 商户类型
CREATE TYPE merchant_kind AS ENUM (
    'Enterprise',          -- 企业
    'SoleProprietorship', -- 个体户
    'Individual'           -- 个人
);

-- 商户
CREATE TABLE "merchants" (
    -- ID 使用 CHAR(20)，适用于 KSUID 或自定义固定长度分布式 ID
    "id"           CHAR(20) PRIMARY KEY,
    -- 商户名称
    "name"         TEXT NOT NULL,
    -- 简称
    "short_name"   TEXT NOT NULL,
    -- 状态
    "status"       audit_status NOT NULL DEFAULT 'Pending',
    -- 类型
    "kind"         merchant_kind NOT NULL,
    -- 创建时间
    "created_at"   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- 元数据 (对应 Rust 中的 MerchantMeta)
    -- 使用 JSONB 以获得更好的查询性能和索引支持
    "meta"         JSONB NOT NULL DEFAULT '{}'
);

-- 商家账户
CREATE TABLE "merchant_accounts" (
    -- ID 使用 CHAR(20)，建议存放 KSUID 或 Snowflake ID
    "id"           CHAR(20) PRIMARY KEY,
    -- 关联到商家表的 ID
    "merchant_id"  CHAR(20) NOT NULL,
    -- 登录邮箱，增加唯一索引和长度限制
    "email"        TEXT NOT NULL UNIQUE,
    -- 密码哈希值。注意：数据库应存储哈希（如 Argon2 或 Bcrypt），而非明文
    "password"     TEXT NOT NULL,
    -- 用户昵称
    "nickname"     TEXT NOT NULL,
    -- 是否为超级用户 (Rust bool 对应 Postgres boolean)
    "is_super"     BOOLEAN NOT NULL DEFAULT FALSE,
    -- 权限位，使用 BIGINT 对应 Rust i64
    "permission"   BIGINT NOT NULL DEFAULT 0,
    -- 创建时间
    "created_at"   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 商家入驻审核
CREATE TABLE "merchant_audits" (
    -- 审核记录 ID
    "id"               CHAR(20) PRIMARY KEY,
    -- 被审核的商家 ID
    "merchant_id"      CHAR(20) NOT NULL,
    -- 执行审核操作的管理员/审核员 ID
    "auditor_id"       CHAR(20) NOT NULL,
    -- 审核状态
    "audit_status"     audit_status NOT NULL DEFAULT 'Pending',
    -- 审核意见
    "audit_comments"   TEXT NOT NULL,
    -- 审核执行时间
    "audit_date"       TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- 用户状态
CREATE TYPE "user_status" AS ENUM ('Pending', 'Actived', 'Freezed');

-- 用户
CREATE TABLE IF NOT EXISTS "users"(
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(72) NOT NULL,
    "nickname" VARCHAR(30) NOT NULL,
    "status" user_status DEFAULT 'Pending',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08'
);

-- 激活码类型
CREATE TYPE "activation_code_kind" AS ENUM ('Register', 'ResetPassword');

-- 激活码
CREATE UNLOGGED TABLE IF NOT EXISTS "activation_codes"(
    "id" CHAR(20) PRIMARY KEY,
    "code" CHAR(6) NOT NULL UNIQUE,
    "user_id" CHAR(20) NOT NULL,
    "kind" activation_code_kind NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "next_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08',
    "expired_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08'
);

-- 登录用户类型
CREATE TYPE "login_user_kind" AS ENUM (
    -- 用户
    'User', 
    -- 商家
    'Merchant', 
    -- 管理员
    'Admin'
);

-- 登录日志
CREATE TABLE IF NOT EXISTS "login_logs"(
    "id" CHAR(20) PRIMARY KEY,
    "user_id" CHAR(20) NOT NULL,
    "user_kind" login_user_kind NOT NULL DEFAULT 'User',
    "client" JSONB NOT NULL DEFAULT '{"ip":"","loc":"","user_agent":""}',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
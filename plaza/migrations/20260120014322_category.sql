-- Add migration script here
-- 商品分类级别
CREATE TYPE "category_level" AS ENUM(
    -- 未指定
    'Unspecified', 
    -- 级别1
    'Level1', 
    -- 级别2
    'Level2',
    -- 级别3 
    'Level3'
);

 -- 商品分类
CREATE TABLE IF NOT EXISTS "categories" (
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL, -- 名称
    "parent" CHAR(20) NOT NULL DEFAULT '', -- 父级分类
    "path" VARCHAR NOT NULL DEFAULT '', -- 路径
    "level" category_level NOT NULL DEFAULT 'Unspecified', -- 级别
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "security_deposit"  BIGINT CHECK ("security_deposit" >= 0) NOT NULL DEFAULT 0, -- 保证金
    UNIQUE("name", "parent")
);
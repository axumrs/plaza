CREATE TABLE IF NOT EXISTS "brands" ( -- 品牌
    "id" CHAR(20) PRIMARY KEY,
    "name" TEXT NOT NULL, -- 名称
    "logo" TEXT NOT NULL, -- LOGO
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE("name")
);
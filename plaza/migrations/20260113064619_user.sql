-- Add migration script here

-- 用户状态
CREATE TYPE "user_status" AS ENUM ('Pending', 'Actived', 'Freezed');

-- 用户
CREATE TABLE IF NOT EXISTS "users"(
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(72) NOT NULL,
    "nickname" VARCHAR(30) NOT NULL,
    "status" user_status DEFAULT 'Pending',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "users_email_status_idx" ON "users" ("email","status");
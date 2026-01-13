/*
CREATE TYPE "user_status" AS ENUM ('Pending', 'Actived', 'Freezed');

-- 用户
CREATE TABLE IF NOT EXISTS "users"(
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(72) NOT NULL,
    "nickname" VARCHAR(30) NOT NULL,
    "status" user_status DEFAULT 'Pending',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
); */

use serde::{Deserialize, Serialize};

use crate::types;

#[derive(Default, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum UserStatus {
    #[default]
    Pending,
    Actived,
    Freezed,
}

#[derive(Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub nickname: String,
    pub status: UserStatus,
    pub created_at: types::Timestamp,
}

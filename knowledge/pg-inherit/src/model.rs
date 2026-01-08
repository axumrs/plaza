use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAccount {
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub user: User,

    pub balance: i32,
}

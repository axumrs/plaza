use serde::{Deserialize, Serialize};

use crate::{types, utils};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Default)]
pub enum CategoryLevel {
    #[default]
    Unspecified,
    Level1,
    Level2,
    Level3,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent: String,
    pub path: String,
    pub level: CategoryLevel,
    pub created_at: types::Timestamp,
    pub security_deposit: i64,
}

impl Category {
    pub fn new(
        name: impl Into<String>,
        parent: impl Into<String>,
        path: impl Into<String>,
        level: CategoryLevel,
        security_deposit: impl Into<i64>,
    ) -> Self {
        Self {
            id: utils::id::new(),
            name: name.into(),
            parent: parent.into(),
            path: path.into(),
            level,
            created_at: chrono::Utc::now(),
            security_deposit: security_deposit.into(),
        }
    }
}

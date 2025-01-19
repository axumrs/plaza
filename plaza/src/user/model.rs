use serde::{Deserialize, Serialize};

use crate::{util, DateTime, PaginationRange, Result};

/// 用户状态
#[derive(Debug, Default, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_status")]
pub enum UserStatus {
    /// 未激活
    #[default]
    Pending,
    /// 激活
    Actived,
    /// 冻结
    Freezed,
}

/// 用户
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    /// 邮箱
    pub email: String,
    /// 密码
    pub password: String,
    /// 昵称
    pub nickname: String,
    /// 状态
    pub status: UserStatus,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

impl User {
    /// 尝试创建
    pub fn try_new(
        email: String,
        password: String,
        nickname: String,
        status: UserStatus,
    ) -> Result<Self> {
        let password = util::pwd::hash(&password)?;
        let now = util::dt::now();
        Ok(User {
            id: util::id::new(),
            email,
            password,
            nickname,
            status,
            created_at: now,
            updated_at: now,
        })
    }
}

/// 查找条件
pub enum FindBy {
    ID(String),
    Email(String),
}

/// 查找过滤
pub struct FindFilter {
    /// 条件
    pub by: FindBy,
    /// 状态
    pub status: Option<UserStatus>,
}

/// 分页过滤
pub struct ListFilter {
    /// 邮箱
    pub email: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 状态
    pub status: Option<UserStatus>,
    /// 创建时间，范围
    pub create_at: Option<(DateTime, DateTime)>,
    /// 排序
    pub order: Option<String>,
    /// 分页
    pub pr: PaginationRange,
}

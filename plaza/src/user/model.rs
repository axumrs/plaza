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
    pub fn try_new_with_nickname(
        email: String,
        password: String,
        status: UserStatus,
        nickname: Option<String>,
    ) -> Result<Self> {
        let password = util::pwd::hash(&password)?;
        let now = util::dt::now();
        let id = util::id::new();

        // 默认昵称：`用户+ID`
        let default_nickname = format!("用户{}", id.to_uppercase());
        // 处理昵称
        let nickname = if let Some(nickname) = nickname {
            // 如果昵称过长，则使用默认昵称
            if nickname.len() > 30 {
                default_nickname
            } else {
                nickname
            }
        } else {
            default_nickname
        };
        Ok(User {
            id,
            email,
            password,
            nickname,
            status,
            created_at: now,
            updated_at: now,
        })
    }
    pub fn try_new(email: String, password: String, status: UserStatus) -> Result<Self> {
        Self::try_new_with_nickname(email, password, status, None)
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

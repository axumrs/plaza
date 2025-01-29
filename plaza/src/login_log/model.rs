use serde::{Deserialize, Serialize};

use crate::{util, DateTime, HttpClient, PaginationRange};

/// 登录用户类型
#[derive(Debug, Default, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "login_user_kind")]
pub enum LoginUserKind {
    #[default]
    /// 用户
    User,
    /// 商家
    Merchant,
    /// 管理员
    Admin,
}

/// 登录日志
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoginLog {
    pub id: String,
    /// 用户ID
    pub user_id: String,
    /// 用户类型
    pub user_kind: LoginUserKind,
    /// 客户端
    pub client: sqlx::types::Json<HttpClient>,
    /// 登录时间
    pub created_at: DateTime,
}

impl LoginLog {
    /// 创建
    pub fn new(
        user_id: impl Into<String>,
        user_kind: LoginUserKind,
        ip: impl Into<String>,
        loc: impl Into<String>,
        user_agent: impl Into<String>,
    ) -> Self {
        Self {
            id: util::id::new(),
            user_id: user_id.into(),
            user_kind,
            client: sqlx::types::Json(HttpClient {
                ip: ip.into(),
                loc: loc.into(),
                user_agent: user_agent.into(),
            }),
            created_at: util::dt::now(),
        }
    }

    /// 创建用户
    pub fn new_user(
        user_id: impl Into<String>,
        ip: impl Into<String>,
        loc: impl Into<String>,
        user_agent: impl Into<String>,
    ) -> Self {
        Self::new(user_id, LoginUserKind::User, ip, loc, user_agent)
    }

    /// 创建商家
    pub fn new_merchant(
        user_id: impl Into<String>,
        ip: impl Into<String>,
        loc: impl Into<String>,
        user_agent: impl Into<String>,
    ) -> Self {
        Self::new(user_id, LoginUserKind::Merchant, ip, loc, user_agent)
    }

    /// 创建管理员
    pub fn new_admin(
        user_id: impl Into<String>,
        ip: impl Into<String>,
        loc: impl Into<String>,
        user_agent: impl Into<String>,
    ) -> Self {
        Self::new(user_id, LoginUserKind::Admin, ip, loc, user_agent)
    }
}

pub struct ListFilter {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户类型
    pub user_kind: Option<LoginUserKind>,
    /// IP
    pub ip: Option<String>,
    /// 登录时间，范围
    pub create_at: Option<(DateTime, DateTime)>,
    /// 排序
    pub order: Option<String>,
    /// 分页
    pub pr: PaginationRange,
}

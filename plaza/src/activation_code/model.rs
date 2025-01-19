use serde::{Deserialize, Serialize};

use crate::{util, DateTime, Result};

/// 激活码类型
#[derive(Debug, Default, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "activation_code_kind")]
pub enum ActivationCodeKind {
    #[default]
    Register,
    ResetPassword,
}

/// 激活码
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActivationCode {
    pub id: String,
    /// 激活码
    pub code: String,
    /// 用户
    pub user_id: String,
    /// 类型
    pub kind: ActivationCodeKind,
    /// 创建时间
    pub created_at: DateTime,
    /// 下次重新生成时间
    pub next_at: DateTime,
    /// 过期时间
    pub expired_at: DateTime,
}

impl ActivationCode {
    /// 尝试创建
    pub fn try_new(
        user_id: String,
        kind: ActivationCodeKind,
        resend_duration: u32,
        expired_duration: u32,
    ) -> Result<Self> {
        let code = Self::gen_code()?;
        let created_at = util::dt::now();
        let next_at = created_at + chrono::Duration::seconds(resend_duration as i64);
        let expired_at = created_at + chrono::Duration::seconds(expired_duration as i64);
        Ok(ActivationCode {
            id: util::id::new(),
            code,
            user_id,
            kind,
            created_at,
            next_at,
            expired_at,
        })
    }

    /// 生成激活码
    pub fn gen_code() -> Result<String> {
        util::code::generate_code(6, false, false, true, false)
    }
}

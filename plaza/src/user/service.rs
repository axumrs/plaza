use anyhow::anyhow;
use sqlx::PgTransaction;

use crate::Result;

use super::{db, model};

pub async fn create(tx: &mut PgTransaction<'_>, m: &model::User) -> Result<String> {
    let email_exists = db::email_exists(&mut **tx, &m.email, None).await?;
    if email_exists {
        return Err(anyhow!("邮箱{}已存在", m.email).into());
    }
    let id = db::create(&mut **tx, m).await?;
    Ok(id.to_string())
}

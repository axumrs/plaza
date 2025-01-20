use anyhow::anyhow;
use sqlx::PgTransaction;

use crate::Result;

use super::{db, model};

pub async fn create(
    tx: &mut PgTransaction<'_>,
    m: model::ActivationCode,
    max_retry_count: u8,
) -> Result<String> {
    let mut code = m.code.clone();
    for _ in 0..max_retry_count {
        let code_exists = db::exists(&mut **tx, &code).await?;
        if !code_exists {
            let m = model::ActivationCode { code, ..m };
            db::create(&mut **tx, &m).await?;
            return Ok(m.code);
        }
        code = model::ActivationCode::gen_code()?;
    }
    return Err(anyhow!("生成激活码失败").into());
}

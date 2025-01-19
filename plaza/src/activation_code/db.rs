/*
CREATE UNLOGGED TABLE IF NOT EXISTS "activation_codes"(
    "id" CHAR(20) PRIMARY KEY,
    "code" CHAR(6) NOT NULL UNIQUE,
    "user_id" CHAR(20) NOT NULL,
    "kind" activation_code_kind NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "next_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08',
    "expired_at" TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01 08:00:00+08'
); */

use sqlx::{PgExecutor, Result};

use super::model;

type Model = model::ActivationCode;

/// 激活码是否存在
pub async fn exists(e: impl PgExecutor<'_>, code: &str) -> Result<bool> {
    let (c,): (i64,) =
        sqlx::query_as(r#"SELECT COUNT(*) FROM "activation_codes" WHERE "code" = $1"#)
            .bind(code)
            .fetch_one(e)
            .await?;
    Ok(c > 0)
}

/// 创建
pub async fn create(e: impl PgExecutor<'_>, m: &Model) -> Result<&str> {
    sqlx::query(r#"INSERT INTO "activation_codes" ("id","code","user_id","kind","created_at","next_at","expired_at") VALUES ($1,$2,$3,$4,$5,$6,$7)"#)
        .bind(&m.id)
        .bind(&m.code)
        .bind(&m.user_id)
        .bind(&m.kind)
        .bind(&m.created_at)
        .bind(&m.next_at)
        .bind(&m.expired_at)
        .execute(e)
        .await?;
    Ok(&m.id)
}

/// 查找
pub async fn find_by_code(e: impl PgExecutor<'_>, code: &str) -> Result<Option<Model>> {
    let m = sqlx::query_as(r#"SELECT "id","code","user_id","kind","created_at","next_at","expired_at" FROM "activation_codes" WHERE "code" = $1"#)
        .bind(code)
        .fetch_optional(e)
        .await?;
    Ok(m)
}

/// 删除
pub async fn delete(e: impl PgExecutor<'_>, id: &str) -> Result<u64> {
    let r = sqlx::query(r#"DELETE FROM "activation_codes" WHERE "id" = $1"#)
        .bind(id)
        .execute(e)
        .await?;
    Ok(r.rows_affected())
}

#[cfg(test)]
mod tests {
    use crate::{activation_code::model, config};

    fn get_cfg() -> crate::Result<config::Config> {
        config::Config::from_toml()
    }
    async fn get_pool() -> crate::Result<sqlx::PgPool> {
        let cfg = get_cfg()?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&cfg.database.dsn)
            .await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn should_create_activation_code() {
        let cfg = get_cfg().unwrap();
        let pool = get_pool().await.unwrap();
        let m = model::ActivationCode::try_new(
            "cu6b6vkdrfaml30r0qc0".into(),
            model::ActivationCodeKind::Register,
            cfg.activation_code.resend_duration,
            cfg.activation_code.expire_duration,
        )
        .unwrap();
        let m = model::ActivationCode {
            id: "cu6cab4drfatmo3n4tdg".into(),
            code: "300036".into(),
            ..m
        };
        let id = super::create(&pool, &m).await.unwrap();
        assert!(id.len() == 20);
    }

    #[tokio::test]
    async fn should_find_activation_code_by_code() {
        let pool = get_pool().await.unwrap();
        let m = super::find_by_code(&pool, "300036").await.unwrap();
        assert!(m.is_some());
        assert!(m.unwrap().code == "300036");
    }

    #[tokio::test]
    async fn should_delete_activation_code() {
        let pool = get_pool().await.unwrap();
        let m = super::delete(&pool, "cu6cab4drfatmo3n4tdg").await;
        assert!(m.is_ok());
        assert!(m.unwrap() == 1);
    }
}

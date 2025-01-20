use sqlx::{PgExecutor, Result};

use crate::util::dt;

use super::model;

type Model = model::User;

/// 检查邮箱是否存在
pub async fn email_exists(e: impl PgExecutor<'_>, email: &str, id: Option<&str>) -> Result<bool> {
    let mut q = sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM "users" WHERE "email" = "#);
    q.push_bind(email);
    if let Some(id) = id {
        q.push(r#" AND "id" <>"#).push_bind(id);
    }
    let (count,): (i64,) = q.build_query_as().fetch_one(e).await?;
    Ok(count > 0)
}

/// 创建
pub async fn create(e: impl PgExecutor<'_>, m: &Model) -> Result<&str> {
    let mut q = sqlx::QueryBuilder::new(
        r#"INSERT INTO "users"("id","email","password","nickname","status","created_at","updated_at") "#,
    );

    q.push_values(&[m], |mut b, m| {
        b.push_bind(&m.id)
            .push_bind(&m.email)
            .push_bind(&m.password)
            .push_bind(&m.nickname)
            .push_bind(&m.status)
            .push_bind(&m.created_at)
            .push_bind(&m.updated_at);
    });
    q.build().execute(e).await?;
    Ok(&m.id)
}

/// 更新
pub async fn update(e: impl PgExecutor<'_>, m: &Model) -> Result<u64> {
    let aff = sqlx::query(r#"UPDATE "users" SET "email" = $2, "password" = $3, "nickname" = $4, "status" = $5, "updated_at" = $6 WHERE "id" = $1"#)
        .bind(&m.id)
        .bind(&m.email)
        .bind(&m.password)
        .bind(&m.nickname)
        .bind(&m.status)
        .bind(&dt::now())
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

/// 删除
pub async fn delete(e: impl PgExecutor<'_>, id: &str) -> Result<u64> {
    let aff = sqlx::query(r#"DELETE FROM "users" WHERE "id" = $1"#)
        .bind(id)
        .execute(e)
        .await?
        .rows_affected();
    Ok(aff)
}

/// 查找单条记录
pub async fn find(e: impl PgExecutor<'_>, f: &model::FindFilter) -> Result<Option<Model>> {
    let mut q = sqlx::QueryBuilder::new(
        r#"SELECT "id","email","password","nickname","status","created_at","updated_at" FROM "users" WHERE 1=1"#,
    );
    match f.by {
        model::FindBy::ID(ref id) => q.push(r#" AND "id" = "#).push_bind(id),
        model::FindBy::Email(ref email) => q.push(r#" AND "email" = "#).push_bind(email),
    };

    q.push(" LIMIT 1");

    let m = q.build_query_as().fetch_optional(e).await?;
    Ok(m)
}

/// 分页数据
pub async fn list_data(e: impl PgExecutor<'_>, f: &model::ListFilter) -> Result<Vec<Model>> {
    let q = sqlx::QueryBuilder::new(
        r#"SELECT "id","email","password","nickname","status","created_at","updated_at" FROM "users" WHERE 1=1"#,
    );
    let mut q = build_list_query(q, f);

    let order = if let Some(v) = &f.order { v } else { "id DESC" };
    q.push(r#" ORDER BY "#)
        .push(order)
        .push(" LIMIT ")
        .push_bind(f.pr.page_size)
        .push(" OFFSET ")
        .push_bind(f.pr.offset());
    q.build_query_as().fetch_all(e).await
}

/// 分页统计
pub async fn list_count(e: impl PgExecutor<'_>, f: &model::ListFilter) -> Result<i64> {
    let q = sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM "users" WHERE 1=1"#);
    let mut q = build_list_query(q, f);

    let (count,): (i64,) = q.build_query_as().fetch_one(e).await?;
    Ok(count)
}

/// 构建分页查询语句
fn build_list_query<'a>(
    mut q: sqlx::QueryBuilder<'a, sqlx::Postgres>,
    f: &'a model::ListFilter,
) -> sqlx::QueryBuilder<'a, sqlx::Postgres> {
    if let Some(v) = &f.email {
        q.push(r#" AND "email" ILIKE "#)
            .push_bind(format!("%{}%", v));
    }

    if let Some(v) = &f.nickname {
        q.push(r#" AND "nickname" ILIKE "#)
            .push_bind(format!("%{}%", v));
    }

    if let Some(v) = &f.status {
        q.push(r#" AND "status" = "#).push_bind(v);
    }

    if let Some((start, end)) = &f.create_at {
        q.push(r#" AND ("created_at" BETWEEN "#)
            .push_bind(start)
            .push(" AND ")
            .push_bind(end)
            .push(")");
    }

    q
}

#[cfg(test)]
mod tests {
    use crate::{config, user::model, util};

    async fn get_pool() -> anyhow::Result<sqlx::PgPool> {
        let cfg = config::Config::from_toml()?;
        let pool = sqlx::PgPool::connect(&cfg.database.dsn).await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn should_create_user() {
        let pool = get_pool().await.unwrap();
        let m = super::model::User::try_new_with_nickname(
            format!("team@axum.rs"),
            format!("axum.rs"),
            model::UserStatus::Actived,
            Some(format!("AXUM中文网")),
        )
        .unwrap();
        let m = model::User {
            id: "cu6b6vkdrfaml30r0qc0".into(),
            ..m
        };
        let r = super::create(&pool, &m).await;
        assert!(r.is_ok());
        let id = r.unwrap();
        assert!(id.len() == 20);
        println!("{}", id);
    }

    #[tokio::test]
    async fn should_find_user_by_email() {
        let pool = get_pool().await.unwrap();
        let m = super::find(
            &pool,
            &model::FindFilter {
                by: model::FindBy::Email("team@axum.rs".into()),
                status: Some(model::UserStatus::Actived),
            },
        )
        .await
        .unwrap();

        assert!(m.is_some());
        assert!(m.unwrap().email == "team@axum.rs");
    }

    #[tokio::test]
    async fn should_find_user_by_id() {
        let pool = get_pool().await.unwrap();
        let m = super::find(
            &pool,
            &model::FindFilter {
                by: model::FindBy::ID("cu6b6vkdrfaml30r0qc0".into()),
                status: Some(model::UserStatus::Actived),
            },
        )
        .await
        .unwrap();
        assert!(m.is_some());
        assert!(m.unwrap().email == "team@axum.rs");
    }

    #[tokio::test]
    async fn should_update_user() {
        let pool = get_pool().await.unwrap();
        let m = super::find(
            &pool,
            &model::FindFilter {
                by: model::FindBy::ID("cu6b6vkdrfaml30r0qc0".into()),
                status: Some(model::UserStatus::Actived),
            },
        )
        .await
        .unwrap();
        let m = m.unwrap();
        let m = model::User {
            nickname: format!("{}-{}", m.nickname, util::dt::now().format("%Y%m%d%H%M%S")),
            ..m
        };
        let m = super::update(&pool, &m).await.unwrap();
        assert!(m == 1);
    }

    #[tokio::test]
    async fn should_list_user() {
        let pool = get_pool().await.unwrap();
        let f = model::ListFilter {
            email: Some("axum".into()),
            nickname: Some("axum".into()),
            status: Some(model::UserStatus::Actived),
            create_at: Some(util::dt::today()),
            order: Some("id ASC,created_at DESC".into()),
            pr: crate::PaginationRange {
                page: 0,
                page_size: 10,
            },
        };
        let r = super::list_data(&pool, &f).await;
        let count = super::list_count(&pool, &f).await;
        assert!(r.is_ok());
        assert!(count.is_ok());
        assert!(count.unwrap() > 0);
        assert!(r.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn should_delete_user() {
        let pool = get_pool().await.unwrap();
        let m = super::delete(&pool, "cu6b6vkdrfaml30r0qc0").await;
        assert!(m.is_ok());
        assert!(m.unwrap() == 1);
    }
}

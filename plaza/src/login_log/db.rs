use super::model;

/// 插入
pub async fn insert(e: impl sqlx::PgExecutor<'_>, m: &model::LoginLog) -> sqlx::Result<&str> {
    let mut q = sqlx::QueryBuilder::new(
        r#"
        INSERT INTO "login_logs"("id","user_id","user_kind","client","created_at")
    "#,
    );
    q.push_values(&[m], |mut b, m| {
        b.push_bind(&m.id)
            .push_bind(&m.user_id)
            .push_bind(&m.user_kind)
            .push_bind(&m.client)
            .push_bind(&m.created_at);
    });
    q.build().execute(e).await?;
    Ok(&m.id)
}

/// 通过ID查找
pub async fn find_by_id(
    e: impl sqlx::PgExecutor<'_>,
    id: &str,
) -> sqlx::Result<Option<model::LoginLog>> {
    sqlx::query_as(
        r#"SELECT "id","user_id","user_kind","client","created_at" FROM "login_logs" WHERE "id" = $1"#,
    ).bind(id).fetch_optional(e).await
}

/// 分页数据
pub async fn list_data(
    e: impl sqlx::PgExecutor<'_>,
    f: &model::ListFilter,
) -> sqlx::Result<Vec<model::LoginLog>> {
    let q = sqlx::QueryBuilder::new(
        r#"SELECT "id","user_id","user_kind","client","created_at" FROM "login_logs" WHERE 1=1"#,
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
pub async fn list_count(e: impl sqlx::PgExecutor<'_>, f: &model::ListFilter) -> sqlx::Result<i64> {
    let q = sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM "login_logs" WHERE 1=1"#);
    let mut q = build_list_query(q, f);

    let (count,): (i64,) = q.build_query_as().fetch_one(e).await?;
    Ok(count)
}

fn build_list_query<'a>(
    mut q: sqlx::QueryBuilder<'a, sqlx::Postgres>,
    f: &'a model::ListFilter,
) -> sqlx::QueryBuilder<'a, sqlx::Postgres> {
    if let Some(v) = &f.user_id {
        q.push(r#" AND "user_id" = "#).push_bind(v);
    }

    if let Some(v) = &f.user_kind {
        q.push(r#" AND "user_kind" = "#).push_bind(v);
    }

    if let Some(v) = &f.ip {
        q.push(r#" AND "client"->>"ip" ILIKE "#)
            .push_bind(format!("%{}%", v));
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

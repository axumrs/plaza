use sqlx::{PgPool, QueryBuilder, query, query_scalar};

use crate::{
    category::model,
    pb::{
        self,
        category::{
            Category, GetCategoryReply, GetCategoryRequest, ListAllCategoryReply,
            ListAllCategoryRequest, ListCategoryReply, ListCategoryRequest,
            UpdateSecurityDepositRequest,
        },
        req, resp,
    },
};

pub struct CategorySrv {
    pool: PgPool,
    // rds: rds::RdsCli,
}

#[tonic::async_trait]
impl pb::category::category_service_server::CategoryService for CategorySrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<Category>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let r = request.into_inner();
        let m = model::Category::from(r);
        let id = query_scalar(
            r#"INSERT INTO "categories" ("id","name", "parent", "path", "level", "security_deposit","created_at") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "id""#,
        ).bind(&m.id)
            .bind(&m.name)
            .bind(&m.parent)
            .bind(&m.path)
            .bind(&m.level)
            .bind(&m.security_deposit)
            .bind(&m.created_at)
            .fetch_one(&self.pool)
            .await.map_err(|e|{
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;
        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 更新
    async fn update(
        &self,
        request: tonic::Request<Category>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let m = model::Category::from(r);
        let rows = query(r#"UPDATE "categories" SET "name" = $1, "parent" = $2, "path" = $3, "level" = $4, "security_deposit" = $5  WHERE "id" = $6"#)
            .bind(&m.name)
            .bind(&m.parent)
            .bind(&m.path)
            .bind(&m.level)
            .bind(&m.security_deposit)
            .bind(&m.id)
            .execute(&self.pool)
            .await.map_err(|e|{
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?.rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 删除
    async fn delete(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = query(r#"DELETE FROM "categories" WHERE "id" = $1"#)
            .bind(&r.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 查找单条
    async fn get(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> std::result::Result<tonic::Response<GetCategoryReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "name", "parent", "path", "level", "security_deposit", "created_at" FROM "categories" WHERE "id" ="#,
        );
        q.push_bind(&r.id);

        if let Some(name) = &r.name {
            q.push(r#" AND "name" ILIKE "#)
                .push_bind(format!("%{name}%"));
        }

        if let Some(parent) = &r.parent {
            q.push(r#" AND "parent" = "#).push_bind(parent);
        }

        if let Some(path) = &r.path {
            q.push(r#" AND "path" = "#).push_bind(path);
        }

        let c: Option<model::Category> = q
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;
        let c = match c {
            Some(v) => Some(v.into()),
            None => None,
        };

        Ok(tonic::Response::new(GetCategoryReply { category: c }))
    }
    /// 分页
    async fn list(
        &self,
        request: tonic::Request<ListCategoryRequest>,
    ) -> std::result::Result<tonic::Response<ListCategoryReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "name", "parent", "path", "level", "security_deposit", "created_at" FROM "categories" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "categories" WHERE 1=1"#);

        if let Some(name) = &r.name {
            q.push(r#" AND "name" ILIKE "#)
                .push_bind(format!("%{name}%"));
            qc.push(r#" AND "name" ILIKE "#)
                .push_bind(format!("%{name}%"));
        }

        if let Some(parent) = &r.parent {
            q.push(r#" AND "parent" = "#).push_bind(parent);
            qc.push(r#" AND "parent" = "#).push_bind(parent);
        }

        if let Some(path) = &r.path {
            q.push(r#" AND "path" = "#).push_bind(path);
            qc.push(r#" AND "path" = "#).push_bind(path);
        }

        if let Some(level) = &r.level {
            q.push(r#" AND "level" = "#).push_bind(level);
            qc.push(r#" AND "level" = "#).push_bind(level);
        }

        if let Some(security_deposits) = &r.security_deposits {
            q.push(r#" AND "security_deposit" BETWEEN "#)
                .push_bind(&security_deposits.start)
                .push(r#" AND "#)
                .push_bind(&security_deposits.end);
            qc.push(r#" AND "security_deposit" BETWEEN "#)
                .push_bind(&security_deposits.start)
                .push(r#" AND "#)
                .push_bind(&security_deposits.end);
        }

        let order = match &r.order {
            Some(v) => v.as_str(),
            None => "id DESC",
        };

        let pr = match r.pr {
            Some(v) => v,
            None => Default::default(),
        };

        q.push(" ORDER BY ")
            .push_bind(order)
            .push(" LIMIT ")
            .push_bind(pr.page_size() as i32)
            .push(" OFFSET ")
            .push_bind((pr.page_size() * pr.page()) as i32);

        let categories_list: Vec<model::Category> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let categories_list = categories_list
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>();

        Ok(tonic::Response::new(ListCategoryReply {
            paginate: Some(pb::paginate::Paginate {
                total: count as u32,
                page: pr.page(),
                page_size: pr.page_size(),
                total_page: f64::ceil(count as f64 / pr.page_size() as f64) as u32,
            }),
            categories: categories_list,
        }))
    }
    /// 全部
    async fn list_all(
        &self,
        request: tonic::Request<ListAllCategoryRequest>,
    ) -> std::result::Result<tonic::Response<ListAllCategoryReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "name", "parent", "path", "level", "security_deposit", "created_at" FROM "categories" WHERE 1=1"#,
        );

        if let Some(name) = &r.name {
            q.push(r#" AND "name" ILIKE "#)
                .push_bind(format!("%{name}%"));
        }

        if let Some(parent) = &r.parent {
            q.push(r#" AND "parent" = "#).push_bind(parent);
        }

        if let Some(path) = &r.path {
            q.push(r#" AND "path" = "#).push_bind(path);
        }

        if let Some(level) = &r.level {
            q.push(r#" AND "level" = "#).push_bind(level);
        }

        if let Some(security_deposits) = &r.security_deposits {
            q.push(r#" AND "security_deposit" BETWEEN "#)
                .push_bind(&security_deposits.start)
                .push(r#" AND "#)
                .push_bind(&security_deposits.end);
        }

        let order = match &r.order {
            Some(v) => v.as_str(),
            None => "id DESC",
        };

        let limit = match r.limit {
            Some(v) => v,
            None => 300,
        };

        q.push(" ORDER BY ")
            .push_bind(order)
            .push(" LIMIT ")
            .push_bind(limit);

        let categories_list: Vec<model::Category> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let categories_list = categories_list
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>();

        Ok(tonic::Response::new(ListAllCategoryReply {
            categories: categories_list,
        }))
    }
    /// 更新保证金
    async fn update_security_deposit(
        &self,
        request: tonic::Request<UpdateSecurityDepositRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = query(r#"UPDATE "categories" SET "security_deposit" = $1  WHERE "id" = $2"#)
            .bind(r.security_deposit)
            .bind(&r.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
}

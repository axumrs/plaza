use sqlx::{PgPool, PgTransaction, QueryBuilder};
use tonic::async_trait;

use super::super::model;
use crate::pb::{
    self,
    brand::{Brand, GetBrandReply, ListBrandReply, ListBrandRequest},
    req, resp,
};

pub struct BrandSrv {
    pool: PgPool,
}
impl BrandSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl BrandSrv {
    async fn _exist(
        tx: &mut PgTransaction<'_>,
        name: &str,
        id: Option<&str>,
    ) -> sqlx::Result<bool> {
        let mut q = QueryBuilder::new(r#"SELECT COUNT(*) FROM "brands" WHERE "name"="#);
        q.push_bind(name);

        if let Some(id) = id {
            q.push(r#" AND "id" <> "#).push_bind(id);
        }

        let count: i64 = q.build_query_scalar().fetch_one(&mut **tx).await?;
        Ok(count > 0)
    }

    async fn _create(&self, r: model::Brand) -> crate::Result<String> {
        let mut tx = self.pool.begin().await?;

        if Self::_exist(&mut tx, &r.name, None).await? {
            return Err(crate::Error::Custom("品牌已存在"));
        }

        let id = sqlx::query_scalar(r#"INSERT INTO "brands" ("id", "name", "logo", "created_at") VALUES ($1, $2, $3, $4) RETURNING "id""#).bind(&r.id).bind(&r.name).bind(&r.logo).bind(&r.created_at).fetch_one(&mut *tx).await?;
        tx.commit().await?;

        Ok(id)
    }

    async fn _update(&self, r: model::Brand) -> crate::Result<u64> {
        let mut tx = self.pool.begin().await?;

        if Self::_exist(&mut tx, &r.name, Some(&r.id)).await? {
            return Err(crate::Error::Custom("品牌已存在"));
        }

        let rows = match sqlx::query(r#"UPDATE "brands" SET "name"=$1, "logo"=$2 WHERE "id"=$3 "#)
            .bind(&r.name)
            .bind(&r.logo)
            .bind(&r.id)
            .execute(&mut *tx)
            .await
        {
            Ok(v) => v.rows_affected(),
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };

        tx.commit().await?;

        Ok(rows)
    }
}

#[async_trait]
impl pb::brand::brand_service_server::BrandService for BrandSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<Brand>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let r = request.into_inner();
        let r: model::Brand = r.into();
        let id = self._create(r).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 修改
    async fn update(
        &self,
        request: tonic::Request<Brand>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r: model::Brand = request.into_inner().into();
        let rows = self._update(r).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 删除
    async fn delete(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = sqlx::query(r#"DELETE FROM "brands" WHERE "id"=$1 "#)
            .bind(&r.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 查找
    async fn get(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<GetBrandReply>, tonic::Status> {
        let r = request.into_inner();
        let b: Option<model::Brand> = sqlx::query_as(
            r#"SELECT "id", "name", "logo", "created_at" FROM "brands" WHERE "id"=$1"#,
        )
        .bind(&r.id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(GetBrandReply {
            brand: b.map(Into::into),
        }))
    }
    /// 分页列表
    async fn list(
        &self,
        request: tonic::Request<ListBrandRequest>,
    ) -> std::result::Result<tonic::Response<ListBrandReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "name", "logo", "created_at" FROM "brands" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "brands" WHERE 1=1"#);

        if let Some(v) = &r.name {
            q.push(r#" AND "name" ILIKE "#).push_bind(format!("%{v}%"));
            qc.push(r#" AND "name" ILIKE "#).push_bind(format!("%{v}%"));
        }

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?;

        let pr = r.pr.unwrap_or_default();

        q.push(" ORDER BY id DESC")
            .push(" LIMIT ")
            .push_bind(pr.page_size() as i32)
            .push(" OFFSET ")
            .push_bind((pr.page() * pr.page_size()) as i32);

        let b: Vec<model::Brand> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?;

        let pa = pb::paginate::Paginate {
            total: count as u32,
            page: pr.page(),
            page_size: pr.page_size(),
            total_page: f64::ceil((count as f64) / (pr.page_size() as f64)) as u32,
        };

        Ok(tonic::Response::new(ListBrandReply {
            paginate: Some(pa),
            brands: b.into_iter().map(Into::into).collect(),
        }))
    }
}

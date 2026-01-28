use sqlx::{PgPool, PgTransaction, QueryBuilder};
use tonic::async_trait;

use super::super::model;
use crate::{
    audit,
    pb::{
        self, req, resp,
        shop::{
            ExistsReply, ExistsRequest, GetReply, GetRequest, ListReply, ListRequest, Shop,
            ShopAudit, UpdateDepositRequest, UpdateStatusRequest,
        },
    },
    utils,
};

pub struct ShopSrv {
    pool: PgPool,
}

impl ShopSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn _exist(
        tx: &mut PgTransaction<'_>,
        name: &str,
        id: Option<&str>,
    ) -> sqlx::Result<bool> {
        let mut q = QueryBuilder::new(r#"SELECT COUNT(*) FROM "shops" WHERE "name"="#);
        q.push_bind(name);

        if let Some(id) = id {
            q.push(r#" AND "id" <> "#).push_bind(id);
        }

        let c: i64 = q.build_query_scalar().fetch_one(&mut **tx).await?;
        Ok(c > 0)
    }
    async fn _create(&self, s: model::Shop) -> crate::Result<String> {
        let mut tx = self.pool.begin().await?;

        if Self::_exist(&mut tx, &s.name, None).await? {
            tx.rollback().await?;
            return Err(crate::Error::Custom("同名店铺已存在"));
        }

        let mut q = QueryBuilder::new(
            r#"INSERT INTO "shops" ("id", "merchant_id", "category_id", "deposit", "name", "kind", "description", "created_at", "status", "meta", "is_platform_self_operated") "#,
        );
        q.push_values(&[&s], |mut b, s| {
            b.push_bind(&s.id)
                .push_bind(&s.merchant_id)
                .push_bind(&s.category_id)
                .push_bind(&s.deposit)
                .push_bind(&s.name)
                .push_bind(&s.kind)
                .push_bind(&s.description)
                .push_bind(&s.created_at)
                .push_bind(&s.status)
                .push_bind(&s.meta)
                .push_bind(&s.is_platform_self_operated);
        });
        q.push(r#" RETURNING "id""#);

        let id: String = match q.build_query_scalar().fetch_one(&mut *tx).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };

        // 审核
        let audit = model::ShopAudit {
            id: utils::id::new(),
            merchant_id: s.merchant_id.clone(),
            shop_id: id.clone(),
            auditor_id: String::new(),
            ..Default::default()
        };

        if let Err(e) = sqlx::query(r#"INSERT INTO "shop_audits" ("id", "merchant_id", "shop_id", "auditor_id") VALUES ($1, $2, $3, $4)"#).bind(&audit.id).bind(&audit.merchant_id).bind(&audit.shop_id).bind(&audit.auditor_id).execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e.into());
        }

        tx.commit().await?;

        Ok(id)
    }

    async fn _update(&self, s: model::Shop) -> crate::Result<u64> {
        let mut tx = self.pool.begin().await?;
        if Self::_exist(&mut tx, &s.name, Some(&s.id)).await? {
            tx.rollback().await?;
            return Err(crate::Error::Custom("同名店铺已存在"));
        }

        let rows = match sqlx::query(r#"UPDATE "shops" SET "merchant_id" = $1, "category_id" = $2, "deposit" = $3, "name" = $4, "kind" = $5, "description" = $6, "status" = $7, "meta" = $8, "is_platform_self_operated" = $9 WHERE "id" = $10"#)
        .bind(&s.merchant_id).bind(&s.category_id).bind(&s.deposit).bind(&s.name).bind(&s.kind).bind(&s.description).bind(&s.status).bind(&s.meta).bind(&s.is_platform_self_operated).bind(&s.id)
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
impl pb::shop::shop_service_server::ShopService for ShopSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<Shop>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let r: model::Shop = request.into_inner().into();
        let id = self._create(r).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 更新
    async fn update(
        &self,
        request: tonic::Request<Shop>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r: model::Shop = request.into_inner().into();

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
        let rows = sqlx::query(r#"DELETE FROM "shops" WHERE id=$1"#)
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
    /// 更新已缴纳的保证金
    async fn update_deposit(
        &self,
        request: tonic::Request<UpdateDepositRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = sqlx::query(r#"UPDATE "shops" SET "deposit"=$1 WHERE "id"=$2"#)
            .bind(&r.deposit)
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
    /// 更新状态
    async fn update_status(
        &self,
        request: tonic::Request<UpdateStatusRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let status: audit::model::AuditStatus = r.status.into();
        let rows = sqlx::query(r#"UPDATE "shops" SET "status"=$1 WHERE "id"=$2"#)
            .bind(&status)
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
    /// 获取单个
    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> std::result::Result<tonic::Response<GetReply>, tonic::Status> {
        let r = request.into_inner();
        let by = match r.by {
            Some(v) => v,
            None => {
                return Err(tonic::Status::invalid_argument("`by` is required"));
            }
        };

        let mut q = QueryBuilder::new(
            r#"SELECT "id", "merchant_id", "category_id", "deposit", "name", "kind", "description", "created_at", "status", "meta", "is_platform_self_operated" FROM "shops" WHERE 1=1"#,
        );

        match by {
            pb::shop::get_request::By::Id(ref id) => {
                q.push(r#" AND "id"="#).push_bind(id);
            }
            pb::shop::get_request::By::Name(ref name) => {
                q.push(r#" AND "name"="#).push_bind(name);
            }
        }

        let s: Option<model::Shop> = q
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(GetReply {
            shop: s.map(|s| s.into()),
        }))
    }
    /// 分页列表
    async fn list(
        &self,
        request: tonic::Request<ListRequest>,
    ) -> std::result::Result<tonic::Response<ListReply>, tonic::Status> {
        let r = request.into_inner();

        let mut q = QueryBuilder::new(
            r#"SELECT "id", "merchant_id", "category_id", "deposit", "name", "kind", "description", "created_at", "status", "meta", "is_platform_self_operated" FROM "shops" WHERE 1=1"#,
        );

        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "shops" WHERE 1=1"#);

        if let Some(v) = r.kind {
            let v: model::ShopKind = v.into();
            q.push(r#" AND "kind"="#).push_bind(v.clone());
            qc.push(r#" AND "kind"="#).push_bind(v);
        }

        if let Some(v) = &r.category_id {
            q.push(r#" AND "category_id"="#).push_bind(v);
            qc.push(r#" AND "category_id"="#).push_bind(v);
        }

        if let Some(v) = &r.merchant_id {
            q.push(r#" AND "merchant_id"="#).push_bind(v);
            qc.push(r#" AND "merchant_id"="#).push_bind(v);
        }

        if let Some(v) = &r.name {
            qc.push(r#" AND "name" ILIKE %"#).push_bind(v).push("%");
            q.push(r#" AND "name" ILIKE %"#).push_bind(v).push("%");
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
        q.push(r#" ORDER BY "id" DESC"#)
            .push(r#" LIMIT "#)
            .push_bind(pr.page_size() as i32)
            .push(r#" OFFSET "#)
            .push_bind((pr.page() * pr.page_size()) as i32);

        let shops: Vec<model::Shop> =
            q.build_query_as()
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
            total_page: f64::ceil(count as f64 / pr.page_size() as f64) as u32,
        };
        Ok(tonic::Response::new(ListReply {
            paginate: Some(pa),
            shops: shops.into_iter().map(|s| s.into()).collect(),
        }))
    }
    /// 是否已存在
    async fn exists(
        &self,
        request: tonic::Request<ExistsRequest>,
    ) -> std::result::Result<tonic::Response<ExistsReply>, tonic::Status> {
        let r = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;
        let exists = Self::_exist(&mut tx, &r.name, r.id.as_deref())
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?;
        tx.commit().await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;
        Ok(tonic::Response::new(ExistsReply { exists }))
    }
    /// 审核
    async fn audit(
        &self,
        request: tonic::Request<ShopAudit>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let sa: model::ShopAudit = request.into_inner().into();

        let rows = sqlx::query(r#"UPDATE "shop_audits" SET "audit_status"=$1,"audit_comments"=$2,"audit_date"=$3,"auditor_id"=$4 WHERE "id"=$5"#)
        .bind(&sa.audit_status).bind(&sa.audit_comments).bind(&sa.audit_date).bind(&sa.auditor_id).bind(&sa.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
}

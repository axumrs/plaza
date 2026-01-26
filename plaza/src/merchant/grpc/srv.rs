use sqlx::{PgPool, QueryBuilder, query, query_as};
use tonic::async_trait;

use crate::{
    audit::model::AuditStatus,
    merchant::model,
    pb::{
        self,
        merchant::{
            AuditMerchantRequest, CreateMerchantRequest, GetMerchantReply, GetMerchantRequest,
            ListMerchantsReply, ListMerchantsRequest, MerchantAddAccountsReply,
            MerchantAddAccountsRequest, MerchantGetAccountReply, MerchantGetAccountRequest,
            MerchantListAccountsReply, MerchantListAccountsRequest, MerchantRemoveAccountsRequest,
            MerchantUpdateAccountPasswordRequest, UpdateAccountRequest, UpdateMerchantRequest,
        },
        req, resp,
    },
    types, utils,
};

pub struct MerchantSrv {
    pool: PgPool,
}

impl MerchantSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MerchantSrv {
    async fn _create(
        &self,
        m: model::Merchant,
        mas: Vec<model::MerchantAccount>,
    ) -> sqlx::Result<String> {
        let mut tx = self.pool.begin().await?;

        // 商家
        let mut mq = QueryBuilder::new(
            r#"INSERT INTO "marchants" ("id", "name", "short_name", "status", "kind", "meta", "created_at") "#,
        );
        mq.push_values(&[&m], |mut b, m| {
            b.push_bind(&m.id)
                .push_bind(&m.name)
                .push_bind(&m.short_name)
                .push_bind(&m.status)
                .push_bind(&m.kind)
                .push_bind(&m.meta)
                .push_bind(&m.created_at);
        });
        mq.push(r#" RETURNING "id""#);

        // 商家账号
        let mut aq = QueryBuilder::new(
            r#"INSERT INTO "marchant_accounts" ("id", "merchant_id", "email", "password", "nickname", "is_super", "permission", "created_at") "#,
        );
        aq.push_values(&mas, |mut b, a| {
            b.push_bind(&a.id)
                .push_bind(&a.merchant_id)
                .push_bind(&a.email)
                .push_bind(&a.password)
                .push_bind(&a.nickname)
                .push_bind(&a.is_super)
                .push_bind(&a.permission)
                .push_bind(&a.created_at);
        });

        // 商家审核
        let mut maq = QueryBuilder::new(
            r#"INSERT INTO "marchant_audits" ("id", "merchant_id", "auditor_id", "audit_status", "audit_comments", "audit_date") "#,
        );
        let audit = model::MerchantAudit {
            id: utils::id::new(),
            merchant_id: m.id.clone(),
            auditor_id: String::new(),
            audit_status: AuditStatus::Pending,
            audit_comments: String::new(),
            audit_date: chrono::Utc::now(),
        };
        maq.push_values(&[&audit], |mut b, a| {
            b.push_bind(&a.id)
                .push_bind(&a.merchant_id)
                .push_bind(&a.auditor_id)
                .push_bind(&a.audit_status)
                .push_bind(&a.audit_comments)
                .push_bind(&a.audit_date);
        });

        let id = match mq.build_query_scalar().fetch_one(&mut *tx).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        if let Err(e) = aq.build().execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e);
        }

        if let Err(e) = maq.build().execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e);
        }

        tx.commit().await?;
        Ok(id)
    }

    async fn _del(&self, id: &str) -> sqlx::Result<u64> {
        let mut tx = self.pool.begin().await?;
        let rows = match query(r#"DELETE FROM "marchants" WHERE id=$1"#)
            .bind(id)
            .execute(&mut *tx)
            .await
        {
            Ok(v) => v.rows_affected(),
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        if let Err(e) = query(r#"DELETE FROM "marchant_accounts" WHERE merchant_id=$1"#)
            .bind(id)
            .execute(&mut *tx)
            .await
        {
            tx.rollback().await?;
            return Err(e);
        }

        if let Err(e) = query(r#"DELETE FROM "marchant_audits" WHERE merchant_id=$1"#)
            .bind(id)
            .execute(&mut *tx)
            .await
        {
            tx.rollback().await?;
            return Err(e);
        }

        tx.commit().await?;

        Ok(rows)
    }

    async fn _audit(&self, r: AuditMerchantRequest) -> sqlx::Result<u64> {
        let mut tx = self.pool.begin().await?;
        let status: AuditStatus = r.audit_status().into();

        let rows = query(r#"UPDATE "marchant_audits" SET audit_status=$1, audit_comments=$2, audit_date=$3, auditor_id=$5 WHERE id=$4"#)
        .bind(&status)
        .bind(&r.audit_comments)
        .bind(chrono::Utc::now())
        .bind(&r.id)
        .bind(&r.auditor_id)
        .execute(&mut *tx).await?.rows_affected();

        if let Err(e) = query(r#"UPDATE "marchants" SET status=$1 WHERE id=$2"#)
            .bind(&status)
            .bind(&r.merchant_id)
            .execute(&mut *tx)
            .await
        {
            tx.rollback().await?;
            return Err(e);
        }
        tx.commit().await?;
        Ok(rows)
    }
}
#[async_trait]
impl pb::merchant::merchant_service_server::MerchantService for MerchantSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<CreateMerchantRequest>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let r = request.into_inner();
        let m = match r.merchant {
            Some(v) => v,
            None => return Err(tonic::Status::invalid_argument("merchant is required")),
        };

        let mas = r.accounts.into_iter().map(|m| m.into()).collect::<Vec<_>>();

        let id = match self._create(m.into(), mas).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("{e}");
                return Err(tonic::Status::internal(e.to_string()));
            }
        };

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 删除
    async fn delete(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();

        let rows = self._del(&r.id).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 审核
    async fn audit(
        &self,
        request: tonic::Request<AuditMerchantRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = self._audit(r).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 修改商户
    async fn update_name(
        &self,
        request: tonic::Request<UpdateMerchantRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let kind: model::MerchantKind = r.kind.into();
        let meta: model::MerchantMeta = match r.meta {
            Some(v) => v.into(),
            None => model::MerchantMeta::default(),
        };

        let m: model::Merchant = model::Merchant {
            id: r.id,
            name: r.name,
            short_name: r.short_name,
            kind,
            meta: sqlx::types::Json(meta),
            ..Default::default()
        };

        let rows = query(r#"UPDATE "merchants" SET "name"=$1, "short_name"=$2, "meta"=$4, "kind"=$5 WHERE "id"=$3"#).bind(&m.name).bind(&m.short_name).bind(&m.id).bind(&m.meta).bind(&m.kind).bind(&m.id).execute(&self.pool).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?.rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取单个商户
    async fn get(
        &self,
        request: tonic::Request<GetMerchantRequest>,
    ) -> std::result::Result<tonic::Response<GetMerchantReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id","name",""short_name","status","kind","created_at","meta" FROM "merchants" WHERE "id"= "#,
        );
        q.push_bind(&r.id);

        if let Some(status) = r.status {
            let status: AuditStatus = status.into();
            q.push(r#" AND "status"= "#);
            q.push_bind(status);
        };

        let m: Option<model::Merchant> = q
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?;

        let merchant = match m {
            Some(v) => Some(v.into()),
            None => None,
        };

        Ok(tonic::Response::new(GetMerchantReply { merchant }))
    }
    /// 商户列表
    async fn list(
        &self,
        request: tonic::Request<ListMerchantsRequest>,
    ) -> std::result::Result<tonic::Response<ListMerchantsReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id","name",""short_name","status","kind","created_at","meta" FROM "merchants" WHERE 1=1 "#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "merchants" WHERE 1=1 "#);

        if let Some(v) = &r.name {
            q.push(r#" AND "name" LIKE "%"#);
            q.push_bind(v);
            q.push(r#"%"#);

            qc.push(r#" AND "name" LIKE "%"#);
            qc.push_bind(v);
            qc.push(r#"%"#);
        }

        if let Some(v) = &r.short_name {
            q.push(r#" AND "short_name" LIKE "%"#);
            q.push_bind(v);
            q.push(r#"%"#);

            qc.push(r#" AND "short_name" LIKE "%"#);
            qc.push_bind(v);
            qc.push(r#"%"#);
        }

        if let Some(v) = r.status {
            let status: AuditStatus = v.into();
            q.push(r#" AND "status"= "#);
            q.push_bind(status);

            let status: AuditStatus = v.into();
            qc.push(r#" AND "status"= "#);
            qc.push_bind(status);
        }

        if let Some(v) = r.kind {
            let kind: model::MerchantKind = v.into();
            q.push(r#" AND "kind"= "#);
            q.push_bind(kind);

            let kind: model::MerchantKind = v.into();
            qc.push(r#" AND "kind"= "#);
            qc.push_bind(kind);
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

        let ms: Vec<model::Merchant> =
            q.build_query_as()
                .fetch_all(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("{e}");
                    tonic::Status::internal(e.to_string())
                })?;

        let merchants = ms.into_iter().map(|m| m.into()).collect::<Vec<_>>();
        let pa = pb::paginate::Paginate {
            total: count as u32,
            page: pr.page(),
            page_size: pr.page_size(),
            total_page: f64::ceil(count as f64 / pr.page_size() as f64) as u32,
        };
        Ok(tonic::Response::new(ListMerchantsReply {
            merchant: merchants,
            paginate: Some(pa),
        }))
    }
    /// 添加账号
    async fn add_accounts(
        &self,
        request: tonic::Request<MerchantAddAccountsRequest>,
    ) -> std::result::Result<tonic::Response<MerchantAddAccountsReply>, tonic::Status> {
        let r = request.into_inner();
        let ids = r.accounts.iter().map(|a| a.id.clone()).collect::<Vec<_>>();

        let mut q = QueryBuilder::new(
            r#"INSERT INTO "merchant_accounts" ("id","merchant_id","email","password","nickname","is_super","permission","created_at") "#,
        );
        q.push_values(&r.accounts, |mut b, a| {
            b.push_bind(&a.id)
                .push_bind(&a.merchant_id)
                .push_bind(&a.email)
                .push_bind(&a.password)
                .push_bind(&a.nickname)
                .push_bind(&a.is_super)
                .push_bind(&a.permission)
                .push_bind(types::prost2chrono(&a.created_at));
        });
        q.build().execute(&self.pool).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(MerchantAddAccountsReply { ids }))
    }
    /// 删除账号
    async fn remove_accounts(
        &self,
        request: tonic::Request<MerchantRemoveAccountsRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(r#"DELETE FROM "merchant_accounts" WHERE "merchant_id"= "#);
        q.push_bind(&r.merchant_id);

        q.push(" AND id IN ");
        q.push_tuples(&r.ids, |mut b, id| {
            b.push_bind(id);
        });

        let rows = q
            .build()
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 修改账号密码
    async fn update_account_password(
        &self,
        request: tonic::Request<MerchantUpdateAccountPasswordRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = query(
            r#"UPDATE "merchant_accounts" SET "password"=$1 WHERE "id"=$2" AND "merchant_id"=$3"#,
        )
        .bind(&r.password)
        .bind(&r.id)
        .bind(&r.merchant_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?
        .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 修改账号
    async fn update_account(
        &self,
        request: tonic::Request<UpdateAccountRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();
        let rows = query(r#"UPDATE "merchant_accounts" SET "nickname"=$1, "is_super"=$2, "permission"=$3 WHERE "id"=$4 AND "merchant_id"=$5 "#)
        .bind(&r.nickname).bind(&r.is_super).bind(&r.permission).bind(&r.id).bind(&r.merchant_id).execute(&self.pool).await.map_err(|e| {
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?.rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取单个账号
    async fn get_account(
        &self,
        request: tonic::Request<MerchantGetAccountRequest>,
    ) -> std::result::Result<tonic::Response<MerchantGetAccountReply>, tonic::Status> {
        let r = request.into_inner();
        let m:Option<model::MerchantAccount> = query_as(r#"SELECT "id","merchant_id","email","password","nickname","is_super","permission","created_at" FROM "merchant_accounts" WHERE "id"=$1 AND "merchant_id"=$2"#)
        .bind(&r.id).bind(&r.merchant_id).fetch_optional(&self.pool).await.map_err(|e|{
            tracing::error!("{e}");
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(MerchantGetAccountReply {
            account: m.map(|v| v.into()),
        }))
    }
    /// 账号列表
    async fn list_accounts(
        &self,
        request: tonic::Request<MerchantListAccountsRequest>,
    ) -> std::result::Result<tonic::Response<MerchantListAccountsReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id","merchant_id","email","password","nickname","is_super","permission","created_at" FROM "merchant_accounts" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "merchant_accounts" WHERE 1=1 "#);

        if let Some(v) = &r.merchant_id {
            q.push(r#" AND "merchant_id" ="#).push_bind(v);
            qc.push(r#" AND "merchant_id" ="#).push_bind(v);
        }

        if let Some(v) = &r.email {
            q.push(r#" AND "email" LIKE "%"#);
            q.push_bind(v);
            q.push(r#"%"#);

            qc.push(r#" AND "email" LIKE "%"#);
            qc.push_bind(v);
            qc.push(r#"%"#);
        }

        if let Some(v) = &r.nickname {
            q.push(r#" AND "nickname" LIKE "%"#);
            q.push_bind(v);
            q.push(r#"%"#);

            qc.push(r#" AND "nickname" LIKE "%"#);
            qc.push_bind(v);
            qc.push(r#"%"#);
        }

        if let Some(v) = &r.is_super {
            q.push(r#" AND "is_super" ="#).push_bind(v);
            qc.push(r#" AND "is_super" ="#).push_bind(v);
        }

        if let Some(v) = &r.permission {
            q.push(r#" AND "permission" ="#).push_bind(v);
            qc.push(r#" AND "permission" ="#).push_bind(v);
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

        let ms: Vec<model::MerchantAccount> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                tonic::Status::internal(e.to_string())
            })?;

        let pa = pb::paginate::Paginate {
            page: pr.page(),
            page_size: pr.page_size(),
            total: count as u32,
            total_page: (count as f64 / pr.page_size() as f64).ceil() as u32,
        };

        Ok(tonic::Response::new(MerchantListAccountsReply {
            paginate: Some(pa),
            accounts: ms.into_iter().map(|v| v.into()).collect(),
        }))
    }
}

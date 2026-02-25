use sqlx::{PgPool, QueryBuilder, query, query_as};
use tonic::async_trait;

use crate::{
    invoice::model,
    pb::{
        self,
        invoice::{GetReply, Invoice, ListReply, ListRequest, UpdateStatusRequest},
        paginate::Paginate,
        req, resp,
    },
    types,
};

pub struct InvoiceSrv {
    pool: PgPool,
}

impl InvoiceSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl pb::invoice::invoice_service_server::InvoiceService for InvoiceSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<Invoice>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let r: model::Invoice = request.into_inner().into();
        let sql = r#"INSERT INTO "invoices" ("id", "order_id", "user_id", "shop_id", "amount", "pay_amount", "status", "kind", "pay_tx_id", "remark", "due_date", "paid_at", "created_at", "updated_at") "#;
        let mut q = QueryBuilder::new(sql);
        q.push_values(&[&r], |mut b, m| {
            b.push_bind(&m.id)
                .push_bind(&m.order_id)
                .push_bind(&m.user_id)
                .push_bind(&m.shop_id)
                .push_bind(&m.amount)
                .push_bind(&m.pay_amount)
                .push_bind(&m.status)
                .push_bind(&m.kind)
                .push_bind(&m.pay_tx_id)
                .push_bind(&m.remark)
                .push_bind(&m.due_date)
                .push_bind(&m.paid_at)
                .push_bind(&m.created_at)
                .push_bind(&m.updated_at);
        });
        q.push(" RETURNING id");

        let id = q
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("create invoice error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 获取单条
    async fn get(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<GetReply>, tonic::Status> {
        let id = request.into_inner().id;
        let sql = r#"SELECT "id", "order_id", "user_id", "shop_id", "amount", "pay_amount", "status", "kind", "pay_tx_id", "remark", "due_date", "paid_at", "created_at", "updated_at" FROM "invoices" WHERE "id" = $1"#;
        let invoice: Option<model::Invoice> = query_as(sql)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("get invoice error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(GetReply {
            invoice: invoice.map(Into::into),
        }))
    }
    /// 列表
    async fn list(
        &self,
        request: tonic::Request<ListRequest>,
    ) -> std::result::Result<tonic::Response<ListReply>, tonic::Status> {
        let r = request.into_inner();
        let (page, page_size) = match r.pr {
            Some(v) => (v.page(), v.page_size()),
            None => (0, 30),
        };
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "order_id", "user_id", "shop_id", "amount", "pay_amount", "status", "kind", "pay_tx_id", "remark", "due_date", "paid_at", "created_at", "updated_at" FROM "invoices" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "invoices" WHERE 1=1"#);

        if let Some(v) = &r.user_id {
            q.push(r#" AND "user_id" = "#).push_bind(v);
            qc.push(r#" AND "user_id" = "#).push_bind(v);
        }

        if let Some(v) = &r.order_id {
            q.push(r#" AND "order_id" = "#).push_bind(v);
            qc.push(r#" AND "order_id" = "#).push_bind(v);
        }

        if let Some(v) = &r.shop_id {
            q.push(r#" AND "shop_id" = "#).push_bind(v);
            qc.push(r#" AND "shop_id" = "#).push_bind(v);
        }

        if let Some(v) = r.status {
            let kind: model::InvoiceKind = v.into();
            q.push(r#" AND "status" = "#).push_bind(kind);
            qc.push(r#" AND "status" = "#).push_bind(kind);
        }

        if let Some(v) = &r.kind {
            q.push(r#" AND "kind" = "#).push_bind(v);
            qc.push(r#" AND "kind" = "#).push_bind(v);
        }

        if let Some(v) = &r.pay_tx_id {
            q.push(r#" AND "pay_tx_id" ILIKE "#)
                .push_bind(format!("%{}%", v));
            qc.push(r#" AND "pay_tx_id" ILIKE "#)
                .push_bind(format!("%{}%", v));
        }

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list invoice error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        q.push(" ORDER BY id DESC")
            .push(" LIMIT ")
            .push_bind(page_size as i32)
            .push(" OFFSET ")
            .push_bind((page * page_size) as i32);

        let invoice_list: Vec<model::Invoice> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list invoice error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let p = Paginate {
            total: count as u32,
            page,
            page_size,
            total_page: f64::ceil(count as f64 / page_size as f64) as u32,
        };

        Ok(tonic::Response::new(ListReply {
            invoices: invoice_list.into_iter().map(Into::into).collect(),
            paginate: Some(p),
        }))
    }
    /// 修改状态
    async fn update_status(
        &self,
        request: tonic::Request<UpdateStatusRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let UpdateStatusRequest { id, status } = request.into_inner();
        let sql = r#"UPDATE "invoices" SET "status" = $2 WHERE "id" = $1"#;
        let rows = query(sql)
            .bind(&id)
            .bind(&status)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("update invoice status error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 完成支付
    async fn complete_paid(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let id = request.into_inner().id;
        let sql = r#"UPDATE "invoices" SET "paid_at" = $2, "status" = 'Paid' WHERE "id" = $1"#;
        let rows = query(sql)
            .bind(&id)
            .bind(types::chrono_now())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("complete invoice error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
}

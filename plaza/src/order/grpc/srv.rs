use sqlx::{PgPool, QueryBuilder, query, query_as};
use tonic::async_trait;

use crate::{
    order::model,
    pb::{
        self,
        order::{
            CompletePayRequest, GetReply, ListReply, ListRequest, Order, UpdateAddressRequest,
            UpdateAmountRequest, UpdateStatusRequest,
        },
        paginate::Paginate,
        req, resp,
    },
    types,
};

pub struct OrderSrv {
    pool: PgPool,
}

impl OrderSrv {
    pub fn new(pool: PgPool) -> Self {
        OrderSrv { pool }
    }
}

#[async_trait]
impl pb::order::order_service_server::OrderService for OrderSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<Order>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let o: model::Order = request.into_inner().into();
        let sql = r#"INSERT INTO "orders" ("id", "user_id", "shop_id", "total_amount", "pay_amount", "freight_amount", "status", "pay_method", "consignee_name", "consignee_phone", "consignee_address", "items", "cart_items", "pay_at", "ship_at", "completed_at", "created_at")"#;
        let mut q = QueryBuilder::new(sql);
        q.push_values(&[&o], |mut b, o| {
            b.push_bind(&o.id)
                .push_bind(&o.user_id)
                .push_bind(&o.shop_id)
                .push_bind(&o.total_amount)
                .push_bind(&o.pay_amount)
                .push_bind(&o.freight_amount)
                .push_bind(&o.status)
                .push_bind(&o.pay_method)
                .push_bind(&o.consignee_name)
                .push_bind(&o.consignee_phone)
                .push_bind(&o.consignee_address)
                .push_bind(&o.items)
                .push_bind(&o.cart_items)
                .push_bind(&o.pay_at)
                .push_bind(&o.ship_at)
                .push_bind(&o.completed_at)
                .push_bind(&o.created_at);
        });
        q.push(" RETURNING id");
        let id = q
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("create order error: {}", e);
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
        let sql = r#"SELECT "id", "user_id", "shop_id", "total_amount", "pay_amount", "freight_amount", "status", "pay_method", "consignee_name", "consignee_phone", "consignee_address", "items", "cart_items", "pay_at", "ship_at", "completed_at", "created_at" FROM "orders" WHERE "id" = $1 LIMIT 1"#;
        let order: Option<model::Order> = query_as(sql)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("get order error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(GetReply {
            order: order.map(Into::into),
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
            r#"SELECT "id", "user_id", "shop_id", "total_amount", "pay_amount", "freight_amount", "status", "pay_method", "consignee_name", "consignee_phone", "consignee_address", "items", "cart_items", "pay_at", "ship_at", "completed_at", "created_at" FROM "orders" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "orders" WHERE 1=1"#);

        if let Some(v) = &r.user_id {
            q.push(r#" AND "user_id" = "#).push_bind(v);
            qc.push(r#" AND "user_id" = "#).push_bind(v);
        }

        if let Some(v) = &r.shop_id {
            q.push(r#" AND "shop_id" = "#).push_bind(v);
            qc.push(r#" AND "shop_id" = "#).push_bind(v);
        }

        if let Some(v) = r.status {
            let v: model::OrderStatus = v.into();
            q.push(r#" AND "status" = "#).push_bind(v);
            qc.push(r#" AND "status" = "#).push_bind(v);
        }

        if let Some(v) = &r.consignee_name {
            q.push(r#" AND "consignee_name" ILIKE "#)
                .push_bind(format!("%{}%", v));
            qc.push(r#" AND "consignee_name" ILIKE "#)
                .push_bind(format!("%{}%", v));
        }

        if let Some(v) = &r.consignee_phone {
            q.push(r#" AND "consignee_phone" ILIKE "#)
                .push_bind(format!("%{}%", v));
            qc.push(r#" AND "consignee_phone" ILIKE "#)
                .push_bind(format!("%{}%", v));
        }

        if let Some(v) = &r.consignee_address {
            q.push(r#" AND "consignee_address" ILIKE "#)
                .push_bind(format!("%{}%", v));
            qc.push(r#" AND "consignee_address" ILIKE "#)
                .push_bind(format!("%{}%", v));
        }

        if let Some(v) = &r.goods_id {
            q.push(r#" AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE (item).goods_id ="#)
                .push_bind(v)
                .push(")");
            qc.push(r#" AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE (item).goods_id ="#)
                .push_bind(v)
                .push(")");
        }

        if let Some(v) = &r.goods_name {
            q.push(r#"AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE (item).name ILIKE "#)
                .push_bind(format!("%{v}%"))
                .push(")");
            qc.push(r#"AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE (item).name ILIKE "#)
                .push_bind(format!("%{v}%"))
                .push(")");
        }

        if let Some(v) = &r.sku_id {
            q.push(r#" AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE (item).sku_id = "#)
                .push_bind(v)
                .push(")");
            qc.push(r#" AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE (item).sku_id = "#)
                .push_bind(v)
                .push(")");
        }

        if let Some(v) = &r.sku_name {
            q.push(r#" AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE "#)
                .push_bind(v)
                .push("= ANY ((item).sku_arr))");
            qc.push(r#" AND EXISTS (SELECT 1 FROM UNNEST(items) AS item WHERE "#)
                .push_bind(v)
                .push("= ANY ((item).sku_arr))");
        }

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("query count error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        q.push(" ORDER BY id DESC")
            .push(" LIMIT ")
            .push_bind(page_size as i32)
            .push(" OFFSET ")
            .push_bind((page * page_size) as i32);

        let order_list: Vec<model::Order> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("query order list error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let paginate = Paginate {
            total: count as u32,
            page,
            page_size,
            total_page: f64::ceil(count as f64 / page_size as f64) as u32,
        };

        Ok(tonic::Response::new(ListReply {
            orders: order_list.into_iter().map(Into::into).collect(),
            paginate: Some(paginate),
        }))
    }
    /// 更新地址
    async fn update_address(
        &self,
        request: tonic::Request<UpdateAddressRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let UpdateAddressRequest {
            id,
            consignee_name,
            consignee_phone,
            consignee_address,
        } = request.into_inner();
        let sql = r#"UPDATE "orders" SET "consignee_name" = $2, "consignee_phone" = $3, "consignee_address" = $4 WHERE "id" = $1"#;
        let rows = query(sql)
            .bind(&id)
            .bind(&consignee_name)
            .bind(&consignee_phone)
            .bind(&consignee_address)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("update address error: {}", e);
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
        let UpdateStatusRequest { id, status } = request.into_inner();
        let sql = r#"UPDATE "orders" SET "status" = $2 WHERE "id" = $1"#;
        let status: model::OrderStatus = status.into();
        let rows = query(sql)
            .bind(&id)
            .bind(&status)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("update status error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 更新价格
    async fn update_amount(
        &self,
        request: tonic::Request<UpdateAmountRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let UpdateAmountRequest {
            id,
            total_amount,
            freight_amount,
        } = request.into_inner();
        let sql =
            r#"UPDATE "orders" SET "total_amount" = $2, "freight_amount" = $3 WHERE "id" = $1"#;
        let rows = query(sql)
            .bind(&id)
            .bind(&total_amount)
            .bind(&freight_amount)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("update amount error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 完成支付
    async fn complete_pay(
        &self,
        request: tonic::Request<CompletePayRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let CompletePayRequest { id, pay_amount } = request.into_inner();
        let sql = r#"UPDATE "orders" SET "pay_amount" = $2, "pay_at" = $3, "status" = 'PendingShipment' WHERE "id" = $1"#;
        let rows = query(sql)
            .bind(&id)
            .bind(&pay_amount)
            .bind(types::chrono_now())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("complete pay error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
}

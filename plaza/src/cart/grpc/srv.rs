use super::super::model;
use sqlx::{PgPool, query, query_as};
use tonic::async_trait;

use crate::pb::{
    self,
    cart::{Cart, CartMetaRequest, GetReply},
    resp,
};

pub struct CartSrv {
    pool: PgPool,
}

impl CartSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl pb::cart::cart_service_server::CartService for CartSrv {
    /// 同步
    async fn sync(
        &self,
        request: tonic::Request<Cart>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let cart: model::Cart = request.into_inner().into();
        let sql = r#"INSERT INTO "carts" ("user_id", "shop_id", "items", "created_at") VALUES ($1, $2, $3, $4) ON CONFLICT ("user_id", "shop_id") DO UPDATE SET "items"=EXCLUDED."items"#;
        let rows = query(sql)
            .bind(&cart.user_id)
            .bind(&cart.shop_id)
            .bind(&cart.items)
            .bind(&cart.created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("sync cart error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取
    async fn get(
        &self,
        request: tonic::Request<CartMetaRequest>,
    ) -> std::result::Result<tonic::Response<GetReply>, tonic::Status> {
        let CartMetaRequest { user_id, shop_id } = request.into_inner();
        let sql = r#"SELECT "user_id", "shop_id", "items", "created_at" FROM "carts" WHERE "user_id"=$1 AND "shop_id"=$2"#;
        let cart: Option<model::Cart> = query_as(sql)
            .bind(&user_id)
            .bind(&shop_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("get cart error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(GetReply {
            cart: cart.map(Into::into),
        }))
    }
    /// 清空
    async fn clean(
        &self,
        request: tonic::Request<CartMetaRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let CartMetaRequest { user_id, shop_id } = request.into_inner();
        let sql = r#"DELETE FROM "carts" WHERE "user_id"=$1 AND "shop_id"=$2"#;
        let rows = query(sql)
            .bind(&user_id)
            .bind(&shop_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("clean cart error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
}

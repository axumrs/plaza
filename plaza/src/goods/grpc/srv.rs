use sqlx::{PgPool, QueryBuilder, query, query_scalar};
use tonic::async_trait;

use crate::{
    goods::model,
    pb::{
        self,
        goods::{
            CalcStarReply, DecrementStockRequest, GetCommentReply, GetCommentRequest, GetReply,
            GetSkuReply, GoodsComment, GoodsInfo, ListCommentsReply, ListCommentsRequest,
            ListReply, ListRequest, UpdateRequest, UpdateStockRequest,
        },
        req, resp,
    },
    types,
};

pub struct GoodsSrv {
    pool: PgPool,
}

impl GoodsSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn _create(
        &self,
        goods: model::Goods,
        metas: Vec<model::GoodsSkuMeta>,
        attrs: Vec<model::GoodsAttr>,
    ) -> crate::Result<String> {
        let sql = r#"INSERT INTO "goods" ("id", "shop_id", "is_vir", "category_id", "name", "images", "status", "detail", "comment_need_audit", "service_guarantee", "tags", "arguments", "stock", "sales", "sku_meta", "fare", "recommendations", "created_at", "updated_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) RETURNING "id""#;
        let mut tx = self.pool.begin().await?;
        let id: String = match query_scalar(sql)
            .bind(&goods.id)
            .bind(&goods.shop_id)
            .bind(&goods.is_vir)
            .bind(&goods.category_id)
            .bind(&goods.name)
            .bind(&goods.images)
            .bind(&goods.status)
            .bind(&goods.detail)
            .bind(&goods.comment_need_audit)
            .bind(&goods.service_guarantee)
            .bind(&goods.tags)
            .bind(&goods.arguments)
            .bind(&goods.stock)
            .bind(&goods.sales)
            .bind(&metas)
            .bind(&goods.fare)
            .bind(&goods.recommendations)
            .bind(&goods.created_at)
            .bind(&goods.updated_at)
            .fetch_one(&mut *tx)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };

        let sql = r#"INSERT INTO "goods_attrs" ("id", "goods_id", "sku_arr", "stock", "price", "sales", "code", "bar_code", "volume", "weight", "ver") "#;
        let mut q = QueryBuilder::new(sql);
        q.push_values(&attrs, |mut b, a| {
            b.push_bind(&a.id).push_bind(&a.goods_id)
                .push_bind(&a.sku_arr)
                .push_bind(&a.stock)
                .push_bind(&a.price)
                .push_bind(&a.sales)
                .push_bind(&a.code)
                .push_bind(&a.bar_code)
                .push_bind(&a.volume)
                .push_bind(&a.weight)
                .push_bind(0) // ver
                ;
        });

        if let Err(e) = q.build().execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e.into());
        }

        tx.commit().await?;

        Ok(id)
    }

    async fn _update(
        &self,
        goods: model::Goods,
        metas: Vec<model::GoodsSkuMeta>,
        attrs: Vec<model::GoodsAttr>,
    ) -> crate::Result<u64> {
        let mut tx = self.pool.begin().await?;

        let now = types::chrono_now();
        let sql = r#"UPDATE "goods" SET "name" = $2, "images" = $3, "detail" = $4, "sku_meta" = $5, "updated_at" = $6, "status" = $7, "stock" = $8, "sales" = $9, "fare" = $10, "recommendations" = $11, "tags" = $12, "arguments" = $13, "service_guarantee" = $14, "comment_need_audit" = $15, "category_id" = $16 WHERE "id" = $1"#;
        let rows = match query(sql)
            .bind(&goods.id)
            .bind(&goods.name)
            .bind(&goods.images)
            .bind(&goods.detail)
            .bind(&metas)
            .bind(&now)
            .bind(&goods.status)
            .bind(&goods.stock)
            .bind(&goods.sales)
            .bind(&goods.fare)
            .bind(&goods.recommendations)
            .bind(&goods.tags)
            .bind(&goods.arguments)
            .bind(&goods.service_guarantee)
            .bind(&goods.comment_need_audit)
            .bind(&goods.category_id)
            .execute(&mut *tx)
            .await
        {
            Ok(r) => r.rows_affected(),
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };

        // 删除旧的sku
        let sql = r#"DELETE FROM "goods_attrs" WHERE "goods_id" = $1"#;
        if let Err(e) = query(sql).bind(&goods.id).execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e.into());
        }

        // 添加新的sku
        let sql = r#"INSERT INTO "goods_attrs" ("id", "goods_id", "sku_arr", "stock", "price", "sales", "code", "bar_code", "volume", "weight", "ver") "#;
        let mut q = QueryBuilder::new(sql);
        q.push_values(&attrs, |mut b, a| {
            b.push_bind(&a.id).push_bind(&a.goods_id)
                .push_bind(&a.sku_arr)
                .push_bind(&a.stock)
                .push_bind(&a.price)
                .push_bind(&a.sales)
                .push_bind(&a.code)
                .push_bind(&a.bar_code)
                .push_bind(&a.volume)
                .push_bind(&a.weight)
                .push_bind(0) // ver
                ;
        });

        if let Err(e) = q.build().execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e.into());
        }

        tx.commit().await?;

        Ok(rows)
    }

    async fn _del(&self, id: &str) -> crate::Result<u64> {
        let mut tx = self.pool.begin().await?;

        let sql = r#"DELETE FROM "goods" WHERE "id" = $1"#;
        let rows = match query(sql).bind(id).execute(&mut *tx).await {
            Ok(r) => r.rows_affected(),
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };

        let sql = r#"DELETE FROM "goods_attrs" WHERE "goods_id" = $1"#;
        if let Err(e) = query(sql).bind(id).execute(&mut *tx).await {
            tx.rollback().await?;
            return Err(e.into());
        }
        tx.commit().await?;

        Ok(rows)
    }
}

#[async_trait]
impl pb::goods::goods_service_server::GoodsService for GoodsSrv {
    /// 添加商品
    async fn create(
        &self,
        request: tonic::Request<GoodsInfo>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let GoodsInfo {
            goods,
            metas,
            attrs,
        } = request.into_inner();

        let goods: model::Goods = match goods {
            Some(g) => g.into(),
            None => return Err(tonic::Status::invalid_argument("goods is empty")),
        };

        let metas = metas
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<model::GoodsSkuMeta>>();

        let attrs = attrs
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<model::GoodsAttr>>();

        let id = self._create(goods, metas, attrs).await.map_err(|e| {
            tracing::error!("create goods error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 修改商品
    async fn update(
        &self,
        request: tonic::Request<UpdateRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let UpdateRequest {
            goods,
            metas,
            attrs,
        } = request.into_inner();
        let goods: model::Goods = match goods {
            Some(v) => v.into(),
            None => return Err(tonic::Status::invalid_argument("goods is empty")),
        };

        let metas = metas
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<model::GoodsSkuMeta>>();

        let attrs = attrs
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<model::GoodsAttr>>();

        let rows = self._update(goods, metas, attrs).await.map_err(|e| {
            tracing::error!("update goods error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 删除
    async fn delete(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let id = request.into_inner().id;
        let rows = self._del(&id).await.map_err(|e| {
            tracing::error!("delete goods error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 修改库存
    async fn update_stock(
        &self,
        request: tonic::Request<UpdateStockRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 扣减库存
    async fn decrement_stock(
        &self,
        request: tonic::Request<DecrementStockRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 获取SKU
    async fn get_sku(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<GetSkuReply>, tonic::Status> {
        unimplemented!()
    }
    /// 获取单个商品
    async fn get(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<GetReply>, tonic::Status> {
        unimplemented!()
    }
    /// 商品列表
    async fn list(
        &self,
        request: tonic::Request<ListRequest>,
    ) -> std::result::Result<tonic::Response<ListReply>, tonic::Status> {
        unimplemented!()
    }
    /// 添加评价
    async fn create_comment(
        &self,
        request: tonic::Request<GoodsComment>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        unimplemented!()
    }
    /// 计算评分
    async fn calc_star(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<CalcStarReply>, tonic::Status> {
        unimplemented!()
    }
    /// 获取单条评价
    async fn get_comment(
        &self,
        request: tonic::Request<GetCommentRequest>,
    ) -> std::result::Result<tonic::Response<GetCommentReply>, tonic::Status> {
        unimplemented!()
    }
    /// 评价列表
    async fn list_comments(
        &self,
        request: tonic::Request<ListCommentsRequest>,
    ) -> std::result::Result<tonic::Response<ListCommentsReply>, tonic::Status> {
        unimplemented!()
    }
}

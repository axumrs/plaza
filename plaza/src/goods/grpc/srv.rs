use sqlx::{PgPool, QueryBuilder, query, query_as, query_scalar};
use tonic::async_trait;

use crate::{
    Error,
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

    async fn _update_stock(
        &self,
        goods_id: String,
        sku: Vec<String>,
        stock: i64,
    ) -> crate::Result<u64> {
        let sql =
            r#"UPDATE "goods_attrs" SET "stock"=$1, "ver"=0 WHERE "goods_id"=$2 AND "sku_arr"=$3"#; // 强制将版本置为0
        let rows = query(sql)
            .bind(&stock)
            .bind(&goods_id)
            .bind(&sku)
            .execute(&self.pool)
            .await?
            .rows_affected();
        Ok(rows)
    }

    async fn _decrement_stock(
        &self,
        goods_id: String,
        sku: Vec<String>,
        stock: Option<i64>,
    ) -> crate::Result<u64> {
        let mut tx = self.pool.begin().await?;
        let attr: Option<model::GoodsAttr> =
            query_as(r#"SELECT "id","goods_id","sku_arr","stock","price","sales","code","bar_code","volume","weight","ver" FROM "goods_attrs" WHERE "goods_id"=$1 AND "sku_arr"=$2"#)
                .bind(&goods_id)
                .bind(&sku)
                .fetch_optional(&mut *tx)
                .await?;
        let attr = match attr {
            Some(v) => v,
            None => return Err(Error::Custom("不存在的记录")),
        };

        // 更新
        let sql = r#"UPDATE "goods_attrs" SET "stock" = "stock" - $1, "ver" = $4 + 1 WHERE "goods_id" = $2 AND "sku_arr"=$3 AND "ver" = $4"#;
        let stock = stock.unwrap_or(1);

        let rows = match query(sql)
            .bind(&stock)
            .bind(&goods_id)
            .bind(&sku)
            .bind(&attr.ver)
            .execute(&mut *tx)
            .await
        {
            Ok(r) => r.rows_affected(),
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
        let UpdateStockRequest {
            goods_id,
            sku,
            stock,
        } = request.into_inner();
        let rows = self
            ._update_stock(goods_id, sku, stock)
            .await
            .map_err(|e| {
                tracing::error!("update stock error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 扣减库存
    async fn decrement_stock(
        &self,
        request: tonic::Request<DecrementStockRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let DecrementStockRequest {
            goods_id,
            sku,
            stock_value,
        } = request.into_inner();

        let rows = self
            ._decrement_stock(goods_id, sku, stock_value)
            .await
            .map_err(|e| {
                tracing::error!("decrement stock error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取SKU
    async fn get_sku(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<GetSkuReply>, tonic::Status> {
        let id = request.into_inner().id;

        let r: Option<(Vec<model::GoodsSkuMeta>,)> =
            query_as(r#"SELECT "sku_meta" FROM "goods" WHERE "id"=$1"#)
                .bind(&id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("get sku error: {}", e);
                    tonic::Status::internal(e.to_string())
                })?;

        let meta = match r {
            Some(v) => v.0,
            None => return Err(tonic::Status::not_found("不存在的商品")),
        };

        Ok(tonic::Response::new(GetSkuReply {
            goods_id: id,
            sku_meta: meta.into_iter().map(|s| s.into()).collect(),
        }))
    }
    /// 获取单个商品
    async fn get(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<GetReply>, tonic::Status> {
        let id = request.into_inner().id;
        let sql = r#"SELECT "id","shop_id","is_vir","category_id","name","images","status","detail","comment_need_audit","service_guarantee","tags","arguments","stock","sales","sku_meta","fare","recommendations","created_at","updated_at" FROM "goods" WHERE "id"=$1"#;
        let goods: Option<model::Goods> = query_as(sql)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("get goods error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let goods = match goods {
            Some(v) => v,
            None => return Ok(tonic::Response::new(GetReply { goods: None })),
        };
        let metas: Vec<pb::goods::GoodsSkuMeta> = goods
            .sku_meta
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect();

        let attrs:Vec<model::GoodsAttr> = query_as(r#"SELECT "id","goods_id","sku_arr","stock","price","sales","code","bar_code","volume","weight","ver" FROM "goods_attrs" WHERE "goods_id"=$1"#).bind(&id).fetch_all(&self.pool).await.map_err(|e| {
            tracing::error!("get attrs error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        let attrs: Vec<pb::goods::GoodsAttr> = attrs
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<pb::goods::GoodsAttr>>();

        let goods: Option<pb::goods::GoodsInfo> = Some(pb::goods::GoodsInfo {
            attrs,
            goods: Some(goods.into()),
            metas,
        });

        Ok(tonic::Response::new(GetReply { goods }))
    }
    /// 商品列表
    async fn list(
        &self,
        request: tonic::Request<ListRequest>,
    ) -> std::result::Result<tonic::Response<ListReply>, tonic::Status> {
        let r = request.into_inner();
        let (page, page_size) = match r.pr {
            Some(pr) => (pr.page(), pr.page_size()),
            None => (0, 30),
        };

        let mut q = QueryBuilder::new(
            r#"SELECT "id","shop_id","is_vir","category_id","name","images","status","detail","comment_need_audit","service_guarantee","tags","arguments","stock","sales","sku_meta","fare","recommendations","created_at","updated_at" FROM "goods" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "goods" WHERE 1=1"#);

        if let Some(v) = &r.shop_id {
            q.push(r#" AND "shop_id"="#).push_bind(v);
            qc.push(r#" AND "shop_id"="#).push_bind(v);
        }

        if let Some(v) = &r.category_id {
            q.push(r#" AND "category_id"="#).push_bind(v);
            qc.push(r#" AND "category_id"="#).push_bind(v);
        }

        if let Some(v) = &r.is_vir {
            q.push(r#" AND "is_vir"="#).push_bind(v);
            qc.push(r#" AND "is_vir"="#).push_bind(v);
        }

        if let Some(v) = &r.name {
            q.push(r#" AND "name" ILIKE "#).push_bind(format!("%{v}%"));
            qc.push(r#" AND "name" ILIKE "#).push_bind(format!("%{v}%"));
        }

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        q.push(" ORDER BY id DESC")
            .push(" LIMIT ")
            .push_bind(page_size as i32)
            .push(" OFFSET ")
            .push_bind((page * page_size) as i32);

        let goods: Vec<model::Goods> =
            q.build_query_as()
                .fetch_all(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("list error: {}", e);
                    tonic::Status::internal(e.to_string())
                })?;
        if goods.is_empty() {
            return Err(tonic::Status::not_found("没有符合条件的商品"));
        }

        let mut goods_info_list = Vec::with_capacity(goods.len());

        for g in goods.into_iter() {
            let metas: Vec<pb::goods::GoodsSkuMeta> =
                g.sku_meta.clone().into_iter().map(|m| m.into()).collect();

            let attrs:Vec<model::GoodsAttr> = query_as(r#"SELECT "id","goods_id","sku_arr","stock","price","sales","code","bar_code","volume","weight","ver" FROM "goods_attrs" WHERE "goods_id"=$1"#).bind(&g.id).fetch_all(&self.pool).await.map_err(|e| {
            tracing::error!("get attrs error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;
            goods_info_list.push(pb::goods::GoodsInfo {
                goods: Some(g.into()),
                metas,
                attrs: attrs.into_iter().map(|a| a.into()).collect(),
            })
        }
        let paginate = pb::paginate::Paginate {
            total: count as u32,
            page,
            page_size,
            total_page: (count as f64 / page_size as f64).ceil() as u32,
        };

        Ok(tonic::Response::new(ListReply {
            paginate: Some(paginate),
            goods: goods_info_list,
        }))
    }
    /// 添加评价
    async fn create_comment(
        &self,
        request: tonic::Request<GoodsComment>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let goods_comment: model::GoodsComment = request.into_inner().into();

        let sql = r#"INSERT INTO "goods_comments" ("id", "goods_id", "goods_full_name", "is_self", "user_id", "user_avatar", "user_nickname", "content", "goods_star", "service_star", "images", "created_at") "#;

        let mut q = QueryBuilder::new(sql);

        q.push_values(&[&goods_comment], |mut b, m| {
            b.push_bind(&m.id)
                .push_bind(&m.goods_id)
                .push_bind(&m.goods_full_name)
                .push_bind(&m.is_self)
                .push_bind(&m.user_id)
                .push_bind(&m.user_avatar)
                .push_bind(&m.user_nickname)
                .push_bind(&m.content)
                .push_bind(&m.goods_star)
                .push_bind(&m.service_star)
                .push_bind(&m.images)
                .push_bind(&m.created_at);
        });

        q.push(" RETURNING id");

        let id: String = q
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("create_comment error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 计算评分
    async fn calc_star(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<CalcStarReply>, tonic::Status> {
        let id = request.into_inner().id;
        let sql = r#"
            SELECT COUNT(*) AS "value", 'positive' AS "name" FROM "goods_comments" WHERE "goods_star" = 5 AND "service_star" = 5 AND "goods_id" = $1
            UNION ALL
            SELECT COUNT(*) AS "value", 'neutral' AS "name" FROM "goods_comments" WHERE ("goods_star" BETWEEN 3 AND 4 ) AND ("service_star" BETWEEN 3 AND 4) AND "goods_id" = $1
            UNION ALL
            SELECT COUNT(*) AS "value", 'negative' AS "name" FROM "goods_comments" WHERE ("goods_star" BETWEEN 1 AND 2 ) AND ("service_star" BETWEEN 1 AND 2) AND "goods_id" = $1
            UNION ALL
            SELECT COUNT(*) AS "value", 'all' AS "name" FROM "goods_comments" WHERE "goods_id" = $1
        "#;
        let r: Vec<(i64, String)> = query_as(sql)
            .bind(&id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("calc_star error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;
        let positive_count = r.iter().find(|v| v.1 == "positive").unwrap().0;
        let all_count = r.iter().find(|v| v.1 == "all").unwrap().0;
        let rate = ((positive_count as f64) / (all_count as f64)) * 100.0;

        Ok(tonic::Response::new(CalcStarReply { goods_id: id, rate }))
    }

    /// 获取单条评价
    async fn get_comment(
        &self,
        request: tonic::Request<GetCommentRequest>,
    ) -> std::result::Result<tonic::Response<GetCommentReply>, tonic::Status> {
        let r = request.into_inner();
        let sql = r#"SELECT "id", "goods_id", "goods_full_name", "is_self", "user_id", "user_avatar", "user_nickname", "content", "goods_star", "service_star", "images", "created_at" FROM "goods_comments" WHERE "id"=$1 AND "goods_id"=$2"#;

        let comment: Option<model::GoodsComment> = query_as(sql)
            .bind(&r.id)
            .bind(&r.goods_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("get_comment error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let comment = match comment {
            Some(v) => Some(v.into()),
            None => return Err(tonic::Status::not_found("不存在的评价")),
        };

        Ok(tonic::Response::new(GetCommentReply { comment }))
    }
    /// 评价列表
    async fn list_comments(
        &self,
        request: tonic::Request<ListCommentsRequest>,
    ) -> std::result::Result<tonic::Response<ListCommentsReply>, tonic::Status> {
        let r = request.into_inner();
        let (page, page_size) = match r.pr {
            Some(v) => (v.page(), v.page_size()),
            None => (0, 30),
        };

        let sql = r#"SELECT "id", "goods_id", "goods_full_name", "is_self", "user_id", "user_avatar", "user_nickname", "content", "goods_star", "service_star", "images", "created_at" FROM "goods_comments" WHERE  "goods_id"= "#;
        let count_sql = r#"SELECT COUNT(*) FROM "goods_comments" WHERE  "goods_id"="#;

        let mut q = QueryBuilder::new(sql);
        let mut qc = QueryBuilder::new(count_sql);
        q.push_bind(&r.goods_id);
        qc.push_bind(&r.goods_id);

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list_comments error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        q.push(" ORDER BY id DESC")
            .push(" LIMIT ")
            .push_bind(page_size as i32)
            .push(" OFFSET ")
            .push_bind((page * page_size) as i32);

        let comments: Vec<model::GoodsComment> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list_comments error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let paginate = pb::paginate::Paginate {
            total: count as u32,
            page,
            page_size,
            total_page: (count as f64 / page_size as f64).ceil() as u32,
        };

        Ok(tonic::Response::new(ListCommentsReply {
            paginate: Some(paginate),
            comments: comments.into_iter().map(|c| c.into()).collect(),
        }))
    }
}

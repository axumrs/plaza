use sqlx::{PgPool, QueryBuilder, query, query_as, query_scalar};
use tonic::async_trait;

use crate::{
    address::model,
    pb::{
        self,
        address::{
            Address, DeleteRequest, GetDefaultRequest, GetRequest, ListReply, ListRequest,
            OptionAddressReply, SetDefaultRequest,
        },
        resp,
    },
};

pub struct AddressSrv {
    pool: PgPool,
}

impl AddressSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn _set_default(&self, id: String, user_id: String) -> crate::Result<u64> {
        let mut tx = self.pool.begin().await?;
        // 将所有地址设置为非默认
        let rows = match query(r#"UPDATE "address" SET "is_default" = false WHERE "user_id" = $1"#)
            .bind(&user_id)
            .execute(&mut *tx)
            .await
        {
            Ok(r) => r.rows_affected(),
            Err(e) => {
                tx.rollback().await?;
                return Err(e.into());
            }
        };
        // 将指定地址设置为默认
        if let Err(e) =
            query(r#"UPDATE "address" SET "is_default" = true WHERE "id" = $1 AND "user_id" = $2"#)
                .bind(&id)
                .bind(&user_id)
                .execute(&mut *tx)
                .await
        {
            tx.rollback().await?;
            return Err(e.into());
        }
        tx.commit().await?;
        Ok(rows)
    }
}

#[async_trait]
impl pb::address::address_service_server::AddressService for AddressSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<Address>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let m: model::Address = request.into_inner().into();
        let sql = r#"INSERT INTO "address" ("id", "user_id", "is_default", "consignee", "phone", "address", "province", "city", "county", "post_code", "created_at") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING "id""#;

        let id: String = query_scalar(sql)
            .bind(&m.id)
            .bind(&m.user_id)
            .bind(&m.is_default)
            .bind(&m.consignee)
            .bind(&m.phone)
            .bind(&m.address)
            .bind(&m.province)
            .bind(&m.city)
            .bind(&m.county)
            .bind(&m.post_code)
            .bind(&m.created_at)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("create address error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 修改
    async fn update(
        &self,
        request: tonic::Request<Address>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let m: model::Address = request.into_inner().into();

        if m.id.is_empty() {
            return Err(tonic::Status::invalid_argument("id is empty"));
        }

        let sql = r#"UPDATE "address" SET "is_default" = $1, "consignee" = $2, "phone" = $3, "address" = $4, "province" = $5, "city" = $6, "county" = $7, "post_code" = $8 WHERE "id" = $9"#;
        let rows = query(sql)
            .bind(&m.is_default)
            .bind(&m.consignee)
            .bind(&m.phone)
            .bind(&m.address)
            .bind(&m.province)
            .bind(&m.city)
            .bind(&m.county)
            .bind(&m.post_code)
            .bind(&m.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("update address error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();
        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 删除
    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let DeleteRequest { id, user_id } = request.into_inner();
        let sql = r#"DELETE FROM "address" WHERE "id" ="#;
        let mut q = QueryBuilder::new(sql);
        q.push_bind(&id);

        if let Some(user_id) = &user_id {
            q.push(r#" AND "user_id" ="#);
            q.push_bind(user_id);
        }

        let rows = q
            .build()
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("delete address error: {}", e);
                tonic::Status::internal(e.to_string())
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取单个地址
    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> std::result::Result<tonic::Response<OptionAddressReply>, tonic::Status> {
        let GetRequest { id, user_id } = request.into_inner();
        let sql = r#"SELECT "id", "user_id", "is_default", "consignee", "phone", "address", "province", "city", "county", "post_code", "created_at" FROM "address" WHERE "id" ="#;
        let mut q = QueryBuilder::new(sql);
        q.push_bind(id);

        if let Some(user_id) = &user_id {
            q.push(r#" AND "user_id" ="#);
            q.push_bind(user_id);
        }

        q.push(" LIMIT 1");

        let address: Option<model::Address> = q
            .build_query_as()
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
            tracing::error!("get address error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(OptionAddressReply {
            address: address.map(Into::into),
        }))
    }
    /// 地址列表
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
            r#"SELECT "id", "user_id", "is_default", "consignee", "phone", "address", "province", "city", "county", "post_code", "created_at" FROM "address" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "address" WHERE 1=1"#);

        if let Some(user_id) = &r.user_id {
            q.push(r#" AND "user_id" = "#);
            q.push_bind(user_id);
            qc.push(r#" AND "user_id" = "#);
            qc.push_bind(user_id);
        }

        if let Some(consignee) = &r.consignee {
            q.push(r#" AND "consignee" ILIKE "#)
                .push_bind(format!("%{consignee}%"));
            qc.push(r#" AND "consignee" ILIKE "#)
                .push_bind(format!("%{consignee}%"));
        }

        if let Some(phone) = &r.phone {
            q.push(r#" AND "phone" ILIKE "#)
                .push_bind(format!("%{phone}%"));
            qc.push(r#" AND "phone" ILIKE "#)
                .push_bind(format!("%{phone}%"));
        }

        if let Some(addr) = &r.address {
            q.push(r#" AND "address" ILIKE "#)
                .push_bind(format!("%{addr}%"));
            qc.push(r#" AND "address" ILIKE "#)
                .push_bind(format!("%{addr}%"));
        }

        let count: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list address error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        q.push(" ORDER BY id DESC")
            .push(" LIMIT ")
            .push_bind(page_size as i32)
            .push(" OFFSET ")
            .push_bind((page * page_size) as i32);

        let address_list: Vec<model::Address> = q
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("list address error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let paginate = pb::paginate::Paginate {
            total: count as u32,
            page,
            page_size,
            total_page: f64::ceil(count as f64 / page_size as f64) as u32,
        };

        Ok(tonic::Response::new(ListReply {
            paginate: Some(paginate),
            address_list: address_list.into_iter().map(Into::into).collect(),
        }))
    }
    /// 设为默认
    async fn set_default(
        &self,
        request: tonic::Request<SetDefaultRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let SetDefaultRequest { id, user_id } = request.into_inner();

        let rows = self._set_default(id, user_id).await.map_err(|e| {
            tracing::error!("set default address error: {}", e);
            tonic::Status::internal(e.to_string())
        })?;

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取默认地址
    async fn get_default(
        &self,
        request: tonic::Request<GetDefaultRequest>,
    ) -> std::result::Result<tonic::Response<OptionAddressReply>, tonic::Status> {
        let user_id = request.into_inner().user_id;
        let sql = r#"SELECT "id", "user_id", "is_default", "consignee", "phone", "address", "province", "city", "county", "post_code", "created_at" FROM "address" WHERE "user_id" = $1 AND "is_default" = true LIMIT 1"#;
        let address: Option<model::Address> = query_as(sql)
            .bind(&user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("get default address error: {}", e);
                tonic::Status::internal(e.to_string())
            })?;
        Ok(tonic::Response::new(OptionAddressReply {
            address: address.map(Into::into),
        }))
    }
}

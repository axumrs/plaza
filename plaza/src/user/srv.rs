use sqlx::{PgPool, QueryBuilder};

use crate::{
    pb::{
        self, req, resp,
        user::{
            GetUserReply, GetUserRequest, ListUserReply, ListUserRequest,
            UpdateUserPasswordRequest, UpdateUserStatusRequest, User, UserExistsRequest,
            UserExistsResponse, UserStatus, get_user_request, user_service_server::UserService,
        },
    },
    types,
    user::model,
    utils,
};

pub struct UserSrv {
    pool: PgPool,
}

impl UserSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl UserService for UserSrv {
    /// 创建用户
    async fn create(
        &self,
        request: tonic::Request<User>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let user = model::User::from(request.into_inner());

        let is_exists_resp = self
            .exists(tonic::Request::new(UserExistsRequest {
                email: user.email.clone(),
                id: None,
            }))
            .await?;

        if is_exists_resp.into_inner().is_exists {
            return Err(tonic::Status::already_exists("用户已存在"));
        }

        let id =  sqlx::query_scalar(r#"INSERT INTO "users" ("id", "email", "password", "nickname", "status", "created_at") VALUES ($1, $2, $3, $4, $5, $6) RETURNING "id""#)
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.nickname)
        .bind(&user.status)
        .bind(&user.created_at)
        .fetch_one(&self.pool)
        .await.map_err(|e| {
            tracing::error!("{e:?}");
            tonic::Status::internal("数据库错误")
        })?;

        Ok(tonic::Response::new(resp::IdReply { id }))
    }
    /// 更新用户
    async fn update(
        &self,
        request: tonic::Request<User>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let user = model::User::from(request.into_inner());
        if user.id.is_empty() {
            return Err(tonic::Status::invalid_argument("id不能为空"));
        }

        let is_exists_resp = self
            .exists(tonic::Request::new(UserExistsRequest {
                email: user.email.clone(),
                id: Some(user.id.clone()),
            }))
            .await?;

        if is_exists_resp.into_inner().is_exists {
            return Err(tonic::Status::already_exists("用户已存在"));
        }

        let pwd = utils::password::hash(&user.password).map_err(|e| {
            tracing::error!("{e:?}");
            tonic::Status::internal("密码加密失败")
        })?;

        let rows = sqlx::query(r#"UPDATE "users" SET "email" = $1, "password" = $2, "nickname" = $3, "status" = $4 WHERE "id" = $5"#)
        .bind(&user.email)
        .bind(&pwd)
        .bind(&user.nickname)
        .bind(&user.status)
        .bind(&user.id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("{e:?}");
            tonic::Status::internal("数据库错误")
        })?.rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 删除用户
    async fn delete(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let id = request.into_inner().id;

        let rows = sqlx::query(r#"DELETE FROM "users" WHERE "id" = $1"#)
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 获取单个用户
    async fn get(
        &self,
        request: tonic::Request<GetUserRequest>,
    ) -> std::result::Result<tonic::Response<GetUserReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "email", "password", "nickname", "status", "created_at" FROM "users" WHERE 1=1"#,
        );
        if let Some(by) = &r.by {
            match by {
                get_user_request::By::Id(id) => q.push(r#" AND "id"="#).push_bind(id),
                get_user_request::By::Email(email) => q.push(r#" AND "email"="#).push_bind(email),
            };
        }
        if let Some(v) = r.status {
            let status = UserStatus::try_from(v).unwrap_or_default();
            let status = model::UserStatus::from(status);
            q.push(r#" AND "status"="#).push_bind(status);
        }
        let user = q
            .build_query_as::<model::User>()
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?;

        let user = match user {
            Some(u) => Some(u.into()),
            None => None,
        };

        Ok(tonic::Response::new(GetUserReply { user }))
    }
    /// 用户列表
    async fn list(
        &self,
        request: tonic::Request<ListUserRequest>,
    ) -> std::result::Result<tonic::Response<ListUserReply>, tonic::Status> {
        let r = request.into_inner();
        let mut q = QueryBuilder::new(
            r#"SELECT "id", "email", "password", "nickname", "status", "created_at" FROM "users" WHERE 1=1"#,
        );
        let mut qc = QueryBuilder::new(r#"SELECT COUNT(*) FROM "users" WHERE 1=1"#);

        let (page, page_size) = if let Some(pr) = r.pr {
            (pr.page.unwrap_or(0), pr.page_size.unwrap_or(30))
        } else {
            (0, 30)
        };

        if let Some(email) = &r.email {
            let email = format!("%{}%", email);
            q.push(r#" AND "email" ILIKE "#).push_bind(email.clone());
            qc.push(r#" AND "email" ILIKE "#).push_bind(email);
        }
        if let Some(nickname) = &r.nickname {
            let nickname = format!("%{}%", nickname);
            q.push(r#" AND "nickname" ILIKE "#)
                .push_bind(nickname.clone());
            qc.push(r#" AND "nickname" ILIKE "#).push_bind(nickname);
        }
        if let Some(status) = r.status {
            let status = UserStatus::try_from(status).unwrap_or_default();
            let status = model::UserStatus::from(status);
            q.push(r#" AND "status"="#).push_bind(status.clone());
            qc.push(r#" AND "status"="#).push_bind(status);
        }
        if let Some(pb::range::DateRange { start, end }) = r.date_range {
            q.push(r#" AND "created_at" BETWEEN "#)
                .push_bind(types::prost2chrono(&start))
                .push(r#" AND "#)
                .push_bind(types::prost2chrono(&end));

            qc.push(r#" AND "created_at" BETWEEN "#)
                .push_bind(types::prost2chrono(&start))
                .push(r#" AND "#)
                .push_bind(types::prost2chrono(&end));
        }
        q.push(r#" ORDER BY "id" DESC"#)
            .push(r#" LIMIT "#)
            .push_bind(page_size as i32)
            .push(r#" OFFSET "#)
            .push_bind((page * page_size) as i32);

        let users = q
            .build_query_as::<model::User>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?
            .into_iter()
            .map(|u| u.into())
            .collect::<Vec<_>>();

        let total: i64 = qc
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?;

        let paginate = pb::paginate::Paginate {
            total: total as u32,
            page,
            page_size,
            total_page: (f64::ceil(total as f64 / page_size as f64)) as u32,
        };
        Ok(tonic::Response::new(ListUserReply {
            paginate: Some(paginate),
            users,
        }))
    }
    /// 用户是否存在
    async fn exists(
        &self,
        request: tonic::Request<UserExistsRequest>,
    ) -> std::result::Result<tonic::Response<UserExistsResponse>, tonic::Status> {
        let input = request.into_inner();
        let mut q = QueryBuilder::new(r#"SELECT COUNT(*) FROM "users" WHERE "email" ="#);
        q.push_bind(&input.email);
        if let Some(id) = &input.id {
            q.push(r#" AND "id" <> "#).push_bind(id);
        }
        let count: i64 = q
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?;

        Ok(tonic::Response::new(UserExistsResponse {
            is_exists: count > 0,
        }))
    }
    /// 修改用户状态
    async fn update_status(
        &self,
        request: tonic::Request<UpdateUserStatusRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();

        let status = pb::user::UserStatus::try_from(r.status).unwrap_or_default();
        let status: model::UserStatus = status.into();

        let rows = sqlx::query(r#"UPDATE "users" SET status=$1 WHERE "id"=$2"#)
            .bind(&status)
            .bind(&r.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
    /// 修改密码
    async fn update_password(
        &self,
        request: tonic::Request<UpdateUserPasswordRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        let r = request.into_inner();

        let pwd = utils::password::hash(&r.password).map_err(|e| {
            tracing::error!("{e:?}");
            tonic::Status::internal("密码加密失败")
        })?;

        let rows = sqlx::query(r#"UPDATE "users" SET "password"=$1 WHERE "id"=$2"#)
            .bind(&pwd)
            .bind(&r.id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("数据库错误")
            })?
            .rows_affected();

        Ok(tonic::Response::new(resp::AffReply { rows }))
    }
}

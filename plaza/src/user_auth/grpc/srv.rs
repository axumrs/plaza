use tonic::{Request, async_trait};

use crate::{
    pb::{
        self, resp,
        user::{self, GetUserRequest, UserStatus},
        user_auth::{UserAuthLoginReply, UserAuthLoginRequest},
    },
    user_auth::grpc::cli::connect_to_user_server,
    utils,
};

pub struct UserAuthSrv {
    user_srv_endpoint: String,
}

impl UserAuthSrv {
    pub fn new(user_srv_endpoint: impl Into<String>) -> Self {
        Self {
            user_srv_endpoint: user_srv_endpoint.into(),
        }
    }
}

#[async_trait]
impl pb::user_auth::user_auth_service_server::UserAuthService for UserAuthSrv {
    async fn register(
        &self,
        request: tonic::Request<user::User>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        let req = request.into_inner();
        let mut cli = connect_to_user_server(&self.user_srv_endpoint)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;
        let resp = cli.create(Request::new(req)).await?.into_inner();

        Ok(tonic::Response::new(resp))
    }
    async fn login(
        &self,
        request: tonic::Request<UserAuthLoginRequest>,
    ) -> std::result::Result<tonic::Response<UserAuthLoginReply>, tonic::Status> {
        let req = request.into_inner();
        let mut cli = connect_to_user_server(&self.user_srv_endpoint)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                tonic::Status::internal(e.to_string())
            })?;

        let user = cli
            .get(Request::new(GetUserRequest {
                status: Some(UserStatus::Actived as i32),
                by: Some(user::get_user_request::By::Email(req.email)),
            }))
            .await?
            .into_inner()
            .user;
        let user = match user {
            Some(v) => v,
            None => return Err(tonic::Status::not_found("用户不存在")),
        };

        if !utils::password::verify(&req.password, &user.password).map_err(|e| {
            tracing::error!("{:?}", e);
            tonic::Status::internal(e.to_string())
        })? {
            return Err(tonic::Status::unauthenticated("密码不正确"));
        }

        Ok(tonic::Response::new(UserAuthLoginReply {
            user: Some(user),
        }))
    }
}

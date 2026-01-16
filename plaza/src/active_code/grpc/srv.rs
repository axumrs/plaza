use std::sync::Arc;

use redis::Client;
use tonic::async_trait;

use crate::{
    pb::{
        self,
        active_code::{ActiveCode, SendActiveCodeRequest, VerifyActiveCodeReply},
    },
    rds,
};

pub struct ActiveCodeSrv {
    cli: Arc<rds::RdsCli>,
    key_prefix: String,
}

impl ActiveCodeSrv {
    pub fn new(cli: rds::RdsCli, key_prefix: impl Into<String>) -> Self {
        Self {
            cli: Arc::new(cli),
            key_prefix: key_prefix.into(),
        }
    }
    fn gen_key(&self, email: &str, kind: i32) -> String {
        format!("{}:{}:{}", self.key_prefix, email, kind)
    }
}

#[async_trait]
impl pb::active_code::active_code_service_server::ActiveCodeService for ActiveCodeSrv {
    async fn send(
        &self,
        request: tonic::Request<SendActiveCodeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();
        let key = self.gen_key(&req.email, req.kind);
        let code: u32 = rand::random_range(100000..=999999);
        let code = format!("{code}");
        let expire_seconds = 60 * 5;

        self.cli
            .set_ex(&key, &code, expire_seconds)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("Redis 错误")
            })?;
        Ok(tonic::Response::new(()))
    }
    async fn verify(
        &self,
        request: tonic::Request<ActiveCode>,
    ) -> std::result::Result<tonic::Response<VerifyActiveCodeReply>, tonic::Status> {
        let req = request.into_inner();
        let key = self.gen_key(&req.email, req.kind);

        let v = self.cli.get::<String>(&key).await.map_err(|e| {
            tracing::error!("{e:?}");
            tonic::Status::internal("Redis 错误")
        })?;

        if let Some(v) = v {
            return Ok(tonic::Response::new(VerifyActiveCodeReply {
                success: v == req.code,
            }));
        }

        Ok(tonic::Response::new(VerifyActiveCodeReply {
            success: false,
        }))
    }
}

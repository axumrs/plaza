use std::sync::Arc;

use tonic::async_trait;

use super::super::model;
use crate::{
    pb::{
        self,
        valid_code::{CreateValidCodeRequest, VerifyValidCodeReply},
    },
    rds,
};

pub struct ValidCodeSrv {
    cli: Arc<rds::RdsCli>,
    key_prefix: String,
    expired_seconds: u64,
}

impl ValidCodeSrv {
    pub fn new(cli: rds::RdsCli, key_prefix: impl Into<String>, expired_seconds: u64) -> Self {
        Self {
            cli: Arc::new(cli),
            key_prefix: key_prefix.into(),
            expired_seconds,
        }
    }
    fn gen_key(&self, email: &str, kind: i32) -> String {
        format!("{}:{}:{}", self.key_prefix, email, kind)
    }
}

#[async_trait]
impl pb::valid_code::valid_code_service_server::ValidCodeService for ValidCodeSrv {
    async fn create(
        &self,
        request: tonic::Request<CreateValidCodeRequest>,
    ) -> std::result::Result<tonic::Response<pb::valid_code::ValidCode>, tonic::Status> {
        let req = request.into_inner();
        let key = self.gen_key(&req.email, req.kind);
        let code: u32 = rand::random_range(100000..=999999);
        let code = format!("{code}");

        let m = model::ValidCode {
            code,
            kind: req.kind().into(),
            email: req.email,
        };
        self.cli
            .set_ex(&key, &m, self.expired_seconds)
            .await
            .map_err(|e| {
                tracing::error!("{e:?}");
                tonic::Status::internal("Redis 错误")
            })?;
        Ok(tonic::Response::new(m.into()))
    }
    async fn verify(
        &self,
        request: tonic::Request<pb::valid_code::ValidCode>,
    ) -> std::result::Result<tonic::Response<VerifyValidCodeReply>, tonic::Status> {
        let req = request.into_inner();
        let key = self.gen_key(&req.email, req.kind);

        let v = self.cli.get::<model::ValidCode>(&key).await.map_err(|e| {
            tracing::error!("{e:?}");
            tonic::Status::internal("Redis 错误")
        })?;

        if let Some(v) = v {
            return Ok(tonic::Response::new(VerifyValidCodeReply {
                success: v.code == req.code,
            }));
        }

        Ok(tonic::Response::new(VerifyValidCodeReply {
            success: false,
        }))
    }
}

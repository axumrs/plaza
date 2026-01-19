use tonic::{
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
};

use crate::{config, jwt};

pub const AUTH_META_KEY: &str = "authorization";
pub const JWT_COFIG_META_KEY: &str = "jwt_secret";
pub const SKIP_AUTH_META_KEY: &str = "skip_auth";
pub fn server_interceptors(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    let skip_auth = req.metadata().get(SKIP_AUTH_META_KEY);
    if skip_auth.is_some() && skip_auth.unwrap().to_str().unwrap() == "true" {
        return Ok(req);
    }

    let jwt_config = match req.metadata().get(JWT_COFIG_META_KEY) {
        Some(v) => v,
        None => return Err(tonic::Status::internal("jwt_secret 不存在")),
    };
    let jwt_config = match jwt_config.to_str() {
        Ok(v) => v,
        Err(_) => return Err(tonic::Status::internal("jwt_secret 格式错误")),
    };

    let jwt_config = serde_json::from_str::<config::JwtConfig>(jwt_config)
        .map_err(|_| tonic::Status::internal("jwt_secret 格式错误"))?;

    let token = match req.metadata().get(AUTH_META_KEY) {
        Some(v) => v,
        None => return Err(tonic::Status::unauthenticated("token 不存在")),
    };

    let token = match token.to_str() {
        Ok(v) => v,
        Err(_) => return Err(tonic::Status::unauthenticated("token 格式错误")),
    };

    // 去除前缀
    let token = token.strip_prefix("Bearer ").unwrap_or(token);

    let c = jwt::get_claims(token, &jwt_config.secret_key)
        .map_err(|_| tonic::Status::unauthenticated("token 格式错误"))?;

    if !c.verify(&jwt_config.sub, jwt::ISSUER) {
        return Err(tonic::Status::unauthenticated("token 验证失败"));
    }

    Ok(req)
}

pub struct UserClientInterceptor {
    pub token: MetadataValue<Ascii>,
    pub cfg: MetadataValue<Ascii>,
}

impl Interceptor for UserClientInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        req.metadata_mut().insert(AUTH_META_KEY, self.token.clone());
        req.metadata_mut()
            .insert(JWT_COFIG_META_KEY, self.cfg.clone());

        Ok(req)
    }
}

use std::ops::Deref;

use axum::extract::FromRequestParts;

use crate::{jwt, util, ArcAppState};

pub struct Auth(jwt::Claims);

impl Deref for Auth {
    type Target = jwt::Claims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequestParts<ArcAppState> for Auth {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &ArcAppState,
    ) -> Result<Self, Self::Rejection> {
        let token = match util::http::get_auth_token(&parts.headers) {
            Some(v) => v,
            None => return Err(crate::Error::Unauthorized("未授权".into())),
        };

        // TODO: 从 extensions 获取 ip 和用户代理
        let ip = util::http::get_ip(&parts.headers);
        let ua = util::http::get_user_agent(&parts.headers);

        let uri = util::http::get_url(&parts.headers, parts.uri.path());

        // TODO：根据URL来选择不同的数据
        let (secret, jwt_sub) = match uri {
            _ => (
                state.cfg.user_jwt.secret.as_str(),
                state.cfg.user_jwt.sub.as_str(),
            ),
        };

        let claims = jwt::get_claims(token, secret)?;

        let claims_verify = claims.verify(jwt_sub, "AXUM.RS", Some((ip, ua)));
        if !claims_verify {
            return Err(crate::Error::Unauthorized("非法令牌".into()));
        }

        Ok(Auth(claims))
    }
}

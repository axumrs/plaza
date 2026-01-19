use axum::extract::FromRequestParts;

use crate::{jwt, user, utils};

pub struct UserAuth {
    pub user: jwt::UserClaimsData,
    pub token: String,
}

impl FromRequestParts<user::api::ArcApiState> for UserAuth {
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &user::api::ArcApiState,
    ) -> Result<Self, Self::Rejection> {
        let token = match utils::http::get_auth_token(&parts.headers) {
            Some(v) => v,
            None => {
                return Err(crate::Error::Custom("token 不存在"));
            }
        };

        let jwt_config = match &state.rtc.user_service.jwt {
            Some(v) => v,
            None => return Err(crate::Error::Custom("jwt 配置错误")),
        };

        let c = jwt::get_claims(token, &jwt_config.secret_key)?;

        if !c.verify(&jwt_config.sub, jwt::ISSUER) {
            return Err(crate::Error::Custom("jwt 验证失败"));
        }

        let u = match c.data {
            jwt::ClaimsData::User(v) => v,
        };
        Ok(UserAuth {
            user: u,
            token: token.to_string(),
        })
    }
}

use std::ops::Deref;

use axum::extract::FromRequestParts;

use crate::util;

/// 客户端信息
pub struct HttpClient(crate::HttpClient);

impl Deref for HttpClient {
    type Target = crate::HttpClient;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> FromRequestParts<S> for HttpClient
where
    S: Send + Sync,
{
    type Rejection = crate::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let ip = util::http::get_ip(&parts.headers);
        let loc = util::http::get_cf_location(&parts.headers);
        let user_agent = util::http::get_user_agent(&parts.headers);

        Ok(HttpClient(crate::HttpClient {
            ip: ip.to_string(),
            loc: loc.to_string(),
            user_agent: user_agent.to_string(),
        }))
    }
}

use axum::extract::FromRequestParts;

pub struct AuthToken(pub Option<String>);

impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头中获取授权令牌
        let auth_token = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));

        match auth_token {
            Some(token) => Ok(AuthToken(Some(token.to_string()))),
            None => Ok(AuthToken(None)),
        }
    }
}

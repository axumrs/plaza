use std::sync::Arc;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub struct CurrentToken(pub String);

pub async fn chain_get_auth_token(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("[chain_get_auth_token] START");
    // 从请求头中获取授权令牌
    let auth_token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    // 确保授权令牌存在
    let auth_token = if let Some(auth_token) = auth_token {
        CurrentToken(auth_token.to_string())
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // 将授权令牌插入到请求的扩展中
    req.extensions_mut().insert(Arc::new(auth_token));

    println!("[chain_get_auth_token] END");
    return Ok(next.run(req).await);
}

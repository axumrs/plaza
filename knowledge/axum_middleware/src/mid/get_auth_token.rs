use axum::{extract::Request, middleware::Next, response::Response};

pub async fn get_auth_token(req: Request, next: Next) -> Response {
    // 从请求头中获取授权令牌
    let auth_token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    println!("鉴权令牌: {:?}", auth_token);

    // 执行下一个中间件/处理函数
    let resp = next.run(req).await;
    // 返回响应
    resp
}

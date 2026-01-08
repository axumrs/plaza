use axum::{extract::Request, middleware::Next, response::Response};

pub async fn req_time(req: Request, next: Next) -> Response {
    // 记录请求开始时间
    let start = std::time::Instant::now();

    // 执行下一个中间件/处理函数
    let resp = next.run(req).await;

    // 计算请求耗时
    let duration = start.elapsed();

    println!("请求耗时: {:?}", duration);

    // 返回响应
    resp
}

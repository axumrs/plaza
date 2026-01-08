use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::AppState;

pub async fn get_time_from_pg(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let (now,): (chrono::DateTime<chrono::Local>,) = sqlx::query_as("SELECT NOW()")
        .fetch_one(&state.pool)
        .await
        .unwrap();

    println!("当前时间: {}", now);
    let resp = next.run(req).await;
    resp
}

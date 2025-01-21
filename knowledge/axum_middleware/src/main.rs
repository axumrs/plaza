#![allow(unused)]

use std::sync::Arc;

use axum::{middleware, response::IntoResponse, routing::get, Router};
use axum_middleware::{mid, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dsn = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@127.0.0.1:5432/postgres?sslmode=disable".into());
    let pool = sqlx::PgPool::connect(&dsn).await.unwrap();
    let state = Arc::new(AppState { pool });

    let addr = "0.0.0.0:9527";

    // 多个独立中间件
    // let app = solo_mid_router_init(state);

    // 中间件之间共享数据
    let app = chain_mid_router_init(state);

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn solo_mid_router_init(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(solo_mid_index_handler))
        .layer(middleware::from_fn(mid::req_time))
        .layer(middleware::from_fn(mid::get_auth_token))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mid::get_time_from_pg,
        ))
        .layer(middleware::from_extractor::<mid::AuthToken>())
        .layer(middleware::from_extractor_with_state::<
            mid::PgNow,
            Arc<AppState>,
        >(state.clone()))
        .with_state(state)
}
async fn solo_mid_index_handler(
    mid::AuthToken(token): mid::AuthToken,
    mid::PgNow(now): mid::PgNow,
) -> impl IntoResponse {
    println!("从extractor中间件中获取鉴权令牌: {:?}", token);
    println!("从extractor中间件中获取数据库时间: {}", now);
    "Hello, World! "
}

fn chain_mid_router_init(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(chain_mid_index_handler))
        .layer(
            tower::ServiceBuilder::new()
                .layer(middleware::from_fn(mid::chain_get_auth_token))
                .layer(middleware::from_extractor_with_state::<
                    mid::TokenAndPgNow,
                    Arc<AppState>,
                >(state.clone())),
        )
        .with_state(state)
}

async fn chain_mid_index_handler(
    mid::TokenAndPgNow { token, now }: mid::TokenAndPgNow,
) -> impl IntoResponse {
    println!(
        "从extractor中间件中获取鉴权令牌: {}, 数据库时间: {}",
        token, now
    );
    "Hello, World! "
}

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

    let app = Router::new()
        .route("/", get(index_handler))
        .layer(middleware::from_fn(mid::req_time))
        .layer(middleware::from_fn(mid::get_auth_token))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mid::get_time_from_pg,
        ))
        .with_state(state);

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    "Hello, World!"
}

use axum::{routing::get, Router};
use pg_inherit::{handler, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:9527";
    let dsn =
        std::env::var("DATABASE_URL").unwrap_or("postgres://test:test@127.0.0.1:5432/test".into());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await?;

    let state = std::sync::Arc::new(AppState { pool });

    let app = Router::new()
        .route("/", get(handler::index).post(handler::create))
        .route(
            "/{id}",
            get(handler::get).put(handler::update).delete(handler::del),
        )
        .with_state(state);

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

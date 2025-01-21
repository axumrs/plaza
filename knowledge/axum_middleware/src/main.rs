use axum::{middleware, routing::get, Router};
use axum_middleware::mid;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:9527";

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(middleware::from_fn(mid::req_time));

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

use axum::{Json, Router, routing::get};
use hotload_config::config::{self, Config};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::spawn(watch_config());

    let listener = TcpListener::bind("127.0.0.1:9527").await?;

    let app = Router::new().route("/", get(get_config_handler).put(set_config_handler));

    axum::serve(listener, app).await?;

    Ok(())
}

async fn watch_config() {
    config::config_watcher().await;
}

async fn get_config_handler() -> Json<Config> {
    let config = config::get_config().into();
    Json(config)
}

async fn set_config_handler(Json(cfg): Json<Config>) -> &'static str {
    config::set_config(cfg).await;
    "OK"
}

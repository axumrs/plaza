use axum::{Json, Router, routing::get};
use notify::{RecommendedWatcher, Watcher};
use static_config_hotload::config::{self, Config};
use std::{fs, path::Path};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut watcher = RecommendedWatcher::new(
        |r: Result<notify::Event, notify::Error>| {
            let event = r.unwrap();
            if event.kind.is_modify() {
                config::load().unwrap();
                println!("config changed");
            }
        },
        notify::Config::default(),
    )?;
    watcher
        .watch(Path::new("./config.json"), notify::RecursiveMode::Recursive)
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:9527").await?;
    let app = Router::new().route("/", get(show_config_handler).put(update_config_handler));

    axum::serve(listener, app).await?;

    Ok(())
}

async fn show_config_handler() -> String {
    let cfg = config::get();
    let cfg = serde_json::to_string(&*cfg).unwrap_or_default();
    cfg
}

async fn update_config_handler(Json(frm): Json<Config>) -> &'static str {
    let contents = serde_json::to_string(&frm).unwrap_or_default();
    fs::write("./config.json", contents.as_bytes()).unwrap();
    "OK"
}

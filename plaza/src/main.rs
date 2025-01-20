use plaza::{config, router, AppState};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or(format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_toml()?;
    let addr = cfg.addr.clone();
    let state = AppState::try_new(cfg).await?;

    let app = router::init(state);

    let listener = TcpListener::bind(&addr).await?;

    info!("Web服务监听于: {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

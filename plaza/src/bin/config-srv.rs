use plaza::{config, utils::log};
use tokio::sync::mpsc;

const CHANNEL_BUFFER_SIZE: usize = 10;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::init();

    config::load_file_config()?;

    let file_config_watch_handler = tokio::spawn(watch_file_config());
    let file_config_reader_handler = tokio::spawn(async move {
        loop {
            let cfg = config::get_file_config();
            tracing::debug!("config: {cfg:?}");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    tokio::select! {
        _ = file_config_watch_handler => {}
        _ = file_config_reader_handler => {}
    }

    Ok(())
}

async fn watch_file_config() {
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>(CHANNEL_BUFFER_SIZE);
    config::watch_file_config(tx, rx).await;
}

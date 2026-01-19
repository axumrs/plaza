use std::sync::Arc;

use plaza::{config, utils::log};
use tokio::sync::mpsc;

const CHANNEL_BUFFER_SIZE: usize = 10;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::init();

    let fc = config::load_file_config()?;
    let etcd_config = Arc::new(fc.etcd.clone());

    // 如果命令行带有 init_runtime 参数，就初始化
    if std::env::args().any(|arg| arg == "init_runtime") {
        let init_runtime_cfg = config::RuntimeConfig {
            mails: vec![],
            user_service: config::ServiceConfig {
                name: "user".into(),
                db: Some(config::DatabaseConfig::default()),
                rds: Some(config::RedisConfig::default()),
                jwt: Some(config::JwtConfig {
                    sub: "AXUM.EU.ORG/PLAZA/USER".into(),
                    secret_key: "".into(),
                    timeout: 10,
                }),
                srv_addr: Some("127.0.0.1:40000".into()),
                api_prefix: Some("/user".into()),
            },
            auth_service: config::ServiceConfig {
                name: "auth".into(),
                db: None,
                rds: Some(config::RedisConfig::default()),
                jwt: None,
                srv_addr: Some("127.0.0.1:40001".into()),
                api_prefix: Some("/auth".into()),
            },
            ..Default::default()
        };
        config::set_runtime_config(etcd_config.clone(), init_runtime_cfg).await?;

        tracing::info!("运行时配置初始化成功");
        return Ok(());
    }

    config::load_runtime_config(etcd_config.clone()).await?;

    let file_config_watch_handler = tokio::spawn(watch_file_config());
    let file_config_reader_handler = tokio::spawn(async move {
        loop {
            let cfg = config::get_file_config();
            tracing::debug!("file config: {cfg:?}");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });
    let runtime_config_watch_handler = tokio::spawn(config::watch_runtime_config(etcd_config));
    let runtime_config_reader_handler = tokio::spawn(async move {
        loop {
            let cfg = config::get_runtime_config();
            tracing::debug!("runtime config: {cfg:?}");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    tokio::select! {
        _ = file_config_watch_handler => {}
        _ = file_config_reader_handler => {}
        _ = runtime_config_watch_handler => {}
        _ = runtime_config_reader_handler => {}
    }

    Ok(())
}

async fn watch_file_config() {
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>(CHANNEL_BUFFER_SIZE);
    config::watch_file_config(tx, rx).await;
}

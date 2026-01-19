use std::sync::{Arc, LazyLock};

use arc_swap::ArcSwap;
use notify::Watcher;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::Result;

pub const CONFIG_FILE_PATH: &str = "config.json";

static FILE_CONFIG: LazyLock<ArcSwap<FileConfig>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(FileConfig::default())));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileConfig {
    pub web_addr: String,
    pub etcd: EtcdConfig,
    pub turnstile: TurnstileConfig,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            web_addr: "127.0.0.1:9527".to_string(),
            etcd: EtcdConfig::default(),
            turnstile: TurnstileConfig::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EtcdConfig {
    pub endpoints: Vec<String>,
    pub timeout_secs: u8,
}

impl Default for EtcdConfig {
    fn default() -> Self {
        Self {
            endpoints: vec!["http://127.0.0.1:2379".to_string()],
            timeout_secs: 10,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurnstileConfig {
    pub secret: String,
    pub timeout_secs: u8,
}

impl Default for TurnstileConfig {
    fn default() -> Self {
        Self {
            secret: "1x0000000000000000000000000000000AA".to_string(),
            timeout_secs: 10,
        }
    }
}

impl From<Arc<FileConfig>> for FileConfig {
    fn from(v: Arc<FileConfig>) -> Self {
        Self {
            web_addr: v.web_addr.clone(),
            etcd: v.etcd.clone(),
            turnstile: v.turnstile.clone(),
        }
    }
}

pub fn get_file_config() -> Arc<FileConfig> {
    (*FILE_CONFIG.load()).clone()
}

pub fn load_file_config() -> Result<Arc<FileConfig>> {
    let file = std::fs::File::open(CONFIG_FILE_PATH)?;
    if file.metadata()?.len() == 0 {
        return Ok(get_file_config());
    }
    let cfg = serde_json::from_reader(&file)?;

    FILE_CONFIG.store(Arc::new(cfg));

    Ok(get_file_config())
}

pub async fn watch_file_config(
    tx: tokio::sync::mpsc::Sender<notify::Result<notify::Event>>,
    mut rx: tokio::sync::mpsc::Receiver<notify::Result<notify::Event>>,
) {
    let mut watcher = match notify::RecommendedWatcher::new(
        move |result: notify::Result<notify::Event>| {
            if let Err(e) = tx.blocking_send(result) {
                tracing::error!("notify事件发送失败：{e:?}");
            }
        },
        notify::Config::default(),
    ) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("watcher创建失败：{e:?}");
            return;
        }
    };
    if let Err(e) = watcher.watch(
        Path::new(CONFIG_FILE_PATH),
        notify::RecursiveMode::Recursive,
    ) {
        tracing::error!("watcher监听失败：{e:?}");
        return;
    }

    while let Some(event) = rx.recv().await {
        if let Ok(event) = event {
            if event.kind.is_modify() {
                tracing::info!("配置文件修改");
                if let Err(e) = load_file_config() {
                    tracing::error!("加载配置失败：{e:?}");
                }
            }
        }
    }
}

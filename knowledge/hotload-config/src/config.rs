use std::sync::{Arc, LazyLock};

use arc_swap::ArcSwap;
use etcd_client::{Client, WatchOptions};
use serde::{Deserialize, Serialize};

const ETCD_KEY: &str = "/CONFIG";
static CONFIG: LazyLock<ArcSwap<Config>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(Config::default())));

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub web_addr: String,
    pub database_url: String,
    pub timeout: u64,
    pub disabled_register: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            web_addr: String::from("127.0.0.1:9527"),
            database_url: String::from("postgres://postgres:postgres@localhost:5432/postgres"),
            timeout: 60,
            disabled_register: false,
        }
    }
}

impl From<Arc<Config>> for Config {
    fn from(value: Arc<Config>) -> Self {
        Self {
            web_addr: value.web_addr.clone(),
            database_url: value.database_url.clone(),
            timeout: value.timeout,
            disabled_register: value.disabled_register,
        }
    }
}

pub fn get_config() -> Arc<Config> {
    (*CONFIG.load()).clone()
}

pub async fn set_config(config: Config) {
    let mut cli = Client::connect(["http://127.0.0.1:2379"], None)
        .await
        .unwrap();

    let value = serde_json::to_string(&config).unwrap();
    cli.put(ETCD_KEY, value, None).await.unwrap();
    //CONFIG.store(Arc::new(config));
}

pub async fn config_watcher() {
    let mut cli = Client::connect(["http://127.0.0.1:2379"], None)
        .await
        .unwrap();
    let (_, mut rx) = cli
        .watch(ETCD_KEY, Some(WatchOptions::default().with_watch_id(1)))
        .await
        .unwrap();

    while let Some(msg) = rx.message().await.unwrap() {
        for e in msg.events() {
            if let Some(kv) = e.kv() {
                if let Ok(v) = kv.value_str() {
                    let config: Config = serde_json::from_str(&v).unwrap();
                    CONFIG.store(Arc::new(config));
                    println!("配置已更新");
                }
            }
        }
    }
}

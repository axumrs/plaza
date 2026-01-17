use std::sync::{Arc, LazyLock};

use arc_swap::ArcSwap;

use serde::{Deserialize, Serialize};
use std::fs;

static CONFIG: LazyLock<ArcSwap<Config>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(Config::default())));

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub addr: String,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:9527".into(),
            database: DatabaseConfig::default(),
        }
    }
}

impl From<Arc<Config>> for Config {
    fn from(v: Arc<Config>) -> Self {
        Self {
            addr: v.addr.clone(),
            database: DatabaseConfig {
                url: v.database.url.clone(),
                max_conns: v.database.max_conns,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_conns: u32,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
            max_conns: 5,
        }
    }
}

pub fn get() -> Arc<Config> {
    (*CONFIG.load()).clone()
}

pub fn load() -> anyhow::Result<Arc<Config>> {
    let file = fs::File::open("./config.json")?;
    if file.metadata()?.len() == 0 {
        println!("empty file");
    }

    let cfg = serde_json::from_reader(&file)?;
    CONFIG.store(Arc::new(cfg));

    Ok(get())
}

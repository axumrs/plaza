use std::sync::{Arc, LazyLock};

use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};

static RUNTIME_CONFIG: LazyLock<ArcSwap<RuntimeConfig>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(RuntimeConfig::default())));

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuntimeConfig {
    pub mails: Vec<MailConfig>,
    pub user_service: ServiceConfig,
    pub auth_service: ServiceConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MailConfig {
    pub smtp: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServiceConfig {
    pub name: String,
    pub db: Option<DatabaseConfig>,
    pub rds: Option<RedisConfig>,
    pub jwt: Option<JwtConfig>,
    pub srv_addr: Option<String>,
    pub api_prefix: Option<String>,
}

impl ServiceConfig {
    pub fn must_get_db(&self) -> &DatabaseConfig {
        self.db.as_ref().unwrap()
    }

    pub fn must_get_rds(&self) -> &RedisConfig {
        self.rds.as_ref().unwrap()
    }

    pub fn must_get_jwt(&self) -> &JwtConfig {
        self.jwt.as_ref().unwrap()
    }

    pub fn must_get_srv_addr(&self) -> &String {
        self.srv_addr.as_ref().unwrap()
    }

    pub fn must_get_api_prefix(&self) -> &String {
        self.api_prefix.as_ref().unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub dsn: String,
    pub max_conns: u8,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            dsn: "postgres://plaze:plaze@127.0.0.1:5432/plaze".to_string(),
            max_conns: 5,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}
impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379/0".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JwtConfig {
    pub sub: String,
    pub secret_key: String,
    pub timeout: u32,
}

pub fn get_runtime_config() -> Arc<RuntimeConfig> {
    (*RUNTIME_CONFIG.load()).clone()
}

async fn get_etcd_cli(etcd_cfg: Arc<super::EtcdConfig>) -> crate::Result<etcd_client::Client> {
    let cli = etcd_client::Client::connect(
        &etcd_cfg.endpoints,
        Some(
            etcd_client::ConnectOptions::default()
                .with_timeout(std::time::Duration::from_secs(etcd_cfg.timeout_secs.into())),
        ),
    )
    .await?;
    Ok(cli)
}

pub async fn load_runtime_config(
    etcd_cfg: Arc<super::EtcdConfig>,
) -> crate::Result<Arc<RuntimeConfig>> {
    let mut cli = get_etcd_cli(etcd_cfg.clone()).await?;
    let resp = cli.get(etcd_cfg.key_prefix.as_bytes(), None).await?;

    if let Some(kv) = resp.kvs().first() {
        let config = serde_json::from_slice::<RuntimeConfig>(kv.value())?;
        let cfg = Arc::new(config);
        RUNTIME_CONFIG.store(cfg.clone());

        return Ok(cfg);
    }

    Ok(get_runtime_config())
}

pub async fn set_runtime_config(
    etcd_cfg: Arc<super::EtcdConfig>,
    cfg: RuntimeConfig,
) -> crate::Result<()> {
    let mut cli = get_etcd_cli(etcd_cfg.clone()).await?;

    let value = serde_json::to_string(&cfg)?;
    tracing::debug!("{}", value);

    cli.put(etcd_cfg.key_prefix.as_bytes(), value.as_bytes(), None)
        .await?;

    Ok(())
}

pub async fn watch_runtime_config(etcd_cfg: Arc<super::EtcdConfig>) -> crate::Result<()> {
    let mut cli = get_etcd_cli(etcd_cfg.clone()).await?;

    let mut watch_stream = cli
        .watch(
            etcd_cfg.key_prefix.as_bytes(),
            Some(etcd_client::WatchOptions::default().with_watch_id(1)),
        )
        .await?;

    while let Some(msg) = watch_stream.message().await? {
        for e in msg.events() {
            if let Some(kv) = e.kv() {
                if let Ok(v) = kv.value_str() {
                    cli.put(etcd_cfg.key_prefix.as_bytes(), v.as_bytes(), None)
                        .await?;

                    let config = serde_json::from_str(v)?;
                    RUNTIME_CONFIG.store(Arc::new(config));

                    tracing::info!("运行时配置已更新");
                }
            }
        }
    }

    Ok(())
}

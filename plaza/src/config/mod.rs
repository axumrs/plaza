mod file_config;
mod runtime_config;

use std::sync::Arc;

pub use file_config::*;
pub use runtime_config::*;

pub async fn load_config() -> crate::Result<(Arc<FileConfig>, Arc<RuntimeConfig>)> {
    let fc = load_file_config()?;
    let etcd_config = Arc::new(fc.etcd.clone());
    let rc = load_runtime_config(etcd_config.clone()).await?;
    Ok((fc, rc))
}

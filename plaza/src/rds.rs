use redis::{AsyncTypedCommands, Client, cluster::ClusterClient};
use serde::{Serialize, de::DeserializeOwned};

use crate::Result;

pub async fn set<T: Serialize, C: AsyncTypedCommands>(
    cli: &mut C,
    key: &str,
    value: &T,
) -> Result<()> {
    let value = serde_json::to_string(value)?;
    cli.set(key, value).await?;
    Ok(())
}

pub async fn set_ex<T: Serialize, C: AsyncTypedCommands>(
    cli: &mut C,
    key: &str,
    value: &T,
    seconds: u64,
) -> Result<()> {
    let value = serde_json::to_string(value)?;
    cli.set_ex(key, value, seconds).await?;
    Ok(())
}

pub async fn get<T: DeserializeOwned, C: AsyncTypedCommands>(
    cli: &mut C,
    key: &str,
) -> Result<Option<T>> {
    let value = match cli.get(key).await? {
        Some(v) => v,
        None => return Ok(None),
    };

    Ok(Some(serde_json::from_str::<T>(&value)?))
}

pub enum RdsCli {
    Single(Client),
    Cluster(ClusterClient),
}

impl RdsCli {
    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        match self {
            Self::Single(cli) => {
                let mut cli = cli.get_multiplexed_async_connection().await?;
                set(&mut cli, key, value).await
            }
            Self::Cluster(cli) => {
                let mut cli = cli.get_async_connection().await?;
                set(&mut cli, key, value).await
            }
        }
    }
    pub async fn set_ex<T: Serialize>(&self, key: &str, value: &T, seconds: u64) -> Result<()> {
        match self {
            Self::Single(cli) => {
                let mut cli = cli.get_multiplexed_async_connection().await?;
                set_ex(&mut cli, key, value, seconds).await
            }
            Self::Cluster(cli) => {
                let mut cli = cli.get_async_connection().await?;
                set_ex(&mut cli, key, value, seconds).await
            }
        }
    }
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        match self {
            Self::Single(cli) => {
                let mut cli = cli.get_multiplexed_async_connection().await?;
                get(&mut cli, key).await
            }
            Self::Cluster(cli) => {
                let mut cli = cli.get_async_connection().await?;
                get(&mut cli, key).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use redis::{aio::MultiplexedConnection, cluster_async::ClusterConnection};
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        id: u32,
        name: String,
    }

    const REDIS_KEY: &str = "rds_test";
    const REDIS_EX_KEY: &str = "rds_test_ex";

    #[allow(unused)]
    /// redis集群。
    /// 需要在 redis.conf 中开启 cluster
    async fn get_cli_cluster() -> Result<ClusterConnection> {
        let url = std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379/0".into());
        let cli = redis::cluster::ClusterClient::new([url.as_str()])?
            .get_async_connection()
            .await?;
        Ok(cli)
    }
    async fn get_cli() -> Result<MultiplexedConnection> {
        let url = std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379/0".into());
        let cli = redis::Client::open(url.as_str())?
            .get_multiplexed_async_connection()
            .await?;
        Ok(cli)
    }

    #[tokio::test]
    async fn rds_test_set() -> Result<()> {
        let u = User {
            id: 1,
            name: "张三".into(),
        };
        let mut cli = get_cli().await?;
        set(&mut cli, REDIS_KEY, &u).await?;
        Ok(())
    }

    #[tokio::test]
    async fn rds_test_get() -> Result<()> {
        let mut cli = get_cli().await?;
        let u = get::<User, _>(&mut cli, REDIS_KEY).await?;

        assert!(u.is_some());

        println!("{:?}", u);
        Ok(())
    }

    #[tokio::test]
    async fn rds_test_set_ex() -> Result<()> {
        let mut cli = get_cli().await?;
        let u = User {
            id: 2,
            name: "李四".into(),
        };
        set_ex(&mut cli, REDIS_EX_KEY, &u, 10).await?;
        Ok(())
    }

    #[tokio::test]
    async fn rds_test_get_ex() -> Result<()> {
        let mut cli = get_cli().await?;
        let u = get::<User, _>(&mut cli, REDIS_EX_KEY).await?;

        assert!(u.is_some());
        println!("{:?}", u);

        tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;

        let u = get::<User, _>(&mut cli, REDIS_EX_KEY).await?;
        assert!(u.is_none());

        Ok(())
    }
}

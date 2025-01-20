use std::sync::Arc;

use crate::config;

pub struct AppState {
    pub cfg: Arc<config::Config>,
    pub pool: sqlx::PgPool,
}

impl AppState {
    pub async fn try_new(cfg: config::Config) -> crate::Result<ArcAppState> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(cfg.database.max_conns)
            .connect(&cfg.database.dsn)
            .await?;

        Ok(Arc::new(AppState {
            cfg: Arc::new(cfg),
            pool,
        }))
    }
}

pub type ArcAppState = Arc<AppState>;

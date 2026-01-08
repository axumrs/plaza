use std::sync::Arc;

use axum::extract::FromRequestParts;

use crate::AppState;

pub struct PgNow(pub String);

impl FromRequestParts<Arc<AppState>> for PgNow {
    type Rejection = ();

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let (now,): (chrono::DateTime<chrono::Local>,) = sqlx::query_as("SELECT NOW()")
            .fetch_one(&state.pool)
            .await
            .unwrap();

        Ok(Self(now.format("%Y-%m-%d %H:%M:%S").to_string()))
    }
}

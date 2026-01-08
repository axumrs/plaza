use std::sync::Arc;

use axum::{extract::FromRequestParts, http::StatusCode};

use crate::AppState;

use super::CurrentToken;

pub struct TokenAndPgNow {
    pub token: String,
    pub now: String,
}

impl FromRequestParts<Arc<AppState>> for TokenAndPgNow {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        println!("[chain_using_token_and_get_time_from_pg_as_extractor] START");
        let current_token = parts
            .extensions
            .get::<Arc<CurrentToken>>()
            .ok_or(StatusCode::UNAUTHORIZED)
            .unwrap();

        let (now,): (chrono::DateTime<chrono::Local>,) = sqlx::query_as("SELECT NOW()")
            .fetch_one(&state.pool)
            .await
            .unwrap();

        println!("[chain_using_token_and_get_time_from_pg_as_extractor] END");

        Ok(Self {
            token: current_token.0.clone(),
            now: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }
}

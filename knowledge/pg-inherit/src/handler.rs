use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};

use crate::{form, model, ArcAppState, Result};

pub async fn index(State(state): State<ArcAppState>) -> Result<Json<Vec<model::User>>> {
    let ls = sqlx::query_as("SELECT id, username, nickname FROM users ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await?;
    Ok(Json(ls))
}

pub async fn create(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::CreateUserForm>,
) -> Result<Json<model::UserAccount>> {
    let mut tx = state.pool.begin().await?;

    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_accounts WHERE username = $1")
        .bind(&frm.username)
        .fetch_one(&mut *tx)
        .await?;
    if count > 0 {
        tx.rollback().await?;
        return Err(anyhow!("用户名{}已存在", &frm.username).into());
    }

    let user = match sqlx::query_as(
        "INSERT INTO user_accounts (username, nickname, balance) VALUES ($1, $2, $3) RETURNING id, username, nickname, balance",
    )
    .bind(&frm.username)
    .bind(&frm.nickname)
    .bind(frm.balance)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };
    tx.commit().await?;
    Ok(Json(user))
}

pub async fn update(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
    Json(frm): Json<form::UpdateUserForm>,
) -> Result<Json<u64>> {
    let aff = sqlx::query("UPDATE user_accounts SET nickname=$1, balance=$2 WHERE id=$3")
        .bind(&frm.nickname)
        .bind(frm.balance)
        .bind(id)
        .execute(&state.pool)
        .await?
        .rows_affected();
    Ok(Json(aff))
}

pub async fn del(State(state): State<ArcAppState>, Path(id): Path<i32>) -> Result<Json<u64>> {
    let aff = sqlx::query("DELETE FROM user_accounts WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await?
        .rows_affected();
    Ok(Json(aff))
}

pub async fn get(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<model::UserAccount>>> {
    let user =
        sqlx::query_as("SELECT id, username, nickname,balance FROM user_accounts WHERE id=$1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?;
    Ok(Json(user))
}

use crate::{Result, api_resp, pb};

use super::{ArcActiveCodeState, payload};
use axum::{Json, extract::State};
use tonic::Request;
use validator::Validate;

pub async fn send(
    State(state): State<ArcActiveCodeState>,
    Json(payload): Json<payload::SendValidCodePayload>,
) -> Result<api_resp::JsonResp<()>> {
    payload.validate()?;

    // TODO: 人机验证

    // grpc
    let mut cli = state.cli.clone();
    let kind: pb::valid_code::ValidCodeKind = payload.kind.into();

    cli.create(Request::new(pb::valid_code::CreateValidCodeRequest {
        email: payload.email,
        kind: kind as i32,
    }))
    .await?;

    // 发送邮件
    tokio::spawn(send_email());

    Ok(api_resp::ok_empty().to_json())
}

async fn send_email() {}

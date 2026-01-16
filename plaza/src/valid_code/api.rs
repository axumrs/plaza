use crate::{Result, api_resp, mail, pb};

use super::model;
use super::{ArcActiveCodeState, payload};
use axum::{Json, extract::State};
use serde::Serialize;
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

    let res = cli
        .create(Request::new(pb::valid_code::CreateValidCodeRequest {
            email: payload.email,
            kind: kind as i32,
        }))
        .await?
        .into_inner();

    // 发送邮件
    tokio::spawn(send_email(res.into()));

    Ok(api_resp::ok_empty().to_json())
}

#[derive(Serialize)]
pub struct VerifyValidCodeResp {
    pub success: bool,
}

pub async fn verify(
    State(state): State<ArcActiveCodeState>,
    Json(payload): Json<payload::ValidateValidCodePayload>,
) -> Result<api_resp::JsonResp<VerifyValidCodeResp>> {
    payload.validate()?;

    // TODO: 人机验证

    let mut cli = state.cli.clone();
    let kind: pb::valid_code::ValidCodeKind = payload.kind.into();

    let resp = cli
        .verify(Request::new(pb::valid_code::ValidCode {
            code: payload.valid_code,
            kind: kind as i32,
            email: payload.email,
        }))
        .await?
        .into_inner();

    Ok(api_resp::ok(VerifyValidCodeResp {
        success: resp.success,
    })
    .to_json())
}

async fn send_email(m: model::ValidCode) {
    let body = format!("你的激活码是：{}", m.code);
    let data = mail::Data::new("激活码", body, &m.email);

    let res = match mail::send(m.email, m.code, "127.0.0.1:40001", data).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("{e:?}");
            return;
        }
    };

    tracing::info!("邮件发送结果：{res:?}");
}

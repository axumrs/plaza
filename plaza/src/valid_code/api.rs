use std::sync::Arc;

use crate::{Error, Result, api_resp, captcha, config, mail, pb};

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

    let turnstile =
        &captcha::Turnstile::new(&state.fc.turnstile.secret, state.fc.turnstile.timeout_secs);
    if !captcha::verify(turnstile, &payload.captcha).await?.success {
        return Err(Error::Custom("人机验证失败"));
    }

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
    tokio::spawn(send_email(res.into(), state.rtc.clone()));

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

    let turnstile =
        &captcha::Turnstile::new(&state.fc.turnstile.secret, state.fc.turnstile.timeout_secs);
    if !captcha::verify(turnstile, &payload.captcha).await?.success {
        return Err(Error::Custom("人机验证失败"));
    }

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

async fn send_email(m: model::ValidCode, rtc: Arc<config::RuntimeConfig>) {
    let mc = match rtc.mail() {
        Some(v) => v,
        None => {
            tracing::error!("邮件配置不存在");
            return;
        }
    };
    let body = format!("你的激活码是：{}", m.code);
    let data = mail::Data::new("激活码", body, &m.email);

    let res = match mail::send(&mc.user, &mc.password, &mc.smtp, data).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("{e:?}");
            return;
        }
    };

    tracing::info!("邮件发送结果：{res:?}");
}

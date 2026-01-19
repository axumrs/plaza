use axum::{Json, extract::State};
use tonic::Request;
use validator::Validate;

use crate::{
    Error, Result, api_resp, captcha, helper, jwt, pb, types,
    user_auth::{ArcUserAuthState, payload},
    utils,
};

pub async fn register(
    State(state): State<ArcUserAuthState>,
    Json(pl): Json<payload::RegisterPayload>,
) -> Result<api_resp::JsonResp<api_resp::Id>> {
    pl.validate()?;

    let turnstile =
        &captcha::Turnstile::new(&state.fc.turnstile.secret, state.fc.turnstile.timeout_secs);
    if !captcha::verify(turnstile, &pl.captcha).await?.success {
        return Err(Error::Custom("人机验证失败"));
    }

    // 验证激活码
    if !helper::invoke_valid_code_verify(&pl.valid_code, &pl.email, 0, state.vc_cli.clone()).await?
    {
        return Err(Error::Custom("激活码错误"));
    }

    let pwd = utils::password::hash(&pl.password)?;
    let mut cli = state.cli.clone();
    let r = cli
        .register(Request::new(pb::user::User {
            id: utils::id::new(),
            email: pl.email,
            password: pwd,
            nickname: pl.nickname,
            status: pb::user::UserStatus::Actived as i32,
            created_at: types::chrono_now_to_prost(),
        }))
        .await?;

    Ok(api_resp::ok(api_resp::Id {
        id: r.into_inner().id,
    })
    .to_json())
}

pub async fn login(
    State(state): State<ArcUserAuthState>,
    Json(pl): Json<payload::LoginPayload>,
) -> Result<api_resp::JsonResp<String>> {
    pl.validate()?;

    let turnstile =
        &captcha::Turnstile::new(&state.fc.turnstile.secret, state.fc.turnstile.timeout_secs);
    if !captcha::verify(turnstile, &pl.captcha).await?.success {
        return Err(Error::Custom("人机验证失败"));
    }

    let mut cli = state.cli.clone();
    let user = cli
        .login(Request::new(pb::user_auth::UserAuthLoginRequest {
            email: pl.email,
            password: pl.password,
        }))
        .await?
        .into_inner()
        .user;
    let user = match user {
        Some(v) => v,
        None => return Err(Error::Custom("用户不存在")),
    };

    let srv_cfg = &state.rtc.user_service;
    let jwt_cfg = match &srv_cfg.jwt {
        Some(v) => v,
        None => return Err(Error::Custom("jwt配置错误")),
    };

    let data = jwt::ClaimsData::User(jwt::UserClaimsData {
        id: user.id,
        email: user.email,
        nickname: user.nickname,
    });
    let token = jwt::token(
        data,
        &jwt_cfg.secret_key,
        &jwt_cfg.sub,
        jwt_cfg.timeout.into(),
    )?
    .to_string();

    Ok(api_resp::ok(token).to_json())
}

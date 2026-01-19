use crate::{Error, Result, api_resp, mw, pb, user::grpc::cli, utils};

use super::{ArcApiState, payload};
use axum::{Json, extract::State};
use tonic::Request;
use validator::Validate;

pub async fn update_nickname(
    State(state): State<ArcApiState>,
    mw::UserAuth { user, token }: mw::UserAuth,
    Json(pl): Json<payload::web::UpdateProfilePayload>,
) -> Result<api_resp::JsonResp<api_resp::Aff>> {
    pl.validate()?;

    let mut cli = cli::connect(&token, &state.rtc.user_service).await?;

    let r = cli
        .update_nickname(Request::new(pb::user::UpdateUserNicknameRequest {
            id: user.id,
            nickname: pl.nickname,
        }))
        .await?
        .into_inner();

    Ok(api_resp::ok(api_resp::Aff { rows: r.rows }).to_json())
}

pub async fn update_password(
    State(state): State<ArcApiState>,
    mw::UserAuth { user, token }: mw::UserAuth,
    Json(pl): Json<payload::web::UpdatePasswordPayload>,
) -> Result<api_resp::JsonResp<api_resp::Aff>> {
    pl.validate()?;

    let mut cli = cli::connect(&token, &state.rtc.user_service).await?;

    let got_user = cli
        .get(Request::new(pb::user::GetUserRequest {
            status: Some(pb::user::UserStatus::Actived as i32),
            by: Some(pb::user::get_user_request::By::Id(user.id.clone())),
        }))
        .await?
        .into_inner();
    let got_user = match got_user.user {
        Some(v) => v,
        None => return Err(Error::Custom("用户不存在")),
    };

    if !utils::password::verify(&pl.password, &got_user.password)? {
        return Err(Error::Custom("密码错误"));
    }

    let pwd = utils::password::hash(&pl.new_password)?;

    let r = cli
        .update_password(Request::new(pb::user::UpdateUserPasswordRequest {
            id: user.id,
            password: pwd,
        }))
        .await?
        .into_inner();

    Ok(api_resp::ok(api_resp::Aff { rows: r.rows }).to_json())
}

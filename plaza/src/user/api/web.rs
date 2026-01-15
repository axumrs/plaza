use crate::{Error, Result, api_resp, pb, types, utils};

use super::{ArcApiState, payload};
use axum::{Json, extract::State};
use tonic::Request;
use validator::Validate;

pub async fn send_valid_code(
    State(_state): State<ArcApiState>,
    Json(pl): Json<payload::web::SendValidCodePayload>,
) -> Result<api_resp::JsonResp<()>> {
    pl.validate()?;

    // TODO: 人机验证

    // TODO: 生成、保存验证码

    // TODO：发送验证码

    Ok(api_resp::ok_empty().to_json())
}

pub async fn register(
    State(state): State<ArcApiState>,
    Json(pl): Json<payload::web::RegisterPayload>,
) -> Result<api_resp::JsonResp<api_resp::Id>> {
    pl.validate()?;

    // TODO: 人机验证

    let pwd = utils::password::hash(&pl.password)?;
    let mut cli = state.cli.clone();
    let r = cli
        .create(Request::new(pb::user::User {
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
    State(state): State<ArcApiState>,
    Json(pl): Json<payload::web::LoginPayload>,
) -> Result<api_resp::JsonResp<String>> {
    pl.validate()?;

    // TODO: 人机验证

    let mut cli = state.cli.clone();
    let user_resp = cli
        .get(Request::new(pb::user::GetUserRequest {
            status: Some(pb::user::UserStatus::Actived as i32),
            by: Some(pb::user::get_user_request::By::Email(pl.email.clone())),
        }))
        .await?;

    let user = match user_resp.into_inner().user {
        Some(v) => v,
        None => return Err(Error::Custom("用户不存在")),
    };

    if !utils::password::verify(&pl.password, &user.password)? {
        return Err(Error::Custom("账号/密码错误"));
    }

    let token = String::new(); // TODO： JWT

    Ok(api_resp::ok(token).to_json())
}

pub async fn update_profile(
    State(_state): State<ArcApiState>,
    Json(pl): Json<payload::web::UpdateProfilePayload>,
) -> Result<api_resp::JsonResp<api_resp::Aff>> {
    pl.validate()?;

    // TODO: 中间件

    unimplemented!()
}

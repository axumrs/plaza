use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::{
    activation_code, jwt, login_log, mail, mid, resp, turnstile,
    util::{self, dt},
    ArcAppState, Error, Result,
};

use super::{db, form, model, service};

/// 注册
pub async fn register(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::RegisterForm>,
) -> Result<resp::JsonIdResp> {
    frm.validate()?;

    if !turnstile::verify(&frm.captcha, &state.cfg.turnstile).await? {
        return Err(anyhow::anyhow!("人机验证失败").into());
    }

    let email = frm.email;

    let status = if state.cfg.user.should_verify_email {
        model::UserStatus::Pending
    } else {
        model::UserStatus::Actived
    };

    let mut tx = state.pool.begin().await?;
    let m = model::User::try_new(email.clone(), frm.password, status)?;
    let id = match service::create(&mut tx, &m).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e);
        }
    };

    if state.cfg.user.should_verify_email {
        // 验证码
        let ac = activation_code::model::ActivationCode::try_new(
            id.clone(),
            activation_code::model::ActivationCodeKind::Register,
            state.cfg.activation_code.resend_duration,
            state.cfg.activation_code.expire_duration,
        )?;
        let code = match activation_code::service::create(
            &mut tx,
            ac,
            state.cfg.activation_code.max_retry_count,
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        // 发送邮件
        let mail_cfg = state.cfg.get_mail()?;
        let subject = "注册激活";
        let body = format!("您的激活码为: {}", code);
        let mail_data = mail::Data::new(subject, body, &email);
        tokio::spawn(mail::send(
            mail_cfg.user.clone(),
            mail_cfg.password.clone(),
            mail_cfg.smtp.clone(),
            mail_data,
        ));
    }

    tx.commit().await?;

    resp::id(id)
}

// 激活
pub async fn activate(
    State(state): State<ArcAppState>,
    Path(code): Path<String>,
) -> Result<resp::JsonAffResp> {
    if code.len() != 6 {
        return Err(anyhow::anyhow!("激活码错误").into());
    }

    let mut tx = state.pool.begin().await?;
    let ac = match activation_code::db::find_by_code(&mut *tx, &code).await {
        Ok(v) => match v {
            Some(v) => v,
            None => {
                return Err(anyhow::anyhow!("激活码不存在").into());
            }
        },
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    // 是否注册时的激活码
    if !matches!(
        ac.kind,
        activation_code::model::ActivationCodeKind::Register
    ) {
        return Err(anyhow::anyhow!("激活码错误").into());
    }

    // 是否过期
    if ac.expired_at < dt::now() {
        return Err(anyhow::anyhow!("激活码已过期").into());
    }

    // 激活
    let user = match db::find(
        &mut *tx,
        &model::FindFilter {
            by: model::FindBy::ID(ac.user_id),
            status: Some(model::UserStatus::Pending),
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => {
                return Err(anyhow::anyhow!("用户不存在").into());
            }
        },
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    let user = model::User {
        status: model::UserStatus::Actived,
        ..user
    };

    let rows = match db::update(&mut *tx, &user).await {
        Ok(v) => v,
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };

    // 删除激活码
    if let Err(e) = activation_code::db::delete(&mut *tx, &ac.id).await {
        tx.rollback().await?;
        return Err(e.into());
    };

    tx.commit().await?;

    resp::aff(rows)
}

// 登录
pub async fn login(
    State(state): State<ArcAppState>,
    cli: mid::HttpClient,
    Json(frm): Json<form::LoginForm>,
) -> Result<resp::JsonResp<String>> {
    frm.validate()?;

    if !turnstile::verify(&frm.captcha, &state.cfg.turnstile).await? {
        return Err(anyhow::anyhow!("人机验证失败").into());
    }

    let mut tx = state.pool.begin().await?;

    // 查找用户
    let user = match db::find(
        &mut *tx,
        &model::FindFilter {
            by: model::FindBy::Email(frm.email.clone()),
            status: None,
        },
    )
    .await
    {
        Ok(v) => match v {
            Some(v) => v,
            None => return Err(Error::NotFound("用户名/密码错误".into())),
        },
        Err(e) => {
            tx.rollback().await?;
            return Err(e.into());
        }
    };
    // 用户状态
    if !matches!(user.status, model::UserStatus::Actived) {
        return Err(anyhow::anyhow!("用户未激活").into());
    }
    // 验证密码
    if !util::pwd::verify(&frm.password, &user.password)? {
        return Err(anyhow::anyhow!("用户名/密码错误").into());
    }
    // 登录日志
    if let Err(e) = login_log::db::insert(
        &mut *tx,
        &login_log::model::LoginLog::new_user(user.id.clone(), &cli.ip, &cli.loc, &cli.user_agent),
    )
    .await
    {
        tx.rollback().await?;
        return Err(e.into());
    }

    // 生成JWT
    let data = jwt::ClaimsData::User(jwt::UserClaimsData {
        id: user.id,
        email: user.email,
        nickname: user.nickname,
    });
    let token = jwt::token(
        data,
        &state.cfg.user_jwt.secret,
        &state.cfg.user_jwt.sub,
        state.cfg.user_jwt.expire_duration,
        &cli.ip,
        &cli.user_agent,
    )?;

    tx.commit().await?;

    resp::ok(token.to_string())
}

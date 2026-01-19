use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email(message = "请输入正确的邮箱地址"))]
    #[validate(length(min = 6, max = 255, message = "请输入正确的邮箱地址"))]
    pub email: String,

    #[validate(length(min = 6, message = "密码至少6位"))]
    pub password: String,

    #[validate(length(min = 1, max = 30, message = "请输入正确的昵称"))]
    pub nickname: String,

    #[validate(length(min = 6, max = 6, message = "请输入6位验证码"))]
    pub valid_code: String,

    #[validate(length(min = 6, message = "请完成人机验证"))]
    pub captcha: String,

    #[validate(must_match(other = "password", message = "两次输入的密码不一致"))]
    pub re_password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "请输入正确的邮箱地址"))]
    #[validate(length(min = 6, max = 255, message = "请输入正确的邮箱地址"))]
    pub email: String,

    #[validate(length(min = 6, message = "密码至少6位"))]
    pub password: String,

    #[validate(length(min = 6, message = "请完成人机验证"))]
    pub captcha: String,
}

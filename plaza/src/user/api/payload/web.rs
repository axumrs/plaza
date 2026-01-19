use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordPayload {
    #[validate(length(min = 6, message = "密码至少6位"))]
    pub password: String,

    #[validate(length(min = 6, message = "新密码至少6位"))]
    pub new_password: String,

    #[validate(must_match(other = "new_password", message = "两次输入的密码不一致"))]
    pub re_password: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordPayload {
    #[validate(length(min = 6, max = 6, message = "请输入6位验证码"))]
    pub valid_code: String,

    #[validate(length(min = 6, message = "新密码至少6位"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "两次输入的密码不一致"))]
    pub re_password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProfilePayload {
    #[validate(length(min = 1, max = 30, message = "请输入正确的昵称"))]
    pub nickname: String,
}

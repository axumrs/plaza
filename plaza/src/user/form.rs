use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(email(message = "请输入正确的邮箱"))]
    #[validate(length(min = 6, message = "邮箱不能少于6位"))]
    pub email: String,

    #[validate(length(min = 6, message = "密码不能少于6位"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "两次密码不一致"))]
    pub re_password: String,

    #[validate(length(min = 6, message = "请完成人机验证"))]
    pub captcha: String,
}

use super::model::ValidCodeKind;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SendValidCodePayload {
    #[validate(email(message = "请输入正确的邮箱地址"))]
    #[validate(length(min = 6, max = 255, message = "请输入正确的邮箱地址"))]
    pub email: String,

    #[validate(length(min = 6, message = "请完成人机验证"))]
    pub captcha: String,

    pub kind: ValidCodeKind,
}

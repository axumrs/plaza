use crate::{Result, valid_code::model};

pub async fn invoke_valid_code_verify(
    code: impl Into<String>,
    email: impl Into<String>,
    kind: i32,
) -> Result<bool> {
    let _m = model::ValidCode {
        code: code.into(),
        email: email.into(),
        kind: kind.into(),
    };

    // TODO: 调用API
    return Ok(false);
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidCodeKind {
    Register,
    ResetPassword,
}

impl From<i32> for ValidCodeKind {
    fn from(v: i32) -> Self {
        match v {
            0 => ValidCodeKind::Register,
            1 => ValidCodeKind::ResetPassword,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidCode {
    pub code: String,
    pub kind: ValidCodeKind,
    pub email: String,
}

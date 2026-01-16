use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidCodeKind {
    Register,
    ResetPassword,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidCode {
    pub code: String,
    pub kind: ValidCodeKind,
    pub email: String,
}

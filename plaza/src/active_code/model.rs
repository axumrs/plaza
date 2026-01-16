use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ActiveCodeKind {
    Register,
    ResetPassword,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveCode {
    pub code: String,
    pub kind: ActiveCodeKind,
    pub email: String,
}

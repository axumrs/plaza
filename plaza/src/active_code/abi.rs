use super::model;
use crate::pb;

impl Into<pb::active_code::ActiveCodeKind> for model::ActiveCodeKind {
    fn into(self) -> pb::active_code::ActiveCodeKind {
        match self {
            model::ActiveCodeKind::Register => pb::active_code::ActiveCodeKind::Register,
            model::ActiveCodeKind::ResetPassword => pb::active_code::ActiveCodeKind::ResetPassword,
        }
    }
}

impl From<pb::active_code::ActiveCodeKind> for model::ActiveCodeKind {
    fn from(value: pb::active_code::ActiveCodeKind) -> Self {
        match value {
            pb::active_code::ActiveCodeKind::Register => model::ActiveCodeKind::Register,
            pb::active_code::ActiveCodeKind::ResetPassword => model::ActiveCodeKind::ResetPassword,
        }
    }
}

impl Into<pb::active_code::ActiveCode> for model::ActiveCode {
    fn into(self) -> pb::active_code::ActiveCode {
        let kind: pb::active_code::ActiveCodeKind = self.kind.into();

        pb::active_code::ActiveCode {
            code: self.code,
            kind: kind as i32,
            email: self.email,
        }
    }
}

impl From<pb::active_code::ActiveCode> for model::ActiveCode {
    fn from(value: pb::active_code::ActiveCode) -> Self {
        let kind: pb::active_code::ActiveCodeKind = value
            .kind
            .try_into()
            .unwrap_or(pb::active_code::ActiveCodeKind::Register);

        model::ActiveCode {
            code: value.code,
            kind: kind.into(),
            email: value.email,
        }
    }
}

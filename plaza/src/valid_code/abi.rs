use super::model;
use crate::pb;

impl Into<pb::valid_code::ValidCodeKind> for model::ValidCodeKind {
    fn into(self) -> pb::valid_code::ValidCodeKind {
        match self {
            model::ValidCodeKind::Register => pb::valid_code::ValidCodeKind::Register,
            model::ValidCodeKind::ResetPassword => pb::valid_code::ValidCodeKind::ResetPassword,
        }
    }
}

impl From<pb::valid_code::ValidCodeKind> for model::ValidCodeKind {
    fn from(value: pb::valid_code::ValidCodeKind) -> Self {
        match value {
            pb::valid_code::ValidCodeKind::Register => model::ValidCodeKind::Register,
            pb::valid_code::ValidCodeKind::ResetPassword => model::ValidCodeKind::ResetPassword,
        }
    }
}

impl Into<pb::valid_code::ValidCode> for model::ValidCode {
    fn into(self) -> pb::valid_code::ValidCode {
        let kind: pb::valid_code::ValidCodeKind = self.kind.into();

        pb::valid_code::ValidCode {
            code: self.code,
            kind: kind as i32,
            email: self.email,
        }
    }
}

impl From<pb::valid_code::ValidCode> for model::ValidCode {
    fn from(value: pb::valid_code::ValidCode) -> Self {
        let kind: pb::valid_code::ValidCodeKind = value
            .kind
            .try_into()
            .unwrap_or(pb::valid_code::ValidCodeKind::Register);

        model::ValidCode {
            code: value.code,
            kind: kind.into(),
            email: value.email,
        }
    }
}

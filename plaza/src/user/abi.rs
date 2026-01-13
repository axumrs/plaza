use crate::{
    pb, types,
    user::model::{User, UserStatus},
};

impl From<pb::user::UserStatus> for UserStatus {
    fn from(value: pb::user::UserStatus) -> Self {
        match value {
            pb::user::UserStatus::Pending => UserStatus::Pending,
            pb::user::UserStatus::Actived => UserStatus::Actived,
            pb::user::UserStatus::Freezed => UserStatus::Freezed,
        }
    }
}

impl Into<pb::user::UserStatus> for UserStatus {
    fn into(self) -> pb::user::UserStatus {
        match self {
            UserStatus::Pending => pb::user::UserStatus::Pending,
            UserStatus::Actived => pb::user::UserStatus::Actived,
            UserStatus::Freezed => pb::user::UserStatus::Freezed,
        }
    }
}

impl From<pb::user::User> for User {
    fn from(value: pb::user::User) -> Self {
        Self {
            status: value.status().into(),
            id: value.id,
            email: value.email,
            password: value.password,
            nickname: value.nickname,
            created_at: types::prost2chrono(&value.created_at),
        }
    }
}

impl Into<pb::user::User> for User {
    fn into(self) -> pb::user::User {
        let status: pb::user::UserStatus = self.status.into();
        pb::user::User {
            status: status.into(),
            id: self.id,
            email: self.email,
            password: self.password,
            nickname: self.nickname,
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

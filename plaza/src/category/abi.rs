use super::model::{Category, CategoryLevel};
use crate::{pb, types};

impl From<pb::category::CategoryLevel> for CategoryLevel {
    fn from(level: pb::category::CategoryLevel) -> Self {
        match level {
            pb::category::CategoryLevel::Unspecified => CategoryLevel::Unspecified,
            pb::category::CategoryLevel::Level1 => CategoryLevel::Level1,
            pb::category::CategoryLevel::Level2 => CategoryLevel::Level2,
            pb::category::CategoryLevel::Level3 => CategoryLevel::Level3,
        }
    }
}

impl Into<pb::category::CategoryLevel> for CategoryLevel {
    fn into(self) -> pb::category::CategoryLevel {
        match self {
            CategoryLevel::Unspecified => pb::category::CategoryLevel::Unspecified,
            CategoryLevel::Level1 => pb::category::CategoryLevel::Level1,
            CategoryLevel::Level2 => pb::category::CategoryLevel::Level2,
            CategoryLevel::Level3 => pb::category::CategoryLevel::Level3,
        }
    }
}

impl From<pb::category::Category> for Category {
    fn from(c: pb::category::Category) -> Self {
        let level = CategoryLevel::from(c.level());

        Self {
            id: c.id,
            name: c.name,
            parent: c.parent,
            path: c.path,
            level,
            created_at: types::prost2chrono(&c.created_at),
            security_deposit: c.security_deposit,
        }
    }
}

impl Into<pb::category::Category> for Category {
    fn into(self) -> pb::category::Category {
        let level: pb::category::CategoryLevel = self.level.into();

        pb::category::Category {
            id: self.id,
            name: self.name,
            parent: self.parent,
            path: self.path,
            level: level.into(),
            created_at: types::chrono2prost(self.created_at),
            security_deposit: self.security_deposit,
        }
    }
}

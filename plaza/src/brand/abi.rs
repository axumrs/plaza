use crate::{pb, types};

use super::model::Brand;

impl From<pb::brand::Brand> for Brand {
    fn from(value: pb::brand::Brand) -> Self {
        Self {
            id: value.id,
            name: value.name,
            logo: value.logo,
            created_at: types::prost2chrono(&value.created_at),
        }
    }
}

impl Into<pb::brand::Brand> for Brand {
    fn into(self) -> pb::brand::Brand {
        pb::brand::Brand {
            id: self.id,
            name: self.name,
            logo: self.logo,
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

use super::model::Address;
use crate::{pb, types};

impl From<pb::address::Address> for Address {
    fn from(v: pb::address::Address) -> Self {
        Self {
            id: v.id,
            user_id: v.user_id,
            is_default: v.is_default,
            consignee: v.consignee,
            phone: v.phone,
            address: v.address,
            province: v.province,
            city: v.city,
            county: v.county,
            post_code: v.post_code,
            created_at: types::prost2chrono(&v.created_at),
        }
    }
}

impl Into<pb::address::Address> for Address {
    fn into(self) -> pb::address::Address {
        pb::address::Address {
            id: self.id,
            user_id: self.user_id,
            is_default: self.is_default,
            consignee: self.consignee,
            phone: self.phone,
            address: self.address,
            province: self.province,
            city: self.city,
            county: self.county,
            post_code: self.post_code,
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

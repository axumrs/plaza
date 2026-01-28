use crate::{pb, types};

use super::model::{Shop, ShopAudit, ShopKind, ShopMeta};

impl From<pb::shop::ShopKind> for ShopKind {
    fn from(value: pb::shop::ShopKind) -> Self {
        match value {
            pb::shop::ShopKind::Standard => ShopKind::Standard,
            pb::shop::ShopKind::OfficialFlagship => ShopKind::OfficialFlagship,
            pb::shop::ShopKind::Flagship => ShopKind::Flagship,
            pb::shop::ShopKind::MallFlagship => ShopKind::MallFlagship,
            pb::shop::ShopKind::Specialty => ShopKind::Specialty,
            pb::shop::ShopKind::Franchise => ShopKind::Franchise,
        }
    }
}

impl Into<pb::shop::ShopKind> for ShopKind {
    fn into(self) -> pb::shop::ShopKind {
        match self {
            ShopKind::Standard => pb::shop::ShopKind::Standard,
            ShopKind::OfficialFlagship => pb::shop::ShopKind::OfficialFlagship,
            ShopKind::Flagship => pb::shop::ShopKind::Flagship,
            ShopKind::MallFlagship => pb::shop::ShopKind::MallFlagship,
            ShopKind::Specialty => pb::shop::ShopKind::Specialty,
            ShopKind::Franchise => pb::shop::ShopKind::Franchise,
        }
    }
}

impl From<i32> for ShopKind {
    fn from(value: i32) -> Self {
        match value {
            0 => ShopKind::Standard,
            1 => ShopKind::OfficialFlagship,
            2 => ShopKind::Flagship,
            3 => ShopKind::MallFlagship,
            4 => ShopKind::Specialty,
            5 => ShopKind::Franchise,
            _ => ShopKind::Standard,
        }
    }
}

impl Into<i32> for ShopKind {
    fn into(self) -> i32 {
        match self {
            ShopKind::Standard => 0,
            ShopKind::OfficialFlagship => 1,
            ShopKind::Flagship => 2,
            ShopKind::MallFlagship => 3,
            ShopKind::Specialty => 4,
            ShopKind::Franchise => 5,
        }
    }
}

impl From<pb::shop::ShopMeta> for ShopMeta {
    fn from(v: pb::shop::ShopMeta) -> Self {
        let expiry_date = match v.expiry_date {
            Some(v) => (types::prost2chrono(&v.start), types::prost2chrono(&v.end)),
            None => (types::chrono_now(), types::chrono_now()),
        };

        Self {
            licensor: v.licensor,
            brand_name: v.brand_name,
            licensee: v.licensee,
            expiry_date,
            authorization_date: types::prost2chrono(&v.authorization_date),
            proof: v.proof,
        }
    }
}

impl Into<pb::shop::ShopMeta> for ShopMeta {
    fn into(self) -> pb::shop::ShopMeta {
        pb::shop::ShopMeta {
            licensor: self.licensor,
            brand_name: self.brand_name,
            licensee: self.licensee,
            expiry_date: Some(pb::range::DateRange {
                start: types::chrono2prost(self.expiry_date.0),
                end: types::chrono2prost(self.expiry_date.1),
            }),
            authorization_date: types::chrono2prost(self.authorization_date),
            proof: self.proof,
        }
    }
}

impl From<pb::shop::Shop> for Shop {
    fn from(v: pb::shop::Shop) -> Self {
        Self {
            id: v.id,
            merchant_id: v.merchant_id,
            category_id: v.category_id,
            deposit: v.deposit,
            name: v.name,
            kind: v.kind.into(),
            description: v.description,
            created_at: types::prost2chrono(&v.created_at),
            status: v.status.into(),
            meta: sqlx::types::Json(v.meta.unwrap_or_default().into()),
            is_platform_self_operated: v.is_platform_self_operated,
        }
    }
}

impl Into<pb::shop::Shop> for Shop {
    fn into(self) -> pb::shop::Shop {
        pb::shop::Shop {
            id: self.id,
            merchant_id: self.merchant_id,
            category_id: self.category_id,
            deposit: self.deposit,
            name: self.name,
            kind: self.kind.into(),
            description: self.description,
            created_at: types::chrono2prost(self.created_at),
            status: self.status.into(),
            meta: Some(self.meta.0.into()),
            is_platform_self_operated: self.is_platform_self_operated,
        }
    }
}

impl From<pb::shop::ShopAudit> for ShopAudit {
    fn from(v: pb::shop::ShopAudit) -> Self {
        Self {
            id: v.id,
            merchant_id: v.merchant_id,
            shop_id: v.shop_id,
            auditor_id: v.auditor_id,
            audit_status: v.audit_status.into(),
            audit_comments: v.audit_comments,
            audit_date: types::prost2chrono(&v.audit_date),
        }
    }
}

impl Into<pb::shop::ShopAudit> for ShopAudit {
    fn into(self) -> pb::shop::ShopAudit {
        pb::shop::ShopAudit {
            id: self.id,
            merchant_id: self.merchant_id,
            shop_id: self.shop_id,
            auditor_id: self.auditor_id,
            audit_status: self.audit_status.into(),
            audit_comments: self.audit_comments,
            audit_date: types::chrono2prost(self.audit_date),
        }
    }
}

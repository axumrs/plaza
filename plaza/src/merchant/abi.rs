use super::model::{Merchant, MerchantAccount, MerchantAudit, MerchantKind, MerchantMeta};
use crate::{pb, types};

impl From<pb::merchant::MerchantKind> for MerchantKind {
    fn from(value: pb::merchant::MerchantKind) -> Self {
        match value {
            pb::merchant::MerchantKind::Enterprise => MerchantKind::Enterprise,
            pb::merchant::MerchantKind::SoleProprietorship => MerchantKind::SoleProprietorship,
            pb::merchant::MerchantKind::Individual => MerchantKind::Individual,
        }
    }
}

impl Into<pb::merchant::MerchantKind> for MerchantKind {
    fn into(self) -> pb::merchant::MerchantKind {
        match self {
            MerchantKind::Enterprise => pb::merchant::MerchantKind::Enterprise,
            MerchantKind::SoleProprietorship => pb::merchant::MerchantKind::SoleProprietorship,
            MerchantKind::Individual => pb::merchant::MerchantKind::Individual,
        }
    }
}

impl From<i32> for MerchantKind {
    fn from(value: i32) -> Self {
        match value {
            0 => MerchantKind::Enterprise,
            1 => MerchantKind::SoleProprietorship,
            2 => MerchantKind::Individual,
            _ => MerchantKind::default(),
        }
    }
}

impl From<pb::merchant::ImagePair> for (String, String) {
    fn from(value: pb::merchant::ImagePair) -> Self {
        (value.front_url, value.back_url)
    }
}

impl Into<pb::merchant::ImagePair> for (String, String) {
    fn into(self) -> pb::merchant::ImagePair {
        pb::merchant::ImagePair {
            front_url: self.0,
            back_url: self.1,
        }
    }
}

impl From<pb::merchant::MerchantMeta> for MerchantMeta {
    fn from(value: pb::merchant::MerchantMeta) -> Self {
        let legal_representative_id_expiry_date = match value.legal_representative_id_expiry_date {
            Some(v) => Some((types::prost2chrono(&v.start), types::prost2chrono(&v.end))),
            None => None,
        };

        let super_admin_id_expiry_date = match value.super_admin_id_expiry_date {
            Some(v) => Some((types::prost2chrono(&v.start), types::prost2chrono(&v.end))),
            None => None,
        };
        let legal_representative_id_pic = match value.legal_representative_id_pic {
            Some(v) => Some(v.into()),
            None => None,
        };
        let super_admin_id_pic = match value.super_admin_id_pic {
            Some(v) => Some(v.into()),
            None => None,
        };

        Self {
            business_license: value.business_license,
            business_license_number: value.business_license_number,
            name: value.name,
            short_name: value.short_name,
            legal_representative_name: value.legal_representative_name,
            business_license_expiry_date: Some(types::prost2chrono(
                &value.business_license_expiry_date,
            )),
            registered_address: value.registered_address,
            legal_representative_id_pic,
            legal_representative_id_name: value.legal_representative_id_name,
            legal_representative_id_number: value.legal_representative_id_number,
            legal_representative_id_expiry_date,
            legal_representative_id_address: value.legal_representative_id_address,
            account_address: value.account_address,
            super_admin_is_legal_representative: value.super_admin_is_legal_representative,
            super_admin_id_pic,
            super_admin_id_expiry_date,
            super_admin_id_name: value.super_admin_id_name,
            super_admin_id_number: value.super_admin_id_number,
            super_admin_phone: value.super_admin_phone,
            super_admin_email: value.super_admin_email,
            special_qualification_pics: Some(value.special_qualification_pics),
            supplementary_material_pics: Some(value.supplementary_material_pics),
            supplementary_explain: value.supplementary_explain,
        }
    }
}

impl Into<pb::merchant::MerchantMeta> for MerchantMeta {
    fn into(self) -> pb::merchant::MerchantMeta {
        let business_license_expiry_date = match self.business_license_expiry_date {
            Some(v) => types::chrono2prost(v),
            None => None,
        };
        let legal_representative_id_expiry_date = match self.legal_representative_id_expiry_date {
            Some(v) => Some(pb::range::DateRange {
                start: types::chrono2prost(v.0),
                end: types::chrono2prost(v.1),
            }),
            None => None,
        };
        let super_admin_id_expiry_date = match self.super_admin_id_expiry_date {
            Some(v) => Some(pb::range::DateRange {
                start: types::chrono2prost(v.0),
                end: types::chrono2prost(v.1),
            }),
            None => None,
        };
        pb::merchant::MerchantMeta {
            business_license: self.business_license,
            business_license_number: self.business_license_number,
            name: self.name,
            short_name: self.short_name,
            legal_representative_name: self.legal_representative_name,
            business_license_expiry_date,
            registered_address: self.registered_address,
            legal_representative_id_pic: self.legal_representative_id_pic.map(Into::into),
            legal_representative_id_name: self.legal_representative_id_name,
            legal_representative_id_number: self.legal_representative_id_number,
            legal_representative_id_expiry_date,
            legal_representative_id_address: self.legal_representative_id_address,
            account_address: self.account_address,
            super_admin_is_legal_representative: self.super_admin_is_legal_representative,
            super_admin_id_pic: self.super_admin_id_pic.map(Into::into),
            super_admin_id_expiry_date,
            super_admin_id_name: self.super_admin_id_name,
            super_admin_id_number: self.super_admin_id_number,
            super_admin_phone: self.super_admin_phone,
            super_admin_email: self.super_admin_email,
            special_qualification_pics: self.special_qualification_pics.unwrap_or(vec![]),
            supplementary_material_pics: self.supplementary_material_pics.unwrap_or(vec![]),
            supplementary_explain: self.supplementary_explain,
        }
    }
}

impl From<pb::merchant::Merchant> for Merchant {
    fn from(value: pb::merchant::Merchant) -> Self {
        let meta = match value.meta {
            Some(meta) => meta.into(),
            None => return Merchant::default(),
        };
        let status = value.status;
        let kind = value.kind;

        Merchant {
            kind: kind.into(),
            status: status.into(),
            id: value.id,
            name: value.name,
            short_name: value.short_name,
            created_at: types::prost2chrono(&value.created_at),
            meta: sqlx::types::Json(meta),
        }
    }
}

impl Into<pb::merchant::Merchant> for Merchant {
    fn into(self) -> pb::merchant::Merchant {
        let status: pb::audit::AuditStatus = self.status.into();
        let kind: pb::merchant::MerchantKind = self.kind.into();
        pb::merchant::Merchant {
            id: self.id,
            name: self.name,
            short_name: self.short_name,
            status: status as i32,
            kind: kind as i32,
            created_at: types::chrono2prost(self.created_at),
            meta: Some(self.meta.0.into()),
        }
    }
}

impl From<pb::merchant::MerchantAccount> for MerchantAccount {
    fn from(value: pb::merchant::MerchantAccount) -> Self {
        MerchantAccount {
            id: value.id,
            merchant_id: value.merchant_id,
            email: value.email,
            password: value.password,
            nickname: value.nickname,
            is_super: value.is_super,
            permission: value.permission,
            created_at: types::prost2chrono(&value.created_at),
        }
    }
}

impl Into<pb::merchant::MerchantAccount> for MerchantAccount {
    fn into(self) -> pb::merchant::MerchantAccount {
        pb::merchant::MerchantAccount {
            id: self.id,
            merchant_id: self.merchant_id,
            email: self.email,
            password: self.password,
            nickname: self.nickname,
            is_super: self.is_super,
            permission: self.permission,
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

impl From<pb::merchant::MerchantAudit> for MerchantAudit {
    fn from(value: pb::merchant::MerchantAudit) -> Self {
        MerchantAudit {
            id: value.id,
            merchant_id: value.merchant_id,
            auditor_id: value.auditor_id,
            audit_status: value.audit_status.into(),
            audit_comments: value.audit_comments,
            audit_date: types::prost2chrono(&value.audit_date),
        }
    }
}

impl Into<pb::merchant::MerchantAudit> for MerchantAudit {
    fn into(self) -> pb::merchant::MerchantAudit {
        pb::merchant::MerchantAudit {
            id: self.id,
            merchant_id: self.merchant_id,
            auditor_id: self.auditor_id,
            audit_status: self.audit_status as i32,
            audit_comments: self.audit_comments,
            audit_date: types::chrono2prost(self.audit_date),
        }
    }
}

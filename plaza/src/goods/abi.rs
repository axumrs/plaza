use super::model::{
    Goods, GoodsArgument, GoodsAttr, GoodsComment, GoodsInfo, GoodsSkuMeta, GoodsStarLevel,
    GoodsStatus,
};
use crate::{pb, types};

impl From<pb::goods::GoodsStatus> for GoodsStatus {
    fn from(v: pb::goods::GoodsStatus) -> Self {
        match v {
            pb::goods::GoodsStatus::Available => GoodsStatus::Available,
            pb::goods::GoodsStatus::Unavailable => GoodsStatus::Unavailable,
        }
    }
}

impl Into<pb::goods::GoodsStatus> for GoodsStatus {
    fn into(self) -> pb::goods::GoodsStatus {
        match self {
            GoodsStatus::Available => pb::goods::GoodsStatus::Available,
            GoodsStatus::Unavailable => pb::goods::GoodsStatus::Unavailable,
        }
    }
}

impl From<i32> for GoodsStatus {
    fn from(v: i32) -> Self {
        match v {
            0 => GoodsStatus::Available,
            1 => GoodsStatus::Unavailable,
            _ => GoodsStatus::default(),
        }
    }
}

impl Into<i32> for GoodsStatus {
    fn into(self) -> i32 {
        match self {
            GoodsStatus::Available => 0,
            GoodsStatus::Unavailable => 1,
        }
    }
}

impl From<pb::goods::GoodsSkuMeta> for GoodsSkuMeta {
    fn from(v: pb::goods::GoodsSkuMeta) -> Self {
        GoodsSkuMeta {
            name: v.name,
            items: v.items,
        }
    }
}

impl Into<pb::goods::GoodsSkuMeta> for GoodsSkuMeta {
    fn into(self) -> pb::goods::GoodsSkuMeta {
        pb::goods::GoodsSkuMeta {
            name: self.name,
            items: self.items,
        }
    }
}

impl From<pb::goods::GoodsArgument> for GoodsArgument {
    fn from(v: pb::goods::GoodsArgument) -> Self {
        GoodsArgument {
            name: v.name,
            value: v.value,
        }
    }
}

impl Into<pb::goods::GoodsArgument> for GoodsArgument {
    fn into(self) -> pb::goods::GoodsArgument {
        pb::goods::GoodsArgument {
            name: self.name,
            value: self.value,
        }
    }
}

impl From<pb::goods::Goods> for Goods {
    fn from(v: pb::goods::Goods) -> Self {
        Self {
            id: v.id,
            shop_id: v.shop_id,
            is_vir: v.is_vir,
            category_id: v.category_id,
            name: v.name,
            images: v.images,
            status: v.status.into(),
            detail: v.detail,
            comment_need_audit: v.comment_need_audit,
            service_guarantee: v.service_guarantee,
            tags: v.tags,
            arguments: v.arguments.into_iter().map(Into::into).collect(),
            stock: v.stock,
            sales: v.sales,
            sku_meta: v.sku_meta.into_iter().map(Into::into).collect(),
            fare: v.fare,
            recommendations: v.recommendations,
            created_at: types::prost2chrono(&v.created_at),
            updated_at: types::prost2chrono(&v.updated_at),
        }
    }
}

impl Into<pb::goods::Goods> for Goods {
    fn into(self) -> pb::goods::Goods {
        pb::goods::Goods {
            id: self.id,
            shop_id: self.shop_id,
            is_vir: self.is_vir,
            category_id: self.category_id,
            name: self.name,
            images: self.images,
            status: self.status.into(),
            detail: self.detail,
            comment_need_audit: self.comment_need_audit,
            service_guarantee: self.service_guarantee,
            tags: self.tags,
            arguments: self.arguments.into_iter().map(Into::into).collect(),
            stock: self.stock,
            sales: self.sales,
            sku_meta: self.sku_meta.into_iter().map(Into::into).collect(),
            fare: self.fare,
            recommendations: self.recommendations,
            created_at: types::chrono2prost(self.created_at),
            updated_at: types::chrono2prost(self.updated_at),
        }
    }
}

impl From<pb::goods::GoodsAttr> for GoodsAttr {
    fn from(v: pb::goods::GoodsAttr) -> Self {
        Self {
            id: v.id,
            goods_id: v.goods_id,
            sku_arr: v.sku_arr,
            stock: v.stock,
            price: v.price,
            sales: v.sales,
            code: v.code,
            bar_code: v.bar_code,
            volume: v.volume,
            weight: v.weight,
            ver: v.ver,
        }
    }
}

impl Into<pb::goods::GoodsAttr> for GoodsAttr {
    fn into(self) -> pb::goods::GoodsAttr {
        pb::goods::GoodsAttr {
            id: self.id,
            goods_id: self.goods_id,
            sku_arr: self.sku_arr,
            stock: self.stock,
            price: self.price,
            sales: self.sales,
            code: self.code,
            bar_code: self.bar_code,
            volume: self.volume,
            weight: self.weight,
            ver: self.ver,
        }
    }
}

impl From<pb::goods::GoodsComment> for GoodsComment {
    fn from(v: pb::goods::GoodsComment) -> Self {
        Self {
            id: v.id,
            goods_id: v.goods_id,
            goods_full_name: v.goods_full_name,
            is_self: v.is_self,
            user_id: v.user_id,
            user_avatar: v.user_avatar,
            user_nickname: v.user_nickname,
            content: v.content,
            goods_star: v.goods_star,
            service_star: v.service_star,
            images: v.images,
            created_at: types::prost2chrono(&v.created_at),
        }
    }
}

impl Into<pb::goods::GoodsComment> for GoodsComment {
    fn into(self) -> pb::goods::GoodsComment {
        pb::goods::GoodsComment {
            id: self.id,
            goods_id: self.goods_id,
            goods_full_name: self.goods_full_name,
            is_self: self.is_self,
            user_id: self.user_id,
            user_avatar: self.user_avatar,
            user_nickname: self.user_nickname,
            content: self.content,
            goods_star: self.goods_star,
            service_star: self.service_star,
            images: self.images,
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

impl From<pb::goods::GoodsInfo> for GoodsInfo {
    fn from(v: pb::goods::GoodsInfo) -> Self {
        Self {
            goods: v.goods.unwrap_or_default().into(),
            metas: v.metas.into_iter().map(Into::into).collect(),
            attrs: v.attrs.into_iter().map(Into::into).collect(),
        }
    }
}

impl Into<pb::goods::GoodsInfo> for GoodsInfo {
    fn into(self) -> pb::goods::GoodsInfo {
        pb::goods::GoodsInfo {
            goods: Some(self.goods.into()),
            metas: self.metas.into_iter().map(Into::into).collect(),
            attrs: self.attrs.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<pb::goods::GoodsStarLevel> for GoodsStarLevel {
    fn from(v: pb::goods::GoodsStarLevel) -> Self {
        match v {
            pb::goods::GoodsStarLevel::Positive => GoodsStarLevel::Positive,
            pb::goods::GoodsStarLevel::Neutral => GoodsStarLevel::Neutral,
            pb::goods::GoodsStarLevel::Negative => GoodsStarLevel::Negative,
        }
    }
}

impl Into<pb::goods::GoodsStarLevel> for GoodsStarLevel {
    fn into(self) -> pb::goods::GoodsStarLevel {
        match self {
            GoodsStarLevel::Positive => pb::goods::GoodsStarLevel::Positive,
            GoodsStarLevel::Neutral => pb::goods::GoodsStarLevel::Neutral,
            GoodsStarLevel::Negative => pb::goods::GoodsStarLevel::Negative,
        }
    }
}

impl From<i32> for GoodsStarLevel {
    fn from(v: i32) -> Self {
        match v {
            0 => GoodsStarLevel::Positive,
            1 => GoodsStarLevel::Neutral,
            2 => GoodsStarLevel::Negative,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for GoodsStarLevel {
    fn into(self) -> i32 {
        match self {
            GoodsStarLevel::Positive => 0,
            GoodsStarLevel::Neutral => 1,
            GoodsStarLevel::Negative => 2,
        }
    }
}

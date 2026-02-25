use crate::{pb, types};

use super::model::{Cart, CartItem};

impl From<pb::cart::CartItem> for CartItem {
    fn from(v: pb::cart::CartItem) -> Self {
        Self {
            goods_id: v.goods_id,
            sku_id: v.sku_id,
            quantity: v.quantity,
            price: v.price,
            real_price: v.real_price,
            goods_full_name: v.goods_full_name,
            goods_image: v.goods_image,
        }
    }
}

impl Into<pb::cart::CartItem> for CartItem {
    fn into(self) -> pb::cart::CartItem {
        pb::cart::CartItem {
            goods_id: self.goods_id,
            sku_id: self.sku_id,
            quantity: self.quantity,
            price: self.price,
            real_price: self.real_price,
            goods_full_name: self.goods_full_name,
            goods_image: self.goods_image,
        }
    }
}

impl From<pb::cart::Cart> for Cart {
    fn from(v: pb::cart::Cart) -> Self {
        Self {
            user_id: v.user_id,
            shop_id: v.shop_id,
            items: v.items.into_iter().map(Into::into).collect(),
            created_at: types::prost2chrono(&v.created_at),
        }
    }
}

impl Into<pb::cart::Cart> for Cart {
    fn into(self) -> pb::cart::Cart {
        pb::cart::Cart {
            user_id: self.user_id,
            shop_id: self.shop_id,
            items: self.items.into_iter().map(Into::into).collect(),
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

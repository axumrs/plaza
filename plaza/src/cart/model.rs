use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::types;

/// 购物车项 (对应 PostgreSQL 的 cart_item 类型)
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "cart_item")]
pub struct CartItem {
    pub goods_id: String,
    pub sku_id: String,
    pub quantity: i32,
    pub price: i64,
    pub real_price: i64,
    pub goods_full_name: String,
    pub goods_image: String,
}

/// 购物车实体 (对应 carts 表)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub user_id: String,
    pub shop_id: String,
    /// sqlx 支持直接映射复合类型数组为 Vec<T>
    pub items: Vec<CartItem>,
    pub created_at: types::Timestamp,
}

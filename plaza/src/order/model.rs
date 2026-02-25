use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::{
    cart::model::CartItem,
    goods::model::{GoodsArgument, GoodsSkuMeta, GoodsStatus},
    types,
};

// --- 枚举定义 ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "order_status", rename_all = "PascalCase")]
pub enum OrderStatus {
    #[default]
    PendingPayment,
    PendingShipment,
    Shipped,
    Completed,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "payment_method", rename_all = "PascalCase")]
pub enum PaymentMethod {
    #[default]
    Web3,
    WeChatPay,
    Alipay,
    Balance,
    CreditCard,
}

// --- 复合类型快照定义 ---

#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "order_item_snap")]
pub struct OrderItemSnap {
    pub goods_id: String,
    pub shop_id: String,
    pub is_vir: bool,
    pub category_id: String,
    pub name: String,
    pub images: Vec<String>,
    pub status: GoodsStatus,
    pub detail: String,
    pub comment_need_audit: bool,
    pub service_guarantee: Vec<String>,
    pub tags: Vec<String>,
    pub arguments: Vec<GoodsArgument>,
    pub sku_meta: Vec<GoodsSkuMeta>,
    pub fare: i32,
    pub recommendations: Vec<String>,
    pub sku_id: String,
    pub sku_arr: Vec<String>,
    pub goods_full_name: String,
    pub goods_image: String,
    pub price: i64,
    pub real_price: i64,
}

// --- 订单主表定义 ---

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub shop_id: String,

    pub total_amount: i64,
    pub pay_amount: i64,
    pub freight_amount: i64,

    pub status: OrderStatus,
    pub pay_method: PaymentMethod,

    pub consignee_name: String,
    pub consignee_phone: String,
    pub consignee_address: String,

    // Postgres 复合类型数组直接映射为 Vec
    pub items: Vec<OrderItemSnap>,
    pub cart_items: Vec<CartItem>,

    pub pay_at: types::Timestamp,
    pub ship_at: types::Timestamp,
    pub completed_at: types::Timestamp,
    pub created_at: types::Timestamp,
}

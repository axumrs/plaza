use crate::{pb, types};

use super::model::{Order, OrderItemSnap, OrderStatus, PaymentMethod};

impl From<pb::order::OrderStatus> for OrderStatus {
    fn from(v: pb::order::OrderStatus) -> Self {
        match v {
            pb::order::OrderStatus::PendingPayment => OrderStatus::PendingPayment,
            pb::order::OrderStatus::PendingShipment => OrderStatus::PendingShipment,
            pb::order::OrderStatus::Shipped => OrderStatus::Shipped,
            pb::order::OrderStatus::Completed => OrderStatus::Completed,
            pb::order::OrderStatus::Cancelled => OrderStatus::Cancelled,
            pb::order::OrderStatus::Refunded => OrderStatus::Refunded,
        }
    }
}

impl Into<pb::order::OrderStatus> for OrderStatus {
    fn into(self) -> pb::order::OrderStatus {
        match self {
            OrderStatus::PendingPayment => pb::order::OrderStatus::PendingPayment,
            OrderStatus::PendingShipment => pb::order::OrderStatus::PendingShipment,
            OrderStatus::Shipped => pb::order::OrderStatus::Shipped,
            OrderStatus::Completed => pb::order::OrderStatus::Completed,
            OrderStatus::Cancelled => pb::order::OrderStatus::Cancelled,
            OrderStatus::Refunded => pb::order::OrderStatus::Refunded,
        }
    }
}

impl From<i32> for OrderStatus {
    fn from(v: i32) -> Self {
        match v {
            0 => OrderStatus::PendingPayment,
            1 => OrderStatus::PendingShipment,
            2 => OrderStatus::Shipped,
            3 => OrderStatus::Completed,
            4 => OrderStatus::Cancelled,
            5 => OrderStatus::Refunded,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for OrderStatus {
    fn into(self) -> i32 {
        match self {
            OrderStatus::PendingPayment => 0,
            OrderStatus::PendingShipment => 1,
            OrderStatus::Shipped => 2,
            OrderStatus::Completed => 3,
            OrderStatus::Cancelled => 4,
            OrderStatus::Refunded => 5,
        }
    }
}

impl From<pb::order::PaymentMethod> for PaymentMethod {
    fn from(v: pb::order::PaymentMethod) -> Self {
        match v {
            pb::order::PaymentMethod::Web3 => Self::Web3,
            pb::order::PaymentMethod::WechatPay => Self::WeChatPay,
            pb::order::PaymentMethod::Alipay => Self::Alipay,
            pb::order::PaymentMethod::Balance => Self::Balance,
            pb::order::PaymentMethod::CreditCard => Self::CreditCard,
        }
    }
}

impl Into<pb::order::PaymentMethod> for PaymentMethod {
    fn into(self) -> pb::order::PaymentMethod {
        match self {
            PaymentMethod::Web3 => pb::order::PaymentMethod::Web3,
            PaymentMethod::WeChatPay => pb::order::PaymentMethod::WechatPay,
            PaymentMethod::Alipay => pb::order::PaymentMethod::Alipay,
            PaymentMethod::Balance => pb::order::PaymentMethod::Balance,
            PaymentMethod::CreditCard => pb::order::PaymentMethod::CreditCard,
        }
    }
}

impl From<i32> for PaymentMethod {
    fn from(value: i32) -> Self {
        match value {
            0 => PaymentMethod::Web3,
            1 => PaymentMethod::WeChatPay,
            2 => PaymentMethod::Alipay,
            3 => PaymentMethod::Balance,
            4 => PaymentMethod::CreditCard,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for PaymentMethod {
    fn into(self) -> i32 {
        match self {
            PaymentMethod::Web3 => 0,
            PaymentMethod::WeChatPay => 1,
            PaymentMethod::Alipay => 2,
            PaymentMethod::Balance => 3,
            PaymentMethod::CreditCard => 4,
        }
    }
}

impl From<pb::order::OrderItemSnap> for OrderItemSnap {
    fn from(v: pb::order::OrderItemSnap) -> Self {
        Self {
            goods_id: v.goods_id,
            shop_id: v.shop_id,
            name: v.name,
            is_vir: v.is_vir,
            category_id: v.category_id,
            images: v.images,
            status: v.status.into(),
            detail: v.detail,
            comment_need_audit: v.comment_need_audit,
            service_guarantee: v.service_guarantee,
            tags: v.tags,
            arguments: v.arguments.into_iter().map(Into::into).collect(),
            sku_meta: v.sku_meta.into_iter().map(Into::into).collect(),
            fare: v.fare,
            recommendations: v.recommendations,
            sku_id: v.sku_id,
            sku_arr: v.sku_arr,
            goods_full_name: v.goods_full_name,
            goods_image: v.goods_image,
            price: v.price,
            real_price: v.real_price,
        }
    }
}

impl Into<pb::order::OrderItemSnap> for OrderItemSnap {
    fn into(self) -> pb::order::OrderItemSnap {
        pb::order::OrderItemSnap {
            goods_id: self.goods_id,
            shop_id: self.shop_id,
            name: self.name,
            is_vir: self.is_vir,
            category_id: self.category_id,
            images: self.images,
            status: self.status.into(),
            detail: self.detail,
            comment_need_audit: self.comment_need_audit,
            service_guarantee: self.service_guarantee,
            tags: self.tags,
            arguments: self.arguments.into_iter().map(Into::into).collect(),
            sku_meta: self.sku_meta.into_iter().map(Into::into).collect(),
            fare: self.fare,
            recommendations: self.recommendations,
            sku_id: self.sku_id,
            sku_arr: self.sku_arr,
            goods_full_name: self.goods_full_name,
            goods_image: self.goods_image,
            price: self.price,
            real_price: self.real_price,
        }
    }
}

impl From<pb::order::Order> for Order {
    fn from(v: pb::order::Order) -> Self {
        Self {
            id: v.id,
            user_id: v.user_id,
            shop_id: v.shop_id,
            total_amount: v.total_amount,
            pay_amount: v.pay_amount,
            freight_amount: v.freight_amount,
            status: v.status.into(),
            pay_method: v.pay_method.into(),
            consignee_name: v.consignee_name,
            consignee_phone: v.consignee_phone,
            consignee_address: v.consignee_address,
            items: v.items.into_iter().map(Into::into).collect(),
            cart_items: v.cart_items.into_iter().map(Into::into).collect(),
            pay_at: types::prost2chrono(&v.pay_at),
            ship_at: types::prost2chrono(&v.ship_at),
            completed_at: types::prost2chrono(&v.completed_at),
            created_at: types::prost2chrono(&v.created_at),
        }
    }
}

impl Into<pb::order::Order> for Order {
    fn into(self) -> pb::order::Order {
        pb::order::Order {
            id: self.id,
            user_id: self.user_id,
            shop_id: self.shop_id,
            total_amount: self.total_amount,
            pay_amount: self.pay_amount,
            freight_amount: self.freight_amount,
            status: self.status.into(),
            pay_method: self.pay_method.into(),
            consignee_name: self.consignee_name,
            consignee_phone: self.consignee_phone,
            consignee_address: self.consignee_address,
            items: self.items.into_iter().map(Into::into).collect(),
            cart_items: self.cart_items.into_iter().map(Into::into).collect(),
            pay_at: types::chrono2prost(self.pay_at),
            ship_at: types::chrono2prost(self.ship_at),
            completed_at: types::chrono2prost(self.completed_at),
            created_at: types::chrono2prost(self.created_at),
        }
    }
}

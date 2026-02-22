use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::types;

/// 商品状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "goods_status", rename_all = "PascalCase")]
pub enum GoodsStatus {
    /// 上架
    #[default]
    Available,
    /// 下架
    Unavailable,
}

/// SKU 元数据 (Postgres Composite Type)
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "goods_sku_meta")]
pub struct GoodsSkuMeta {
    /// 名称
    pub name: String,
    /// 项
    pub items: Vec<String>,
}

/// 商品参数 (Postgres Composite Type)
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[sqlx(type_name = "goods_argument")]
pub struct GoodsArgument {
    /// 名称
    pub name: String,
    /// 值
    pub value: String,
}

// --- 核心表模型 ---

/// 商品 (goods 表)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct Goods {
    pub id: String,
    /// 店铺ID             
    pub shop_id: String, // 店铺ID
    /// 是否虚拟商品
    pub is_vir: bool, // 是否虚拟商品
    /// 分类ID
    pub category_id: String, // 分类ID
    /// 商品名称
    pub name: String, // 商品名称
    /// 图片
    pub images: Vec<String>, // 图片数组
    /// 商品状态
    pub status: GoodsStatus, // 商品状态
    /// 详情
    pub detail: String, // 详情 (TEXT)
    /// 评论需审核
    pub comment_need_audit: bool, // 评论需审核
    /// 服务保障
    pub service_guarantee: Vec<String>, // 服务保障
    /// 标签
    pub tags: Vec<String>, // 标签
    /// 参数
    pub arguments: Vec<GoodsArgument>, // 参数 (复合类型数组)
    /// 库存总计
    pub stock: i64, // 库存总计 (BIGINT)
    /// 销量总计
    pub sales: i64, // 销量总计 (BIGINT)
    /// SKU 元数据
    pub sku_meta: Vec<GoodsSkuMeta>, // SKU配置元数据
    /// 运费
    pub fare: i32, // 运费 (INTEGER)
    /// 商品推荐
    pub recommendations: Vec<String>, // 商品推荐
    pub created_at: types::Timestamp,
    pub updated_at: types::Timestamp,
}

/// 商品规格属性 (goods_attrs 表)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct GoodsAttr {
    pub id: String,
    /// 商品ID
    pub goods_id: String,
    /// 选中的SKU组合
    pub sku_arr: Vec<String>, // 选中的SKU组合，如 ["红色", "XL"]
    /// 库存
    pub stock: i64, // 该规格库存
    /// 价格
    pub price: i64, // 价格 (建议存最小单位：分)
    /// 销量
    pub sales: i64, // 该规格销量
    /// 商品编号
    pub code: String, // 商品编号
    /// 条码
    pub bar_code: String, // 条码
    /// 体积(cm3)
    pub volume: i32, // 体积 (cm3)
    /// 重量(kg)
    pub weight: i32, // 重量 (kg)
    pub ver: i64,
}

/// 商品评论 (goods_comments 表)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct GoodsComment {
    pub id: String,
    /// 商品ID
    pub goods_id: String,
    /// 含规格的名称
    pub goods_full_name: String, // 含规格的名称
    /// 是否自评
    pub is_self: bool, // 是否自评
    /// 用户ID
    pub user_id: String, // 用户ID
    /// 用户头像
    pub user_avatar: String, // 用户头像
    /// 用户昵称
    pub user_nickname: String, // 用户昵称
    /// 内容
    pub content: String, // 评论内容
    /// 商品星级
    pub goods_star: i32, // 1-5星
    /// 服务星级
    pub service_star: i32, // 1-5星
    /// 图片
    pub images: Vec<String>, // 评论图片
    pub created_at: types::Timestamp,
}

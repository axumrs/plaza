CREATE TABLE IF NOT EXISTS "address" ( -- 收货地址
	"id" CHAR(20) PRIMARY KEY,
	"user_id" CHAR(20) NOT NULL, -- 用户ID
	"is_default" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否默认地址
	"consignee" VARCHAR NOT NULL, -- 收货人
	"phone" VARCHAR NOT NULL, -- 电话
	"address" VARCHAR NOT NULL, -- 地址
	"province" VARCHAR NOT NULL, -- 省份
	"city" VARCHAR NOT NULL DEFAULT '', -- 城市
	"county" VARCHAR NOT NULL DEFAULT '', -- 县
	"post_code" VARCHAR CHECK(length(post_code) = 6) NOT NULL DEFAULT '', -- 邮编
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP -- 添加时间
);
use std::ops::Deref;

use serde::Serialize;

/// 默认分页条数
const DEFAULT_PAGE_SIZE: i64 = 30;

/// 分页范围
#[derive(Serialize)]
pub struct PaginationRange {
    /// 页码
    pub page: i64,
    /// 每页条数
    pub page_size: i64,
}

impl PaginationRange {
    /// 计算偏移
    pub fn offset(&self) -> i64 {
        self.page * self.page_size
    }
}

impl Default for PaginationRange {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

/// 分页
#[derive(Serialize)]
pub struct Pagination<T> {
    /// 分页范围
    inner: PaginationRange,
    /// 总条数
    pub total: i64,
    /// 总页数
    pub page_total: i64,
    /// 数据
    pub data: T,
}

impl<T: Serialize> Pagination<T> {
    /// 根据分页大小创建
    pub fn with_page_size(data: T, total: i64, page: i64, page_size: i64) -> Self {
        let page_total = f64::ceil(total as f64 / page_size as f64) as i64;
        Self {
            inner: PaginationRange { page, page_size },
            total,
            page_total,
            data,
        }
    }
    /// 创建
    pub fn new(data: T, total: i64, page: i64) -> Self {
        Self::with_page_size(data, total, page, DEFAULT_PAGE_SIZE)
    }
}

impl<T> Deref for Pagination<T> {
    type Target = PaginationRange;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

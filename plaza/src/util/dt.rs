use chrono::{Local, NaiveDateTime, TimeZone};

use crate::{DateTime, Result};

/// 获取当前时间
pub fn now() -> DateTime {
    chrono::Local::now()
}

/// 转换为本地时间
fn naive_to_local(n: &NaiveDateTime) -> Result<DateTime> {
    match Local.from_local_datetime(n) {
        chrono::offset::LocalResult::Single(v) => Ok(v),
        chrono::offset::LocalResult::Ambiguous(v, _) => Ok(v),
        chrono::offset::LocalResult::None => Err(anyhow::anyhow!("无法解析日期时间").into()),
    }
}
/// 解析时间
pub fn parse(dt_str: &str) -> Result<DateTime> {
    let nd = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S")?;
    naive_to_local(&nd)
}

/// 获取今天的开始时间和结束时间
pub fn today() -> (DateTime, DateTime) {
    let now = Local::now();
    let start = now.format("%Y-%m-%d 00:00:00").to_string();
    let end = now.format("%Y-%m-%d 23:59:59").to_string();
    (parse(&start).unwrap_or(now), parse(&end).unwrap_or(now))
}

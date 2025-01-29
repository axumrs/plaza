use axum::http::{
    header::{AsHeaderName, AUTHORIZATION, USER_AGENT},
    HeaderMap,
};

/// 从请求头中获取值，如果不存在返回None
pub fn get_header_opt(headers: &HeaderMap, key: impl AsHeaderName) -> Option<&str> {
    headers.get(key).and_then(|v| v.to_str().ok())
}

/// 从请求头中获取用户代理，如果不存在返回None
pub fn get_user_agent_opt(headers: &HeaderMap) -> Option<&str> {
    get_header_opt(headers, USER_AGENT)
}

/// 从请求头中获取用户代理，如果不存在返回空字符串
pub fn get_user_agent(headers: &HeaderMap) -> &str {
    get_user_agent_opt(headers).unwrap_or_default()
}

/// 从请求头中获取IP，如果不存在返回空字符串
pub fn get_ip(headers: &HeaderMap) -> &str {
    let cf_connection_ip = get_header_opt(&headers, "CF-CONNECTING-IP").unwrap_or_default();
    let forwarded_for = get_header_opt(&headers, "X-FORWARDED-FOR").unwrap_or_default();
    let real_ip = get_header_opt(&headers, "X-REAL-IP").unwrap_or_default();

    if !cf_connection_ip.is_empty() {
        return cf_connection_ip;
    }

    if !forwarded_for.is_empty() {
        let forwarded_for_arr = forwarded_for.split(",").collect::<Vec<_>>();
        return forwarded_for_arr.get(0).copied().unwrap_or(real_ip);
    }

    real_ip
}

/// 从请求头中获取地理位置，如果不存在返回空字符串
pub fn get_cf_location(headers: &HeaderMap) -> &str {
    get_header_opt(headers, "CF-IPCOUNTRY").unwrap_or_default()
}

/// 从请求头中获取授权信息，如果不存在返回None
pub fn get_auth(headers: &HeaderMap) -> Option<&str> {
    get_header_opt(headers, AUTHORIZATION)
}

/// 从请求头中获取授权信息，如果不存在返回None
pub fn get_auth_token(headers: &HeaderMap) -> Option<&str> {
    let v = get_auth(headers);

    if let Some(v) = v {
        if let Some(v) = v.strip_prefix("Bearer ") {
            return Some(v);
        }
    }

    None
}

// 从请求头中获取URL
pub fn get_url<'a>(headers: &'a HeaderMap, req_uri: &'a str) -> &'a str {
    get_header_opt(headers, "X-URL").unwrap_or(req_uri)
}

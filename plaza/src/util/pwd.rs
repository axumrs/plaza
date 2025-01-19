use crate::Result;

/// 对密码进行哈希
pub fn hash(pwd: &str) -> Result<String> {
    let h = bcrypt::hash(pwd, bcrypt::DEFAULT_COST)?.to_string();
    Ok(h)
}

/// 验证密码
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool> {
    let r = bcrypt::verify(pwd, hashed_pwd)?;
    Ok(r)
}

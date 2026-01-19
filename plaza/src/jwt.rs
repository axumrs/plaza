use serde::{Deserialize, Serialize};

use crate::Result;

pub const ISSUER: &str = "AXUM.EU.ORG";

/// 用户信息
#[derive(Serialize, Deserialize)]
pub struct UserClaimsData {
    pub id: String,
    pub email: String,
    pub nickname: String,
}

/// jwt payload
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaimsData {
    User(UserClaimsData),
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
    pub data: ClaimsData,
}

impl Claims {
    pub fn new(sub: impl Into<String>, exp: i64, iat: i64, data: ClaimsData) -> Self {
        Self {
            sub: sub.into(),
            iss: ISSUER.into(),
            exp,
            iat,
            data,
        }
    }

    /// 生成带过去时间的 JWT
    pub fn with_exp(sub: impl Into<String>, exp_secs: u32, data: ClaimsData) -> Self {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + exp_secs as i64;
        Self::new(sub, exp, iat, data)
    }

    /// 生成 JWT 令牌
    pub fn encode(&self, secret: &[u8]) -> Result<Token> {
        Ok(Token(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret.into()),
        )?))
    }

    /// 解析 JWT
    pub fn decode(token: &str, secret: &[u8]) -> Result<Self> {
        Ok(jsonwebtoken::decode(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.into()),
            &jsonwebtoken::Validation::default(),
        )?
        .claims)
    }

    /// 判断是否过期
    pub fn is_expired(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp()
    }

    /// 验证元数据
    pub fn verify_meta(&self, sub: &str, iss: &str) -> bool {
        self.sub == sub && self.iss == iss
    }

    /// 验证
    pub fn verify(&self, sub: &str, iss: &str) -> bool {
        let vm = self.verify_meta(sub, iss);
        let ve = self.is_expired();
        vm && (!ve)
    }
}

/// JWT 令牌
#[derive(Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    pub fn to_string(self) -> String {
        self.0
    }
}

/// 生成 JWT 令牌
pub fn token(data: ClaimsData, secret: &str, sub: &str, exp_secs: u32) -> Result<Token> {
    let claims = Claims::with_exp(sub, exp_secs, data);
    claims.encode(secret.as_bytes())
}

/// 解析 JWT
pub fn get_claims(token: &str, secret: &str) -> Result<Claims> {
    Claims::decode(token, secret.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_gen_token() {
        let token = token(
            ClaimsData::User(UserClaimsData {
                id: "DUMMY.USER.ID".into(),
                email: "TEAM@MAIL.AXUM.EU.ORG".into(),
                nickname: "AXUM中文网".into(),
            }),
            "secret",
            "/PLAZA/USER",
            3600,
        )
        .unwrap();
        println!("{}", token.to_string());
    }
    #[test]
    fn test_jwt_get_claims() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIvUExBWkEvVVNFUiIsImlzcyI6IkFYVU0uRVUuT1JHIiwiZXhwIjoxNzY4NTQzOTgwLCJpYXQiOjE3Njg1NDAzODAsImRhdGEiOnsiaWQiOiJEVU1NWS5VU0VSLklEIiwiZW1haWwiOiJURUFNQE1BSUwuQVhVTS5FVS5PUkciLCJuaWNrbmFtZSI6IkFYVU3kuK3mlofnvZEifX0.J0JDw8dCe4x4JmdgjyMW-Nz0gNosYuJ9AYnNmEJRKx0";
        let claims = get_claims(token, "secret").unwrap();

        assert!(claims.verify("/PLAZA/USER", "AXUM.EU.ORG",));
        println!("{}", serde_json::to_string(&claims).unwrap());
    }
}

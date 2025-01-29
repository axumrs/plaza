use serde::{Deserialize, Serialize};

/// 用户信息
#[derive(Serialize, Deserialize)]
pub struct UserClaimsData {
    pub id: String,
    pub email: String,
    pub nickname: String,
}

/// JWT Claims
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaimsData {
    User(UserClaimsData),
}

/// 客户端信息
#[derive(Serialize, Deserialize)]
pub struct ClaimsClientData {
    pub ip: String,
    pub ua: String,
}

/// JWT
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
    pub data: ClaimsData,
    pub client: Option<ClaimsClientData>,
}

impl Claims {
    pub fn new(
        sub: String,
        exp: i64,
        iat: i64,
        data: ClaimsData,
        client: Option<ClaimsClientData>,
    ) -> Self {
        Self {
            sub,
            iss: "AXUM.RS".to_string(),
            exp,
            iat,
            data,
            client,
        }
    }

    /// 生成带过期时间的 JWT
    pub fn with_exp(
        sub: String,
        exp_secs: u32,
        data: ClaimsData,
        client: Option<ClaimsClientData>,
    ) -> Self {
        let now = chrono::Utc::now();
        let iat = now.timestamp();
        let exp = now.timestamp() + exp_secs as i64;
        Self::new(sub, exp, iat, data, client)
    }

    /// 生成 JWT Token
    pub fn encode(&self, secret: &[u8]) -> jsonwebtoken::errors::Result<Token> {
        Ok(Token(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret),
        )?))
    }

    /// 解析 JWT
    pub fn decode(token: &str, secret: &[u8]) -> jsonwebtoken::errors::Result<Self> {
        let data = jsonwebtoken::decode(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret),
            &jsonwebtoken::Validation::default(),
        )?;
        Ok(data.claims)
    }

    /// 判断是否过期
    pub fn is_expired(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp()
    }

    /// 验证客户端
    pub fn verify_client(&self, ip: &str, ua: &str) -> bool {
        if let Some(client) = &self.client {
            client.ip == ip && client.ua == ua
        } else {
            false
        }
    }

    /// 验证元数据
    pub fn verify_meta(&self, sub: &str, iss: &str) -> bool {
        self.sub == sub && self.iss == iss
    }

    /// 验证
    pub fn verify(&self, sub: &str, iss: &str, cli: Option<(&str, &str)>) -> bool {
        let vm = self.verify_meta(sub, iss);
        let ve = self.is_expired();
        let vc = match cli {
            Some((ip, ua)) => self.verify_client(ip, ua),
            None => true,
        };
        vm && (!ve) && vc
    }
}

/// JWT Token
#[derive(Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    /// 获取字符串
    pub fn to_string(self) -> String {
        self.0
    }
}

/// 生成 JWT Token
pub fn token(
    data: ClaimsData,
    secret: &str,
    sub: &str,
    exp_secs: u32,
    ip: &str,
    ua: &str,
) -> jsonwebtoken::errors::Result<Token> {
    let claims = Claims::with_exp(
        sub.into(),
        exp_secs,
        data,
        Some(ClaimsClientData {
            ip: ip.into(),
            ua: ua.into(),
        }),
    );
    claims.encode(secret.as_bytes())
}

/// 生成 JWT Token，不带客户端信息
pub fn token_without_client(
    data: ClaimsData,
    secret: &str,
    sub: &str,
    exp_secs: u32,
) -> jsonwebtoken::errors::Result<Token> {
    let claims = Claims::with_exp(sub.into(), exp_secs, data, None);
    claims.encode(secret.as_bytes())
}

/// 解析 JWT
pub fn get_claims(token: &str, secret: &str) -> jsonwebtoken::errors::Result<Claims> {
    Claims::decode(token, secret.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_gen_token() {
        let token = token(
            ClaimsData::User(UserClaimsData {
                id: "DUMMY.USER.ID".to_string(),
                email: "team@axum.rs".to_string(),
                nickname: "AXUM中文网".to_string(),
            }),
            "secret",
            "PLAZA_USER",
            3600,
            "127.0.0.1",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.142.86 Safari/537.36",
        )
        .unwrap();
        assert_eq!("", token.to_string());
    }
    #[test]
    fn test_jwt_get_claims() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJQTEFaQV9VU0VSIiwiaXNzIjoiQVhVTS5SUyIsImV4cCI6MTczNzUyMTcwNywiaWF0IjoxNzM3NTE4MTA3LCJkYXRhIjp7ImlkIjoiRFVNTVkuVVNFUi5JRCIsImVtYWlsIjoidGVhbUBheHVtLnJzIiwibmlja25hbWUiOiJBWFVN5Lit5paH572RIn0sImNsaWVudCI6eyJpcCI6IjEyNy4wLjAuMSIsInVhIjoiTW96aWxsYS81LjAgKFdpbmRvd3MgTlQgMTAuMDsgV2luNjQ7IHg2NCkgQXBwbGVXZWJLaXQvNTM3LjM2IChLSFRNTCwgbGlrZSBHZWNrbykgQ2hyb21lLzg5LjAuMTQyLjg2IFNhZmFyaS81MzcuMzYifX0.61BwPJ7oFUGuR8JJrOsskFwbhNiUTRMqm4jUAL-9mpI";
        let claims = get_claims(token, "secret").unwrap();
        // assert!(claims.verify_meta("PLAZA_USER", "AXUM.RS"));
        // assert!(!claims.is_expired());
        // assert!(claims.verify_client("127.0.0.1", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.142.86 Safari/537.36"));
        assert!(claims.verify(
            "PLAZA_USER",
            "AXUM.RS",
            Some(("127.0.0.1", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.142.86 Safari/537.36"))
        ));
    }
}

use std::time::Duration;

use serde::Deserialize;

use crate::Result;

const VERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

pub struct Turnstile<'a> {
    client: reqwest::Client,
    secret: &'a str,
}

#[derive(Deserialize)]
pub struct Response {
    pub success: bool,
}

impl<'a> Turnstile<'a> {
    pub fn try_new(secret: &'a str, timeout_secs: u8) -> Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(timeout_secs as _))
            .build()?;
        Ok(Self { client, secret })
    }

    pub async fn verify(&self, token: &str) -> Result<Response> {
        let form = [("secret", self.secret), ("response", token)];
        let res = self
            .client
            .post(VERIFY_URL)
            .form(&form)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    #[tokio::test]
    async fn should_turnstile_verify() {
        let cfg = config::Config::from_toml().unwrap();
        let res = Turnstile::try_new(&cfg.turnstile.secret_key, cfg.turnstile.timeout)
            .unwrap()
            .verify("XXXX.DUMMY.TOKEN.XXXX")
            .await
            .unwrap();
        assert!(res.success);
    }
}

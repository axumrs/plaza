use serde::Deserialize;

use crate::Result;

pub struct Turnstile<'a> {
    pub secret_key: &'a str,
    pub timeout: u8,
}

#[derive(Deserialize)]
pub struct Response {
    pub success: bool,
}

impl<'a> Turnstile<'a> {
    pub fn new(secret_key: &'a str, timeout: u8) -> Self {
        Self {
            secret_key,
            timeout,
        }
    }
}

pub async fn verify<'a>(turnstile: &Turnstile<'a>, token: &str) -> Result<Response> {
    let form = [("secret", turnstile.secret_key), ("response", token)];

    let url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

    let cli = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(turnstile.timeout.into()))
        .build()?;
    let resp = cli.post(url).form(&form).send().await?.json().await?;

    Ok(resp)
}

use crate::Result;
use anyhow::anyhow;
use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, response::Response},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct Data {
    pub subject: String,
    pub body: String,
    pub to: String,
}

impl Data {
    pub fn new(subject: impl Into<String>, body: impl Into<String>, to: impl Into<String>) -> Self {
        Data {
            subject: subject.into(),
            body: body.into(),
            to: to.into(),
        }
    }

    pub fn to_message(&self, sender: &str) -> Result<Message> {
        let sender = sender.parse().map_err(|e| anyhow!("{e}"))?;
        let to = self.to.parse().map_err(|e| anyhow!("{e}"))?;

        Message::builder()
            .from(sender)
            .to(to)
            .subject(self.subject.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(self.body.clone())
            .map_err(|e| e.into())
    }
}

pub async fn send(
    sender: impl Into<String>,
    password: impl Into<String>,
    smtp: impl Into<String>,
    m: Data,
) -> Result<Response> {
    let sender: String = sender.into();
    let smtp: String = smtp.into();
    let message = m.to_message(&sender)?;
    let creds = Credentials::new(sender, password.into());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp)?
        .credentials(creds)
        .build();
    let res = mailer.send(message).await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    #[tokio::test]
    async fn shuld_send_mail() {
        let cfg = config::Config::from_toml().unwrap();
        let mail_cfg = cfg.get_mail().unwrap();
        let m = Data::new("你好，世界", "欢迎使用商城", "team@mail.axum.eu.org");
        let res = send(&mail_cfg.user, &mail_cfg.password, &mail_cfg.smtp, m).await;
        assert!(res.is_ok());
    }
}

use crate::Result;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, response::Response},
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
        let sender = sender.parse()?;
        let to = self.to.parse()?;

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

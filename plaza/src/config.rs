use rand::Rng;
use serde::Deserialize;

use crate::Result;

/// 配置
#[derive(Deserialize)]
pub struct Config {
    /// Web 服务监听地址
    pub addr: String,
    /// Web 服务 URL 前缀
    pub url_prefix: String,
    /// 激活码配置
    pub activation_code: ActivationCode,
    /// 邮箱配置
    pub mails: Vec<Mail>,
    /// Turnstile 配置
    pub turnstile: Turnstile,
    /// 数据库配置
    pub database: Database,
}

/// 激活码配置
#[derive(Deserialize)]
pub struct ActivationCode {
    /// 重新发送间隔
    pub resend_duration: u64,
    /// 过期时长
    pub expire_duration: u64,
    /// 最大重试次数
    pub max_retry_count: u8,
}

/// 邮箱配置
#[derive(Deserialize)]
pub struct Mail {
    /// SMTP 服务器
    pub smtp: String,
    /// 邮箱用户名
    pub user: String,
    /// 邮箱密码
    pub password: String,
}

#[derive(Deserialize)]
pub struct Turnstile {
    /// Turnstile 秘钥
    pub secret_key: String,
    /// 验证超时，秒
    pub timeout: u8,
}

#[derive(Deserialize)]
pub struct Database {
    /// 数据库连接字符串
    pub dsn: String,
    /// 最大连接数
    pub max_conns: u32,
}

impl Config {
    /// 从配置文件初始化配置
    pub fn from_toml_opt(name: Option<&str>) -> Result<Self> {
        let name = name.unwrap_or("plaza");
        let cfg = ::config::Config::builder()
            .add_source(::config::File::with_name(name))
            .build()?
            .try_deserialize()?;
        Ok(cfg)
    }

    /// 从默认配置文件`plaza.toml`初始化配置
    pub fn from_toml() -> Result<Self> {
        Self::from_toml_opt(None)
    }

    /// 获取一个可用的邮箱配置
    pub fn get_mail(&self) -> Result<&Mail> {
        if self.mails.is_empty() {
            return Err(anyhow::anyhow!("没有可用的邮箱配置").into());
        }

        let len = self.mails.len();
        let mail = if len == 1 {
            &self.mails[0]
        } else {
            let idx = rand::thread_rng().gen_range(0..len);
            match self.mails.get(idx) {
                Some(v) => v,
                None => return Err(anyhow::anyhow!("获取邮箱配置失败").into()),
            }
        };

        Ok(mail)
    }
}

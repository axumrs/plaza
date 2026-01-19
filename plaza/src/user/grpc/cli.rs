use tonic::{
    metadata::MetadataValue, service::interceptor::InterceptedService, transport::Channel,
};

use crate::{
    config,
    interceptors::user_auth::UserClientInterceptor,
    pb::{self, user::user_service_client::UserServiceClient},
};

pub async fn connect<'a>(
    token: &'a str,
    cfg: &config::ServiceConfig,
) -> anyhow::Result<
    UserServiceClient<InterceptedService<tonic::transport::Channel, UserClientInterceptor>>,
> {
    let cli = Channel::builder(cfg.must_get_srv_addr().parse()?)
        .connect()
        .await?;
    let token: MetadataValue<_> = format!("Bearer {}", token).parse()?;

    let cfg_str = serde_json::to_string(&cfg)?;
    let cfg: MetadataValue<_> = cfg_str.parse()?;

    let c = pb::user::user_service_client::UserServiceClient::with_interceptor(
        cli,
        UserClientInterceptor { token, cfg },
    );

    Ok(c)
}

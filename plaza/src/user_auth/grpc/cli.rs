use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::{
    interceptors::skip_user_auth::SkipUserAuthInterceptor,
    pb::{
        user::user_service_client::UserServiceClient,
        user_auth::user_auth_service_client::UserAuthServiceClient,
    },
};

pub async fn connect_to_user_server(
    user_srv_endpoint: &str,
) -> anyhow::Result<UserServiceClient<InterceptedService<Channel, SkipUserAuthInterceptor>>> {
    let channel = Channel::builder(user_srv_endpoint.parse()?)
        .connect()
        .await?;
    let cli = UserServiceClient::new(InterceptedService::new(channel, SkipUserAuthInterceptor {}));
    Ok(cli)
}

pub async fn connect(
    addr: tonic::transport::Endpoint,
) -> anyhow::Result<UserAuthServiceClient<Channel>> {
    let cli = UserAuthServiceClient::connect(addr).await?;
    Ok(cli)
}

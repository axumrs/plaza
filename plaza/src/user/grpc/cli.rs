use crate::pb;

pub async fn connect(
    addr: tonic::transport::Endpoint,
) -> anyhow::Result<pb::user::user_service_client::UserServiceClient<tonic::transport::Channel>> {
    let c = pb::user::user_service_client::UserServiceClient::connect(addr).await?;
    Ok(c)
}

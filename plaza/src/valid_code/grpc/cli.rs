use crate::pb;

pub async fn connect(
    addr: tonic::transport::Endpoint,
) -> anyhow::Result<
    pb::valid_code::valid_code_service_client::ValidCodeServiceClient<tonic::transport::Channel>,
> {
    let c =
        pb::valid_code::valid_code_service_client::ValidCodeServiceClient::connect(addr).await?;
    Ok(c)
}

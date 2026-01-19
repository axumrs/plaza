use plaza::{config, pb, rds, utils, valid_code};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::log::init();

    let (_, runtime_config) = config::load_config().await?;
    let srv_config = &runtime_config.valid_code_service;

    let cli = redis::Client::open(srv_config.must_get_rds().url.as_str())?;
    let cli = rds::RdsCli::Single(cli);

    let key_prefix = &srv_config.must_get_rds().prefix;
    let expired_seconds = srv_config.must_get_rds().timeout;

    let addr = srv_config.must_get_srv_addr().as_str().parse()?;
    tracing::info!("Valid Code gRPC Server Listening on {}", addr);

    let srv = valid_code::grpc::srv::ValidCodeSrv::new(cli, key_prefix, expired_seconds.into());
    tonic::transport::Server::builder()
        .add_service(pb::valid_code::valid_code_service_server::ValidCodeServiceServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}

use plaza::{pb, rds, utils, valid_code};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::log::init();

    let cli = redis::Client::open("redis://127.0.0.1:6379/0")?;
    let cli = rds::RdsCli::Single(cli);

    let key_prefix = "plaza:valid_code";
    let expired_seconds = 60 * 5;

    let addr = "127.0.0.1:40001".parse()?;
    tracing::info!("Valid Code gRPC Server Listening on {}", addr);

    let srv = valid_code::grpc::srv::ValidCodeSrv::new(cli, key_prefix, expired_seconds);
    tonic::transport::Server::builder()
        .add_service(pb::valid_code::valid_code_service_server::ValidCodeServiceServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}

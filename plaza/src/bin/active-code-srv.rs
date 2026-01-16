use plaza::{active_code, pb, rds, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::log::init();

    let cli = redis::Client::open("redis://127.0.0.1:6379/0")?;
    let cli = rds::RdsCli::Single(cli);

    let key_prefix = "plaza:active_code";
    let expired_seconds = 60 * 5;

    let addr = "127.0.0.1:40001".parse()?;
    tracing::info!("Active Code gRPC Server Listening on {}", addr);

    let srv = active_code::grpc::srv::ActiveCodeSrv::new(cli, key_prefix, expired_seconds);
    tonic::transport::Server::builder()
        .add_service(pb::active_code::active_code_service_server::ActiveCodeServiceServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}

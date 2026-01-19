use plaza::{config, pb, user_auth::grpc::srv::UserAuthSrv, utils::log};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::init();

    let (_, rtc) = config::load_config().await?;
    let cfg = &rtc.user_auth_service;
    let user_srv_endpoint = rtc.user_service.must_get_srv_addr().as_str();

    tonic::transport::Server::builder()
        .add_service(
            pb::user_auth::user_auth_service_server::UserAuthServiceServer::new(UserAuthSrv::new(
                user_srv_endpoint,
            )),
        )
        .serve(cfg.must_get_srv_addr().parse()?)
        .await?;
    Ok(())
}

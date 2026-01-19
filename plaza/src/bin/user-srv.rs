use plaza::{config, pb, user, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::log::init();

    let (_, runtime_config) = config::load_config().await?;
    let srv_config = &runtime_config.user_service;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(srv_config.must_get_db().max_conns.into())
        .connect(srv_config.must_get_db().dsn.as_str())
        .await?;

    let addr = srv_config.must_get_srv_addr().as_str().parse()?;
    tracing::info!("User gRPC Server Listening on {}", addr);

    let srv = user::grpc::srv::UserSrv::new(pool);
    tonic::transport::Server::builder()
        .add_service(pb::user::user_service_server::UserServiceServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}

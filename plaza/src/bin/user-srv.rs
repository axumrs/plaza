use plaza::{pb, user, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::log::init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(
            std::env::var("DATABASE_URL")
                .unwrap_or("postgres://plaza:plaza@127.0.0.1:5432/plaza".into())
                .as_str(),
        )
        .await?;

    let addr = "127.0.0.1:40000".parse()?;
    tracing::info!("User gRPC Server Listening on {}", addr);

    let srv = user::grpc::srv::UserSrv::new(pool);
    tonic::transport::Server::builder()
        .add_service(pb::user::user_service_server::UserServiceServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}

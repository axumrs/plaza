use grpc_auth::pb;

fn interceptor(mut req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    req.metadata_mut().insert(
        "authorization",
        tonic::metadata::MetadataValue::from_static("Bearer AXUM.EU.ORG"),
    );

    Ok(req)
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = tonic::transport::Endpoint::from_static("http://127.0.0.1:9527")
        .connect()
        .await?;

    let mut cli = pb::greeter_client::GreeterClient::with_interceptor(cli, interceptor);
    let req = tonic::Request::new(pb::HelloRequest {
        name: "张三".into(),
    });

    let res = cli.say_hello(req).await?;
    println!("res: {res:?}");

    Ok(())
}

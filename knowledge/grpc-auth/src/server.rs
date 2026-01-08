use grpc_auth::pb;

#[derive(Default)]
pub struct GreeterSerice {}

#[tonic::async_trait]
impl pb::greeter_server::Greeter for GreeterSerice {
    async fn say_hello(
        &self,
        request: tonic::Request<pb::HelloRequest>,
    ) -> Result<tonic::Response<pb::HelloReply>, tonic::Status> {
        Ok(tonic::Response::new(pb::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        }))
    }
}

fn interceptor(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => v,
        None => return Err(tonic::Status::unauthenticated("未授权")),
    };

    if token.to_str().unwrap() != "Bearer AXUM.EU.ORG" {
        return Err(tonic::Status::unauthenticated("非法令牌"));
    }

    Ok(req)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let svc = GreeterSerice::default();
    let svc = pb::greeter_server::GreeterServer::with_interceptor(svc, interceptor);

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve("127.0.0.1:9527".parse().unwrap())
        .await
        .unwrap();

    Ok(())
}

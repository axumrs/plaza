use std::sync::Arc;

use crate::pb;

pub struct ApiState {
    pub cli: pb::user::user_service_client::UserServiceClient<tonic::transport::Channel>,
}

pub type ArcApiState = Arc<ApiState>;

pub fn arc(
    cli: pb::user::user_service_client::UserServiceClient<tonic::transport::Channel>,
) -> Arc<ApiState> {
    Arc::new(ApiState { cli })
}

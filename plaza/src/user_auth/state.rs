use std::sync::Arc;

use crate::{Result, config, pb, user_auth::grpc::cli::connect};

pub struct UserAuthState {
    pub fc: Arc<config::FileConfig>,
    pub rtc: Arc<config::RuntimeConfig>,
    pub vc_cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
        tonic::transport::Channel,
    >,
    pub cli:
        pb::user_auth::user_auth_service_client::UserAuthServiceClient<tonic::transport::Channel>,
}

pub type ArcUserAuthState = Arc<UserAuthState>;

impl UserAuthState {
    pub async fn arc(
        addr: tonic::transport::Endpoint,
        vc_cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
            tonic::transport::Channel,
        >,
    ) -> Result<ArcUserAuthState> {
        let (fc, rtc) = config::load_config().await?;
        let cli = connect(addr).await?;

        Ok(Arc::new(Self {
            vc_cli,
            fc,
            rtc,
            cli,
        }))
    }
}

use std::sync::Arc;

use crate::{Result, config, pb};

pub struct ApiState {
    pub fc: Arc<config::FileConfig>,
    pub rtc: Arc<config::RuntimeConfig>,
    pub vc_cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
        tonic::transport::Channel,
    >,
}

pub type ArcApiState = Arc<ApiState>;

impl ApiState {
    pub async fn arc(
        vc_cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
            tonic::transport::Channel,
        >,
    ) -> Result<ArcApiState> {
        let (fc, rtc) = config::load_config().await?;

        Ok(Arc::new(Self { vc_cli, fc, rtc }))
    }
}

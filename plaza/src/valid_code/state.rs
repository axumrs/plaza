use std::sync::Arc;

use crate::{config, pb};

pub struct ValidCodeState {
    pub cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
        tonic::transport::Channel,
    >,
    pub fc: Arc<config::FileConfig>,
    pub rtc: Arc<config::RuntimeConfig>,
}

impl ValidCodeState {
    pub async fn arc(
        cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
            tonic::transport::Channel,
        >,
    ) -> Arc<ValidCodeState> {
        let (fc, rtc) = config::load_config().await.unwrap();
        Arc::new(ValidCodeState { cli, fc, rtc })
    }
}

pub type ArcActiveCodeState = Arc<ValidCodeState>;

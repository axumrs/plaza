use std::sync::Arc;

use crate::pb;

pub struct ValidCodeState {
    pub cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
        tonic::transport::Channel,
    >,
}

impl ValidCodeState {
    pub fn arc(
        cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
            tonic::transport::Channel,
        >,
    ) -> Arc<ValidCodeState> {
        Arc::new(ValidCodeState { cli })
    }
}

pub type ArcActiveCodeState = Arc<ValidCodeState>;

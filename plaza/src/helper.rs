use crate::{Result, pb};

pub async fn invoke_valid_code_verify(
    code: impl Into<String>,
    email: impl Into<String>,
    kind: i32,
    mut vc_cli: pb::valid_code::valid_code_service_client::ValidCodeServiceClient<
        tonic::transport::Channel,
    >,
) -> Result<bool> {
    let r = vc_cli
        .verify(tonic::Request::new(pb::valid_code::ValidCode {
            code: code.into(),
            email: email.into(),
            kind: kind.into(),
        }))
        .await?
        .into_inner();
    return Ok(r.success);
}

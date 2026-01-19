use tonic::metadata::MetadataValue;

pub struct SkipUserAuthInterceptor;

impl tonic::service::Interceptor for SkipUserAuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        req.metadata_mut().insert(
            super::user_auth::SKIP_AUTH_META_KEY,
            MetadataValue::from_static("true"),
        );
        Ok(req)
    }
}

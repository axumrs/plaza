use sqlx::PgPool;
use tonic::async_trait;

use crate::pb::{
    self,
    merchant::{
        AuditMerchantRequest, CreateMerchantRequest, GetMerchantReply, GetMerchantRequest,
        ListMerchantsReply, ListMerchantsRequest, MerchantAddAccountsReply,
        MerchantAddAccountsRequest, MerchantGetAccountReply, MerchantGetAccountRequest,
        MerchantListAccountsReply, MerchantListAccountsRequest, MerchantRemoveAccountsRequest,
        MerchantUpdateAccountPasswordRequest, UpdateAccountRequest, UpdateMerchantRequest,
    },
    req, resp,
};

pub struct MerchantSrv {
    pool: PgPool,
}

impl MerchantSrv {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl pb::merchant::merchant_service_server::MerchantService for MerchantSrv {
    /// 创建
    async fn create(
        &self,
        request: tonic::Request<CreateMerchantRequest>,
    ) -> std::result::Result<tonic::Response<resp::IdReply>, tonic::Status> {
        unimplemented!()
    }
    /// 删除
    async fn delete(
        &self,
        request: tonic::Request<req::IdRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 审核
    async fn audit(
        &self,
        request: tonic::Request<AuditMerchantRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 修改商户
    async fn update_name(
        &self,
        request: tonic::Request<UpdateMerchantRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 获取单个商户
    async fn get(
        &self,
        request: tonic::Request<GetMerchantRequest>,
    ) -> std::result::Result<tonic::Response<GetMerchantReply>, tonic::Status> {
        unimplemented!()
    }
    /// 商户列表
    async fn list(
        &self,
        request: tonic::Request<ListMerchantsRequest>,
    ) -> std::result::Result<tonic::Response<ListMerchantsReply>, tonic::Status> {
        unimplemented!()
    }
    /// 添加账号
    async fn add_accounts(
        &self,
        request: tonic::Request<MerchantAddAccountsRequest>,
    ) -> std::result::Result<tonic::Response<MerchantAddAccountsReply>, tonic::Status> {
        unimplemented!()
    }
    /// 删除账号
    async fn remove_accounts(
        &self,
        request: tonic::Request<MerchantRemoveAccountsRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 修改账号密码
    async fn update_account_password(
        &self,
        request: tonic::Request<MerchantUpdateAccountPasswordRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 修改账号
    async fn update_account(
        &self,
        request: tonic::Request<UpdateAccountRequest>,
    ) -> std::result::Result<tonic::Response<resp::AffReply>, tonic::Status> {
        unimplemented!()
    }
    /// 获取单个账号
    async fn get_account(
        &self,
        request: tonic::Request<MerchantGetAccountRequest>,
    ) -> std::result::Result<tonic::Response<MerchantGetAccountReply>, tonic::Status> {
        unimplemented!()
    }
    /// 账号列表
    async fn list_accounts(
        &self,
        request: tonic::Request<MerchantListAccountsRequest>,
    ) -> std::result::Result<tonic::Response<MerchantListAccountsReply>, tonic::Status> {
        unimplemented!()
    }
}

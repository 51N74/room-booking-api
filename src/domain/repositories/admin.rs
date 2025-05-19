use axum::async_trait;
use mockall::automock;
use anyhow::Result;
use crate::domain::entities::admin::RegisterAdminEntity;

#[async_trait]
#[automock]
pub trait AdminRepository{
    async fn register_admin(&self,register_admin_entity:RegisterAdminEntity) -> Result<i32>;
    async fn find_by_username(&self,username:String) -> Result<RegisterAdminEntity>;
}
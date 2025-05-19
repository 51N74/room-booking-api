use axum::async_trait;
use mockall::automock;
use anyhow::Result;
use crate::domain::entities::users::RegisterUserEntity;

#[async_trait]
#[automock]
pub trait UserRepository{
    async fn register_user(&self,register_user_entity:RegisterUserEntity) -> Result<i32>;
    async fn find_by_username(&self,username:String) -> Result<RegisterUserEntity>;
}
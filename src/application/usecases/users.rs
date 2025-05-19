// use std::sync::Arc;

// use crate::domain::{repositories::users::UserRepository, value_objects::user_models::RegisterUserModel};

// pub struct UserUseCase<T>
// where 
// T:UserRepository + Send + Sync
// {
//     user_repository:Arc<T>,
// }

// impl <T>UserUseCase<T>
// where
// T:UserRepository + Send + Sync
// {
//     pub fn new(user_repository:Arc<T>) -> Self {
//         UserUseCase {
//             user_repository,
//         }
//     }

//     pub async fn register_user(&self, mut register_user_model:RegisterUserModel) -> Result<i32> {
//         self.user_repository.register_user(register_user_entity).await
//     }

// }


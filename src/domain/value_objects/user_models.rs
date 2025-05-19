use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

use crate::domain::entities::users::RegisterUserEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserModel {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,

}

impl RegisterUserModel{
    pub fn to_entity(&self)-> RegisterUserEntity{
        RegisterUserEntity{
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
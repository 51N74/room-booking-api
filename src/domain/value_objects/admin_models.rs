use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

use crate::domain::entities::admin::RegisterAdminEntity;

#[derive(Debug, Clone, Serialize, Deserialize, )]
pub struct AdminModel {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct RegisterAdminModel {
    pub username: String,
    pub password: String,
}

impl RegisterAdminModel{
    pub fn to_entity(&self)-> RegisterAdminEntity{
        RegisterAdminEntity{
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
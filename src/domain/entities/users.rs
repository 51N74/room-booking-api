use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{infrastructure::postgres::schema::users};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// impl UserEntity{
//     pub fn to_model(&self) -> UserModel {
//         UserModel {
//             id: self.id,
//             username: self.username.clone(),
//             email: self.email.clone(),            
//             password: self.password.clone(),   
//             first_name: self.first_name.clone(),
//             last_name: self.last_name.clone(),      
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//             deleted_at: self.deleted_at,
//         }
//     }
// }

#[derive(Debug, Clone,Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity{
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
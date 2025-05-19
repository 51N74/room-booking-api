use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{domain::value_objects::admin_models::AdminModel, infrastructure::postgres::schema::admin};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = admin)]pub struct AdminEntity{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// impl AdminEntity{
//     pub fn to_model(&self) -> AdminModel {
//         AdminModel {
//             id: self.id,
//             username: self.username.clone(),
//             password: self.password.clone(),
//             created_at: self.created_at,
//             updated_at: self.updated_at,
//         }
//     }
// }

#[derive(Debug, Clone,Insertable,Queryable)]
#[diesel(table_name = admin)]
pub struct RegisterAdminEntity{
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
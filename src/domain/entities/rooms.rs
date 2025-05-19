use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, AsChangeset};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::room_models::RoomModel;
use crate::infrastructure::postgres::schema::rooms;



#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = rooms)]
pub struct RoomEntity{
    pub id: i32,
    pub room_number: i32,
    pub room_type: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub is_available: bool,
    pub price_per_night: i32,
    pub amenities: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,

}


impl RoomEntity{
    pub fn to_model(&self) -> RoomModel {
        RoomModel {
            id: self.id,
            room_number: self.room_number,
            room_type: self.room_type.clone(),
            description: self.description.clone(),
            capacity: self.capacity,
            is_available: self.is_available,
            price_per_night: self.price_per_night,
            amenities: self.amenities.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[derive(Debug, Clone,Insertable,Queryable)]
#[diesel(table_name = rooms)]
pub struct AddRoomEntity{
    pub room_number: i32,
    pub room_type: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub is_available: bool,
    pub price_per_night: i32,
    pub amenities: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    
}

#[derive(Debug, Clone,Insertable,Queryable,AsChangeset)]
#[diesel(table_name = rooms)]
pub struct UpdateRoomEntity{
    pub room_number: Option<i32>,
    pub room_type: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<i32>,
    pub is_available: Option<bool>,
    pub price_per_night: Option<i32>,
    pub amenities: Option<String>,
    pub updated_at: NaiveDateTime,
}
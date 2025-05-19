use chrono::NaiveDateTime;
use diesel::prelude::AsChangeset;
use serde::{Deserialize, Serialize};
use crate::domain::entities::rooms::{AddRoomEntity, UpdateRoomEntity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomModel {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRoomModel{
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

impl AddRoomModel {
    pub fn to_entity(&self) -> AddRoomEntity {
        AddRoomEntity {
            room_number: self.room_number,
            room_type: self.room_type.clone(),
            description: self.description.clone(),
            capacity: self.capacity,
            is_available: self.is_available,
            price_per_night: self.price_per_night,
            amenities: self.amenities.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at:chrono::Utc::now().naive_utc(),
        }
    }
        
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomModel{
    pub room_number: Option<i32>,
    pub room_type: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<i32>,
    pub is_available: Option<bool>,
    pub price_per_night: Option<i32>,
    pub amenities: Option<String>,
}

impl UpdateRoomModel {
    pub fn to_entity(&self) -> UpdateRoomEntity {
        UpdateRoomEntity {
            room_number: self.room_number,
            room_type: self.room_type.clone(),
            description: self.description.clone(),
            capacity: self.capacity,
            is_available: self.is_available,
            price_per_night: self.price_per_night,
            amenities: self.amenities.clone(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}


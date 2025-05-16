// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Int4,
        user_id -> Int4,
        room_id -> Int4,
        check_in_date -> Date,
        check_out_date -> Date,
        number_of_guests -> Int4,
        total_price -> Numeric,
        #[max_length = 50]
        booking_status -> Varchar,
        booking_date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        #[max_length = 50]
        room_number -> Varchar,
        #[max_length = 100]
        room_type -> Varchar,
        capacity -> Int4,
        price_per_night -> Numeric,
        amenities -> Nullable<Text>,
        description -> Nullable<Text>,
        is_available -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    rooms,
    users,
);

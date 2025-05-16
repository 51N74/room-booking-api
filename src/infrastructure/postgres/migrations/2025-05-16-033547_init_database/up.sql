
-- ตาราง: rooms
CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    room_number VARCHAR(50) UNIQUE NOT NULL,
    room_type VARCHAR(100) NOT NULL,
    capacity INTEGER NOT NULL,
    price_per_night DECIMAL(10, 2) NOT NULL,
    amenities TEXT,
    description TEXT,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

-- ตาราง: users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    role VARCHAR(50) NOT NULL DEFAULT 'guest',
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

-- ตาราง: bookings
CREATE TABLE bookings (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    room_id INTEGER NOT NULL REFERENCES rooms(id),
    check_in_date DATE NOT NULL,
    check_out_date DATE NOT NULL,
    number_of_guests INTEGER NOT NULL,
    total_price DECIMAL(10, 2) NOT NULL,
    booking_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    booking_date TIMESTAMP NOT NULL DEFAULT now(),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

-- ความสัมพันธ์ระหว่างตาราง
ALTER TABLE bookings ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE bookings ADD CONSTRAINT fk_room FOREIGN KEY (room_id) REFERENCES rooms(id);

-- ตารางเสริม (Optional): payment_methods
-- CREATE TABLE payment_methods (
--     id SERIAL PRIMARY KEY,
--     method_name VARCHAR(100) UNIQUE NOT NULL,
--     is_active BOOLEAN NOT NULL DEFAULT TRUE,
--     created_at TIMESTAMP NOT NULL DEFAULT now(),
--     updated_at TIMESTAMP NOT NULL DEFAULT now()
-- );

-- ตารางเสริม (Optional): payments
-- CREATE TABLE payments (
--     id SERIAL PRIMARY KEY,
--     booking_id INTEGER NOT NULL REFERENCES bookings(id),
--     payment_method_id INTEGER NOT NULL REFERENCES payment_methods(id),
--     payment_date TIMESTAMP NOT NULL DEFAULT now(),
--     amount DECIMAL(10, 2) NOT NULL,
--     transaction_id VARCHAR(255),
--     payment_status VARCHAR(50) NOT NULL DEFAULT 'pending',
--     created_at TIMESTAMP NOT NULL DEFAULT now(),
--     updated_at TIMESTAMP NOT NULL DEFAULT now()
-- );

-- ความสัมพันธ์ระหว่างตารางเสริม
-- ALTER TABLE payments ADD CONSTRAINT fk_booking FOREIGN KEY (booking_id) REFERENCES bookings(id);
-- ALTER TABLE payments ADD CONSTRAINT fk_payment_method FOREIGN KEY (payment_method_id) REFERENCES payment_methods(id);

-- ตารางเสริม (Optional): room_images
-- CREATE TABLE room_images (
--     id SERIAL PRIMARY KEY,
--     room_id INTEGER NOT NULL REFERENCES rooms(id),
--     image_url VARCHAR(255) NOT NULL,
--     is_thumbnail BOOLEAN NOT NULL DEFAULT FALSE,
--     created_at TIMESTAMP NOT NULL DEFAULT now(),
--     updated_at TIMESTAMP NOT NULL DEFAULT now()
-- );

-- ความสัมพันธ์ระหว่างตารางเสริม
-- ALTER TABLE room_images ADD CONSTRAINT fk_room_image FOREIGN KEY (room_id) REFERENCES rooms(id);
-- ลบตารางที่อาจมีอยู่ (ตามลำดับที่ควรลบเพื่อหลีกเลี่ยง Foreign Key errors)
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS payment_methods;
DROP TABLE IF EXISTS bookings;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS room_images; -- ลบตาราง room_images ก่อน rooms ถ้ามี FK อ้างอิง
DROP TABLE IF EXISTS rooms;
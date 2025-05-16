use anyhow::Result;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

pub type PgPoolSquard = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> Result<PgPoolSquard> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool.");

        Ok(pool)
}
use std::sync::Arc;

use room_booking_api::{config::config_loader, infrastructure::{axum_http::http_serve::start_server, postgres::postgres_connection}};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load(){
        Ok(env)=>env,
        Err(e)=>{
            tracing::error!("Failed to load environment variables: {}", e);
            return;
        }
    };

    info!("Env has been loaded successfully");

    let postgres_pool = match postgres_connection::create_pool(&dotenvy_env.database.url){
        Ok(pool)=>pool,
        Err(e)=>{
            tracing::error!("Failed to create postgres pool: {}", e);
            std::process::exit(1);
        }
    };

    info!("Postgres connection has been established");

      start_server(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start the server");

}

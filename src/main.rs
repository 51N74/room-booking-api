use room_booking_api::config::config_loader;
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

    info!("Env has been loaded {:?}", dotenvy_env);


}

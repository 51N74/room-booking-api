use anyhow::Result;
use super::{config_model::{AdminSecret, Database, DotEnvyConfig, Server, UserSecret}, stage::Stage};


pub fn load()->Result<DotEnvyConfig>{
    dotenvy::dotenv().ok();

    let server = Server{
        port:std::env::var("SERVER_PORT").expect("SERVER_PORT is invalid").parse()?,
        body_limit:std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is invalid").parse()?,
        timeout:std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is invalid").parse()?,
    };

    let database = Database{
        url:std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig{
        server,
        database,
    })
}

pub fn get_stage() -> Stage{
    dotenvy::dotenv().ok();

    let stage = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::from_str(&stage).unwrap_or_default()
    
}

fn get_user_secret_env() -> Result<UserSecret>{
    dotenvy::dotenv().ok();
   
    Ok(UserSecret{
        secret: std::env::var("USER_SECRET").expect("USER_SECRET is invalid"),
        refresh_secret: std::env::var("USER_REFRESH_SECRET").expect("USER_REFRESH_SECRET is invalid"),
    })
}

fn get_admin_secret_env() -> Result<AdminSecret>{
    dotenvy::dotenv().ok();

    Ok(AdminSecret{
        secret: std::env::var("ADMIN_SECRET").expect("ADMIN_SECRET is invalid"),
        refresh_secret: std::env::var("ADMIN_REFRESH_SECRET").expect("ADMIN_REFRESH_SECRET is invalid"),
    })
}
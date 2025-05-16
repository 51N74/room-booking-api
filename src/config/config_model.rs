#[derive(Debug,Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug,Clone)]
pub struct Server{
    pub port:u16,
    pub body_limit:u32,
    pub timeout:u32,
}

#[derive(Debug,Clone)]
pub struct Database{
    pub url:String
}

#[derive(Debug,Clone)]
pub struct UserSecret{
    pub secret:String,
    pub refresh_secret:String,
}

#[derive(Debug,Clone)]
pub struct AdminSecret{
     pub secret:String,
    pub refresh_secret:String,
}

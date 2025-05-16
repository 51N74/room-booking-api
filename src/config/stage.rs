use std::fmt;

use serde::de;

#[derive(Debug,Clone,PartialEq,Default)]
pub enum Stage{
    Local,
    #[default]
    Development,
    Production
}

    impl fmt::Display for Stage{
        fn fmt(&self,f:&mut fmt::Formatter<'_>) -> fmt::Result{
            match self{
                Stage::Local => write!(f,"Local"),
                Stage::Development => write!(f,"Development"),
                Stage::Production => write!(f,"Production"),
            }
        }
    
}

impl Stage{
    pub fn from_str(s:&str) -> Result<Stage,Box<dyn std::error::Error>>{
        match s.to_lowercase().as_str(){
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(format!("Invalid stage: {}",s).into()),
        }
    }
}
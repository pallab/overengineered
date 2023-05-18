
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host : String,
    pub port : u16
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MySqlConfig {
    pub db_url : String
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RpcConfig {
    pub host : String,
    pub port : u16,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub server : ServerConfig,
    pub mysql : MySqlConfig,
    pub rpc : RpcConfig,
}

pub fn load_config(config_file : String) -> AppConfig {
    Config::builder()
        .add_source( File::new(&config_file, FileFormat::Json))
        .build().unwrap().try_deserialize().unwrap()
}
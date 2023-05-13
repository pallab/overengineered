
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub host : String,
    pub port : u16
}

pub fn load_config(config_file : String) -> AppConfig {
    Config::builder()
        .add_source( File::new(&config_file, FileFormat::Json))
        .build().unwrap().try_deserialize().unwrap()
}
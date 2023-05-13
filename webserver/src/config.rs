
use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub name : String,
}

pub fn load_config() -> AppConfig {
    Config::builder()
        .add_source( File::new("./config.json", FileFormat::Json))
        .build().unwrap().try_deserialize().unwrap()
}
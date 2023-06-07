use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServerConfig;

impl ServerConfig {
    pub fn host() -> String {
        std::env::var("BIND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
    }
    pub fn port() -> u16 {
        std::env::var("BIND_PORT")
            .map(|p| p.parse::<u16>().unwrap_or(8080))
            .unwrap_or(8080)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MySqlConfig;

impl MySqlConfig {
    pub fn db_url() -> String {
        std::env::var("MYSQL_URL")
            .unwrap_or_else(|_| "mysql://tenxdev:tenxpasswd@127.0.0.1:3306/overengineered".to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RpcConfig;

impl RpcConfig {
    pub fn address() -> String {
        std::env::var("RPC_ADDRESS")
            .unwrap_or_else(|_| "http://127.0.0.1:8089".to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KafkaConfig;

impl KafkaConfig {
    pub fn servers() -> String {
        std::env::var("KAFKA_SERVERS")
            .unwrap_or_else(|_| "localhost:9094".to_string())
    }
    pub fn source_topic() -> String {
        std::env::var("KAFKA_SOURCE")
            .unwrap_or_else(|_| "words".to_string())
    }
    pub fn sink_topic() -> String {
        std::env::var("KAFKA_SINK")
            .unwrap_or_else(|_| "letter_counts".to_string())
    }
    pub fn partitions() -> i32 {
        std::env::var("KAFKA_PARTITIONS")
            .map(|p| p.parse::<i32>().unwrap_or(8080))
            .unwrap_or(8080)
    }
}


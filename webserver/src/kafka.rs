use std::time::Duration;
use rdkafka::admin::{AdminClient, AdminOptions, TopicReplication, TopicResult};
use rdkafka::ClientConfig;
use rdkafka::config::FromClientConfig;
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use crate::config::KafkaConfig;
use crate::stocks_rpc::StockPriceResponse;

pub struct Kafka {
    producer: FutureProducer,
}

impl Kafka {
    pub fn new(config : KafkaConfig) -> Self {
        let producer : Option<FutureProducer> = ClientConfig::new()
            .set("bootstrap.servers", config.server)
            .set("message.timeout.ms", "5000")
            .create()
            .ok();

        Self { producer: producer.unwrap() }
    }

    pub async fn create_topic(&self, topic : &str, partitions : i32) -> KafkaResult<Vec<TopicResult>>{
        let new_topic = rdkafka::admin::NewTopic::new(
            topic, partitions, TopicReplication::Fixed(3)
        );

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", "localhost:9094");
        client_config.set("message.timeout.ms", "5000");

        let admin_client = AdminClient::from_config(&client_config)
            .ok().unwrap();
        let admin_options = AdminOptions::new();

        admin_client.create_topics([new_topic].iter(), &admin_options).await
    }

    pub async fn send(&self, p: StockPriceResponse) -> OwnedDeliveryResult {
        self.producer
            .send(
                FutureRecord::to("g")
                    .payload(&format!("{:?}", p))
                    .key(&format!("{}-{}", p.ticker, p.timestamp)),
                Duration::from_secs(0),
            ).await
    }

    pub async fn send_word(&self, topic : &str, key: &str, value: &str, timestamp: i64 ) -> OwnedDeliveryResult {
        self.producer
            .send(
                FutureRecord::to(topic)
                    .payload(value)
                    .timestamp(timestamp)
                    .key(key),
                Duration::from_secs(0),
            ).await
    }
}
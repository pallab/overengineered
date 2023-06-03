use std::time::Duration;
use rdkafka::admin::{AdminClient, AdminOptions, TopicReplication, TopicResult};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use rdkafka::config::FromClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use crate::config::KafkaConfig;

pub struct KafkaAdmin {
    pub(crate) config: KafkaConfig,
    pub(crate) client: Option<AdminClient<DefaultClientContext>>,
}

pub struct KafkaProducer {
    pub(crate) config: KafkaConfig,
    pub(crate) producer: FutureProducer,
}

pub struct KafkaConsumer {
    pub(crate) config: KafkaConfig,
    pub(crate) consumer: BaseConsumer,
}

impl KafkaAdmin {
    pub fn new( config : KafkaConfig) -> Self {

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &config.server);
        client_config.set("message.timeout.ms", "5000");

        let client = AdminClient::from_config(&client_config)
            .ok();

       Self {config, client}
    }

    pub async fn create_topic(&self, topic : &str) -> KafkaResult<Vec<TopicResult>> {
        let new_topic = rdkafka::admin::NewTopic::new(
            topic, self.config.partitions, TopicReplication::Fixed(3),
        );

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &self.config.server);
        client_config.set("message.timeout.ms", "5000");

        let admin_client = AdminClient::from_config(&client_config)
            .ok().unwrap();
        let admin_options = AdminOptions::new();

        admin_client.create_topics([new_topic].iter(), &admin_options).await
    }
}

impl KafkaProducer {
    pub fn new( config: KafkaConfig) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.server)
            .set("message.timeout.ms", "5000")
            .create()
            .ok()
            .unwrap();
        Self { config, producer }
    }

    pub async fn send_word(&self, topic: &str, key: &str, value: &str, timestamp: i64) -> OwnedDeliveryResult {
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

impl KafkaConsumer {
    pub fn new(config: KafkaConfig, topic: String, group_id: String) -> Self {
        let consumer: Option<BaseConsumer> = ClientConfig::new()
            .set("bootstrap.servers", &config.server)
            .set("group.id", group_id)
            .create()
            .ok();

        consumer.as_ref().map(|c| {
            c.subscribe(&[&topic])
        });

        Self {config, consumer: consumer.unwrap() }
    }
}

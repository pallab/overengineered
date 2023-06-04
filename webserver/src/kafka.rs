use std::time::Duration;
use log::info;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication, TopicResult};
use rdkafka::ClientConfig;
use rdkafka::config::FromClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::util::Timeout;
use crate::config::KafkaConfig;

pub struct KafkaAdmin {
    pub(crate) config: KafkaConfig,
}

pub struct KafkaProducer {
    pub(crate) producer: FutureProducer,
}

pub struct KafkaConsumer {
    pub(crate) consumer: BaseConsumer,
}

impl KafkaAdmin {
    pub fn new(config: KafkaConfig) -> Self {
        Self { config }
    }

    pub async fn create_topic(&self, topics: Vec<String>) -> KafkaResult<Vec<TopicResult>> {
        let new_topics: Vec<NewTopic> = topics.iter().map(|topic| {
            NewTopic::new(
                topic, self.config.partitions, TopicReplication::Fixed(1),
            )
        }).collect();

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &self.config.server);
        client_config.set("message.timeout.ms", "5000");

        let admin_client = AdminClient::from_config(&client_config)
            .ok().unwrap();
        let admin_options = AdminOptions::new();

        admin_client.create_topics(new_topics.iter(), &admin_options).await
    }
}

impl KafkaProducer {
    pub fn new(config: KafkaConfig) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.server)
            .set("message.timeout.ms", "5000")
            .create()
            .ok()
            .unwrap();
        Self {  producer }
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

        if let Some(c) = consumer.as_ref() {
            let t = Timeout::After(Duration::from_secs(10));
            c.subscribe(&[&topic]).expect("Error : could no subscribe");

            // required because the partition is assigned to this consumer only after a poll
            // otherwise the seek will fail
            if let Some(r) = c.poll(t) {
                info!("poll result - {:#?}", r)
            }

            // c.seek(&topic, 0, Offset::Beginning,
            //        t)
            //     .expect("Error : could not seek");
        }

        Self { consumer: consumer.unwrap() }
    }
}

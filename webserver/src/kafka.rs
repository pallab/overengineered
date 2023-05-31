use std::time::Duration;
use rdkafka::admin::{AdminClient, AdminOptions, TopicReplication, TopicResult};
use rdkafka::ClientConfig;
use rdkafka::config::FromClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaResult;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use crate::config::KafkaConfig;
use crate::stocks_rpc::StockPriceResponse;

pub struct KafkaClient {
    pub(crate) config: KafkaConfig,
    pub(crate) producer: Option<FutureProducer>,
}

impl KafkaClient {
    pub fn producer(&mut self) -> FutureProducer {
        if self.producer.is_some() {
            self.producer.clone().unwrap()
        } else {
            let producer: Option<FutureProducer> = ClientConfig::new()
                .set("bootstrap.servers", &self.config.server)
                .set("message.timeout.ms", "5000")
                .create()
                .ok();
            self.producer = producer.clone();
            producer.unwrap()
        }
    }

    pub fn consumer(&mut self, group_id: String) -> BaseConsumer {
        let consumer: Option<BaseConsumer> = ClientConfig::new()
            .set("bootstrap.servers", &self.config.server)
            .set("group.id", group_id)
            .create()
            .ok();

        consumer.as_ref().map(|c| {
            c.subscribe(&[&self.config.topic])
        });

        consumer.unwrap()
    }

    pub async fn create_topic(&self) -> KafkaResult<Vec<TopicResult>> {
        let new_topic = rdkafka::admin::NewTopic::new(
            self.config.topic.as_str(), self.config.partitions, TopicReplication::Fixed(3),
        );

        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &self.config.server);
        client_config.set("message.timeout.ms", "5000");

        let admin_client = AdminClient::from_config(&client_config)
            .ok().unwrap();
        let admin_options = AdminOptions::new();

        admin_client.create_topics([new_topic].iter(), &admin_options).await
    }

    pub async fn send(&mut self, p: StockPriceResponse) -> OwnedDeliveryResult {
        self.producer.clone().unwrap_or_else(|| self.producer())
            .send(
                FutureRecord::to("g")
                    .payload(&format!("{:?}", p))
                    .key(&format!("{}-{}", p.ticker, p.timestamp)),
                Duration::from_secs(0),
            ).await
    }

    pub async fn send_word(&mut self, topic: &str, key: &str, value: &str, timestamp: i64) -> OwnedDeliveryResult {
        self.producer.clone().unwrap_or_else(|| self.producer())
            .send(
                FutureRecord::to(topic)
                    .payload(value)
                    .timestamp(timestamp)
                    .key(key),
                Duration::from_secs(0),
            ).await
    }
}
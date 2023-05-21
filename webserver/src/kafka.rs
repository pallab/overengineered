use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use crate::stocks_rpc::StockPriceResponse;

pub struct Kafka {
    producer: FutureProducer,
}

impl Kafka {
    pub fn new() -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9094")
            .set("message.timeout.ms", "5000")
            .create()
            .ok();

        Self { producer: producer.unwrap() }
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
}
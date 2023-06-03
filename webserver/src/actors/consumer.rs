use std::time::{Instant, Duration};
use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use log::{error, info};
use actix::prelude::*;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::message::{Headers, Message};
use serde::Deserialize;
use serde_json::*;
use crate::config::KafkaConfig;
use crate::kafka::{KafkaConsumer};
use crate::route_websocket::{CharMetrics, WebSocket};

#[derive(Deserialize, Debug)]
pub struct CharCount {
    c: String,
    count: i32,
}

pub struct ConsumerActor {
    pub parent: Addr<WebSocket>,
    pub kafka_config: Option<KafkaConfig>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct StartConsumer;

impl Actor for ConsumerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Consumer actor started");
        ctx.address().do_send(StartConsumer)
    }
}


impl Handler<StartConsumer> for ConsumerActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: StartConsumer, ctx: &mut Self::Context) -> Self::Result {
        let kafka_config = self.kafka_config.take().unwrap();

        Box::pin(
            async move {
                let consumer = KafkaConsumer::new(kafka_config.clone(), kafka_config.sink_topic, "g1".to_string());

                for record in consumer.consumer.iter() {
                    let m = record.unwrap();
                    let v: &str = m.payload_view::<str>().unwrap().unwrap();
                    match serde_json::from_str::<Vec<CharCount>>(v) {
                        Ok(r) => {
                            self.parent.do_send(CharMetrics{counts : r});
                            // info!("CONSUMER msg {:?} -> {:?}", m.key() , r)
                        },
                        Err(e) => error!("could not parse {} \n {}", e, v)
                    }
                }
            }
                .into_actor(self)
        )
    }
}


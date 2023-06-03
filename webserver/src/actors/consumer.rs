
use std::time::{ Instant, Duration};
use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use log::info;
use actix::prelude::*;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::message::{Headers, Message};
use serde_json::*;
use crate::config::KafkaConfig;
use crate::kafka::{ KafkaConsumer};
use crate::route_websocket::WebSocket;

pub struct ConsumerActor {
    pub parent: Addr<WebSocket>,
    pub kafka_config: Option<KafkaConfig>
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct PrintStats;

impl Actor for ConsumerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Consumer actor started");
        ctx.address().do_send(PrintStats)
        // ctx.run_interval(Duration::from_secs(10), |act, ctx|{
        //     ctx.address().do_send(PrintStats)
        // });
    }
}


impl Handler<PrintStats> for ConsumerActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: PrintStats, ctx: &mut Self::Context) -> Self::Result {
       info!("Hertbeat from consumer");
        let kafka_config = self.kafka_config.take().unwrap();

        Box::pin(
            async move {
                info!("in async");
                let consumer = KafkaConsumer::new(kafka_config.clone(), kafka_config.sink_topic, "g1".to_string());
                info!("created kafka");

                info!("2");

                for record in consumer.consumer.iter() {
                    let m  = record.unwrap();
                    let v: &str = m.payload_view::<str>().unwrap().unwrap();

                    info!("CONSUMER msg {:?} -> {}", m.key() , v)
                }
                info!("4")
            }
                .into_actor(self)
        )
    }
}


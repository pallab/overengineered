use std::sync::Arc;
use std::time::Duration;
use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use log::{error, info};
use actix::prelude::*;
use rdkafka::message::{Message};
use rdkafka::util::Timeout;
use serde::{Serialize, Deserialize};
use crate::config::KafkaConfig;
use crate::kafka::{KafkaConsumer};
use crate::route_websocket::{CharMetrics, WebSocket};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharCount {
    c: String,
    count: i32,
}

pub struct ConsumerActor {
    pub websockets: Vec<Addr<WebSocket>>,
    pub kafka_client: Arc<KafkaConsumer>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct RegisterWebsocket {
    pub addr: Addr<WebSocket>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct PollKafka;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct PollResult {
    pub found: bool,
}

impl ConsumerActor {
    pub fn start(kafka_config: KafkaConfig) -> Addr<ConsumerActor> {
        let consumer = KafkaConsumer::new(kafka_config.clone(),
                                          kafka_config.sink_topic,
                                          "g1".to_string());
        ConsumerActor {
            websockets: vec![],
            kafka_client: Arc::new(consumer),
        }.start()
    }
}

impl Actor for ConsumerActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Consumer actor started");
    }
}

impl Handler<RegisterWebsocket> for ConsumerActor {
    type Result = ();

    fn handle(&mut self, msg: RegisterWebsocket, ctx: &mut Self::Context) -> Self::Result {
        self.websockets.push(msg.addr);
        info!("Registering new websocket");
        if self.websockets.len() == 1 {
            ctx.notify(PollKafka)
        }
    }
}

impl Handler<PollResult> for ConsumerActor {
    type Result = ();

    fn handle(&mut self, msg: PollResult, ctx: &mut Self::Context) -> Self::Result {
        let duration = if msg.found {1} else {5};
        ctx.notify_later(PollKafka,  Duration::from_secs(duration));
        info!("Poll Result {}", msg.found);
    }
}

impl Handler<PollKafka> for ConsumerActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: PollKafka, ctx: &mut Self::Context) -> Self::Result {
        let websockets = self.websockets.clone();
        let self_addr = ctx.address();
        let kafka_client = self.kafka_client.clone();

        Box::pin(
            async move {
                let timeout = Timeout::from(Duration::ZERO);

                if let Some(Ok(msg)) = kafka_client.consumer.poll(timeout) {
                    let v: &str = msg.payload_view::<str>().unwrap().unwrap();
                    match serde_json::from_str::<Vec<CharCount>>(v) {
                        Ok(r) => {
                            websockets.iter().for_each(|ws| {
                                ws.do_send(CharMetrics { counts: r.clone() })
                            });
                        }
                        Err(e) => error!("could not parse {} \n {}", e, v)
                    };
                    self_addr.do_send(PollResult { found: true })
                } else {
                    self_addr.do_send(PollResult { found: false })
                }
            }
                .into_actor(self)
        )
    }
}


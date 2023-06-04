use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use actix::{Actor, StreamHandler, Handler};
use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::{error, info};
use serde::Serialize;
use crate::actors::consumer::{CharCount, ConsumerActor};
use crate::actors::producer::ProducerActor;
use crate::actors::stats::StatsActor;
use crate::canvas::Tile;
use crate::config::{KafkaConfig, RpcConfig};

#[derive(Message)]
#[rtype(result = "()")]
pub struct PrintLine {
    line: String,
    count: i32,
}


#[derive(Message)]
#[rtype(result = "()")]
pub enum Setup {
    Producer,
    Consumer,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct CharMetrics {
    pub counts: Vec<CharCount>,
}

pub struct WebSocket {
    rpc_config: RpcConfig,
    kafka_config: KafkaConfig,
    producer: Option<Addr<ProducerActor>>,
    consumer: Option<Addr<ConsumerActor>>,
    stats: Option<Addr<StatsActor>>,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                info!("Received ping ");
                ctx.pong(&msg)
            }

            Ok(ws::Message::Text(msg)) => {
                info!("Received text to print : {}", msg);
                if msg.starts_with(r"\print") {
                    ctx.address().do_send(Setup::Consumer)
                } else if msg.starts_with(r"\setup") {
                    match msg.strip_prefix(r"\setup").map(|s| s.trim()) {
                        Some("producer") => ctx.address().do_send(Setup::Producer),
                        Some("consumer") => ctx.address().do_send(Setup::Consumer),
                        _ => ctx.text("Unknown command")
                    };
                } else {
                    error!("Msg not understood {}", msg)
                }
            }
            _ => error!("Received unknown msg {:#?}", item)
        }
    }
}

impl Handler<Setup> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: Setup, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Setup::Producer => {
                if self.producer.is_none() {
                    let a = Arbiter::new();
                    let rpc_conf = self.rpc_config.clone();
                    let kafk_config = self.kafka_config.clone();
                    let self_add = ctx.address();

                    a.spawn(
                        async move {
                            let stats = StatsActor {
                                stats: HashMap::new(),
                                last_updated_at: Instant::now(),
                            }.start();
                            let producer = ProducerActor {
                                parent: self_add,
                                stats_actor: stats,
                                rpc_config: Some(rpc_conf),
                                kafka_config: Some(kafk_config),
                            }.start();
                        }
                    );
                }
            }
            Setup::Consumer => {
                if self.consumer.is_none() {
                    let b = Arbiter::new();
                    let kafk_config = self.kafka_config.clone();
                    let self_add = ctx.address();
                    b.spawn(
                        async move {
                            let consumer = ConsumerActor {
                                parent: self_add,
                                kafka_config: Some(kafk_config.clone()),
                            }.start();
                        }
                    );
                }
            }
        }
    }
}

impl Handler<PrintLine> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: PrintLine, ctx: &mut Self::Context) -> Self::Result {
        let tiles: Vec<Tile> = msg.line.to_uppercase().chars().map(|c| Tile::new(c, msg.count * 10)).collect();
        ctx.text(serde_json::to_string(&tiles).unwrap());

        ctx.run_later(Duration::from_secs(5),
                      move |_, ctx| {
                          ctx.address().do_send(PrintLine { line: msg.line, count: msg.count + 1 })
                      });
    }
}

impl Handler<CharMetrics> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: CharMetrics, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

pub async fn ws_route(req: HttpRequest, rpc_config: web::Data<Arc<RpcConfig>>,
                      kafka_config: web::Data<Arc<KafkaConfig>>, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Websokcet req {:?}", req);

    let new_rpc_conf = RpcConfig { host: rpc_config.host.clone(), port: rpc_config.port };
    let new_kafka_conf = KafkaConfig {
        server: kafka_config.server.clone(),
        source_topic: kafka_config.source_topic.clone(),
        sink_topic: kafka_config.sink_topic.clone(),
        partitions: kafka_config.partitions.clone(),
    };

    let resp = ws::start(
        WebSocket {
            rpc_config: new_rpc_conf,
            kafka_config: new_kafka_conf,
            producer: None,
            consumer: None,
            stats: None,
        }, &req, stream);
    resp
}
use std::sync::Arc;
use std::time::Duration;
use actix::{Actor, StreamHandler, Handler};
use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::{error, info};
use crate::actors::producer::ProducerActor;
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
pub struct Setup;

pub struct WebSocket {
    rpc_config: RpcConfig,
    kafka_config: KafkaConfig,
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
                if msg.starts_with(r"\print") {
                    info!("Received text to print : {}", msg);
                    ctx.address().do_send(PrintLine { line: msg.to_string(), count: 0 })
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
        ProducerActor{
            parent: ctx.address(),
            rpc_config: Some(self.rpc_config.clone()),
            kafka_config: Some(self.kafka_config.clone()),
        }.start();
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


pub async fn ws_route(req: HttpRequest, rpc_config: web::Data<Arc<RpcConfig>>,
                      kafka_config: web::Data<Arc<KafkaConfig>>, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Websokcet req {:?}", req);

    let new_rpc_conf = RpcConfig { host: rpc_config.host.clone(), port: rpc_config.port };
    let new_kafka_conf = KafkaConfig { server: kafka_config.server.clone() };

    let resp = ws::start(
        WebSocket {
            rpc_config: new_rpc_conf,
            kafka_config: new_kafka_conf,
        }, &req, stream);
    resp
}
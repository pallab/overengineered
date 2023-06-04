use std::sync::Arc;
use actix::{Actor, StreamHandler, Handler};
use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::{error, info};
use serde::Serialize;
use crate::actors::consumer::{CharCount, ConsumerActor, RegisterWebsocket};

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct CharMetrics {
    pub counts: Vec<CharCount>,
}

pub struct WebSocket {
    consumer: Arc<Addr<ConsumerActor>>,
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
                    self.consumer.do_send(RegisterWebsocket{ addr : ctx.address()})
                } else {
                    error!("Msg not understood {}", msg)
                }
            }
            _ => error!("Received unknown msg {:#?}", item)
        }
    }
}

impl Handler<CharMetrics> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: CharMetrics, ctx: &mut Self::Context) -> Self::Result {
        info!("received metrics at socket {:#?}", msg);
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

pub async fn ws_route(req: HttpRequest, consumer: web::Data<Arc<Addr<ConsumerActor>>>,
                      stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Websokcet req {:?}", req);
    let addr = Arc::clone(consumer.get_ref());

    let resp = ws::start(
        WebSocket {
            consumer: addr,
        }, &req, stream);
    resp
}
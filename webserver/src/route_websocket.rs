
use std::time::Duration;
use actix::{Actor, StreamHandler, Handler};
use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::info;
use crate::canvas::Tile;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendLetters {
    count: i32,
}

pub struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                info!("received ping ");
                ctx.pong(&msg)
            }

            Ok(ws::Message::Text(msg)) => {
                info!("received text {}", msg);

                ctx.run_later(Duration::from_secs(0),
                              |act, ctx| {
                                  ctx.address().do_send(SendLetters { count: 0 })
                              });
            }
            _ => ()
        }
    }
}

impl Handler<SendLetters> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: SendLetters, ctx: &mut Self::Context) -> Self::Result {
        let tiles: Vec<Tile> = "Hello World!".to_uppercase().chars().map(Tile::new).collect();
        ctx.text(serde_json::to_string(&tiles).unwrap());
        // if msg.count < 100 {
            ctx.run_later(Duration::from_secs(5),
                          move |act, ctx| {
                              ctx.address().do_send(SendLetters { count: msg.count + 1 })
                          });
        // }
    }
}


pub async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Websokcet req {:?}", req);
    let resp = ws::start(WebSocket {}, &req, stream);
    resp
}
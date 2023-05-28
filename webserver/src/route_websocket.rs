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
pub struct PrintLine {
    line: String,
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
                info!("Received ping ");
                ctx.pong(&msg)
            }

            Ok(ws::Message::Text(msg)) => {
                info!("Received text to print : {}", msg);
                ctx.address().do_send(PrintLine { line: msg.to_string(), count: 0 })
            }
            _ => info!("Received unknown msg {:#?}", item)
        }
    }
}

impl Handler<PrintLine> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: PrintLine, ctx: &mut Self::Context) -> Self::Result {

        let tiles: Vec<Tile> = msg.line.to_uppercase().chars().map(|c|Tile::new(c, msg.count * 10)).collect();
        ctx.text(serde_json::to_string(&tiles).unwrap());

        ctx.run_later(Duration::from_secs(5),
                      move |_, ctx| {
                          ctx.address().do_send(PrintLine { line: msg.line, count: msg.count + 1 })
                      });
    }
}


pub async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Websokcet req {:?}", req);
    let resp = ws::start(WebSocket {}, &req, stream);
    resp
}

use std::time::Duration;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use log::info;
use tonic::transport::Channel;
use crate::stocks_rpc::stock_market_client::StockMarketClient;
use actix::prelude::*;
use crate::stocks_rpc::StockPriceRequest;

pub struct LeaderActor {
    pub rpc_client: Option<StockMarketClient<Channel>>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Start;

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Ping;

impl Actor for LeaderActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Leader is started")
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("Leader is stopped")
    }
}

impl Handler<Start> for LeaderActor {
    type Result = ();

    fn handle(&mut self, msg: Start, ctx: &mut Self::Context) -> Self::Result {
        info!("received start");
        ctx.address().do_send(Ping);
    }
}

impl Handler<Ping> for LeaderActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        info!("received ping");
        // we take the client out since we can not move it to the async bloc while its part of self
        // reason is the async future may outlive the actor itself
        let mut client = self.rpc_client.take().unwrap();

        Box::pin(
            async move {
                let request = tonic::Request::new(
                    StockPriceRequest { name: "h".to_string() }
                );

                let response = client.get_stock_price(request).await.expect("");

                let price = response.into_inner().message().await.unwrap_or(None);

                (client, price)
            }
                .into_actor(self)
                .map(
                    |(client, price), _self, _ctx| {
                        _self.rpc_client = Some(client);

                        price
                    })
                .map(
                    |price, _self, _ctx| {

                        if price.is_some() {
                            // add to kafka
                            info!("adding to kafka {:#?}", price.unwrap());
                            // immediately ask for the next msg
                            _ctx.address().do_send(Ping);
                        } else {
                            // wait for a while and then ask
                            info!("No more prices . waiting for 10 seconds");
                            _ctx.notify_later(Ping, Duration::from_secs(10));
                        }
                    })
        )
    }
}
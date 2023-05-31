use std::time::Duration;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use log::{debug, error, info};
use tonic::transport::Channel;
use crate::stocks_rpc::stock_market_client::StockMarketClient;
use actix::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tonic::IntoStreamingRequest;
use crate::kafka::KafkaClient;
use crate::stocks_rpc::StockPriceRequest;

pub struct LeaderActor {
    pub rpc_client: Option<StockMarketClient<Channel>>,
    pub kafka_client: Option<KafkaClient>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Start;

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Ping{counter : i32}

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
        ctx.address().do_send(Ping{counter : 0});
    }
}

impl Handler<Ping> for LeaderActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        info!("received ping");
        // we take the client out since we can not move it to the async bloc while its part of self
        // reason is the async future may outlive the actor itself
        let mut rpc_client = self.rpc_client.take().unwrap();
        let mut kafka_client = self.kafka_client.take().unwrap();

        Box::pin(
            async move {

                let request = tonic::Request::new(
                    StockPriceRequest { name: "d".to_string() }
                );

                let response = rpc_client.get_stock_price(request).await.expect("");
                let mut stream = response.into_inner();

                let mut counts = 0;

                while let Some(p) = &mut stream.message().await.unwrap_or(None) {
                    counts+=1;
                    let resp = kafka_client.send(p.to_owned()).await;

                    if resp.is_err() {
                        error!("Could not send : {:?} - err : {:?}", p, resp.err())
                    } else {
                        debug!("Successful send {counts} {:?}", p)
                    }
                }

                info!("started_with {} / total_saved {}", msg.counter , counts);

                (rpc_client, kafka_client, counts)
            }
                .into_actor(self)
                .map(
                    |(rpc_client, kafka_client, counts), _self, _ctx| {
                        _self.rpc_client = Some(rpc_client);
                        _self.kafka_client = Some(kafka_client);
                        counts
                    })
                .map(
                    |counts, _self, _ctx| {
                        if counts >0 {
                            // immediately ask for the next msg
                            info!("More available . count = {}", counts);
                            _ctx.address().do_send(Ping{counter : msg.counter + counts});
                        } else {
                            // wait for a while and then ask
                            info!("No more prices . waiting for 10 seconds");
                            _ctx.notify_later(msg, Duration::from_secs(10));
                        }
                    })
        )
    }
}
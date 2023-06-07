
use std::thread;
use std::time::Duration;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use log::{debug, error, info};
use actix::prelude::*;
use crate::config::KafkaConfig;
use crate::kafka::{ KafkaProducer};
use crate::words_rpc_impl::WordsRpc;

pub struct ProducerActor;

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Status {
    msg: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct StartPoll;

impl ProducerActor {
    pub fn start() -> bool {
        let a = Arbiter::new();
        let producer = ProducerActor {};
        a.spawn(async move {
            producer.start();
        })
    }
}

impl Actor for ProducerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("ProducerActor is started {:?}", ctx.address());
        ctx.notify(StartPoll);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        info!("ProducerActor is stopping");
        Running::Continue
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("ProducerActor is stopped {:?}", ctx.address())
    }
}

impl Handler<Status> for ProducerActor {
    type Result = ();

    fn handle(&mut self, msg: Status, _ctx: &mut Self::Context) -> Self::Result {
        info!("STATUS : {}", msg.msg);
    }
}

impl Handler<StartPoll> for ProducerActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: StartPoll, ctx: &mut Self::Context) -> Self::Result {
        info!("received ping");
        let self_addr = ctx.address();

        Box::pin(
            async move {

                let rpc = WordsRpc::new_client().await;

                if let Ok(mut rpc) = rpc {

                    self_addr.do_send(Status { msg: "Created a kafka and rpc clients".to_string() });

                    let mut words_stream = WordsRpc::get_words_stream(&mut rpc).await;

                    let mut counts = 0;
                    let producer = KafkaProducer::new();

                    while let Some(p) = &mut words_stream.message().await.unwrap_or(None) {
                        counts += 1;

                        let resp = producer.send_word(
                            KafkaConfig::sink_topic().as_str(),
                            &p.timestamp.to_string(),
                            &p.word,
                            p.timestamp as i64,
                        ).await;

                        if resp.is_err() {
                            error!("Could not send : {:?} - err : {:?}", p, resp.err())
                        } else {
                            debug!("Successful send {counts} {:?}", resp)
                        }
                        if counts % 100 == 0 {
                            info!("Producer words count : {}", counts)
                        }
                    }

                    info!("Total words saved {}", counts);
                } else {
                    error!("Could not connect to rpc endpoint. Retry in 1 sec ..");
                    thread::sleep(Duration::from_secs(1));
                    self_addr.do_send(StartPoll)
                }
            }
                .into_actor(self)
        )
    }
}
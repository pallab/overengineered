use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use log::{debug, error, info};
use actix::prelude::*;
use crate::config::{KafkaConfig, RpcConfig};
use crate::kafka::{ KafkaProducer};
use crate::words_rpc_impl::WordsRpc;

pub struct ProducerActor {
    pub rpc_config: Option<RpcConfig>,
    pub kafka_config: Option<KafkaConfig>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct Status {
    msg: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct StartPoll;

impl ProducerActor {
    pub fn start(rpc_conf: RpcConfig, kafka_config: KafkaConfig) -> bool {
        let a = Arbiter::new();
        let producer = ProducerActor {
            rpc_config: Some(rpc_conf),
            kafka_config: Some(kafka_config),
        };
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
        let topic = "words";
        let kafka_config = self.kafka_config.take().unwrap();
        let rpc_config = self.rpc_config.take().unwrap();
        let self_addr = ctx.address();

        Box::pin(
            async move {

                let mut rpc = WordsRpc::new_client(
                    rpc_config.host.as_str(), rpc_config.port)
                    .await.unwrap();
                self_addr.do_send(Status { msg: "Created a kafka and rpc clients".to_string() });

                let mut words_stream = WordsRpc::get_words_stream(&mut rpc).await;

                let mut counts = 0;
                let producer = KafkaProducer::new(kafka_config.clone());

                while let Some(p) = &mut words_stream.message().await.unwrap_or(None) {
                    counts += 1;

                    let resp = producer.send_word(
                        &topic,
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
            }
                .into_actor(self)
        )
    }
}
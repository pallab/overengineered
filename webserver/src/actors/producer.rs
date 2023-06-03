
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use log::{debug, error, info};
use actix::prelude::*;
use rdkafka::producer::{FutureProducer, FutureRecord};
use crate::actors::stats::{StatsActor, WordsCount};
use crate::config::{KafkaConfig, RpcConfig};
use crate::kafka::{KafkaAdmin, KafkaProducer};
use crate::route_websocket::WebSocket;
use crate::words_rpc_impl::WordsRpc;

pub struct ProducerActor {
    pub parent: Addr<WebSocket>,
    pub stats_actor : Addr<StatsActor>,
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

impl Actor for ProducerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("ProducerActor is started {:?}", ctx.address());
        ctx.address().do_send(StartPoll);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("ProducerActor is stopped {:?}", ctx.address()) }
}

impl Handler<Status> for ProducerActor {
    type Result = ();

    fn handle(&mut self, msg: Status, ctx: &mut Self::Context) -> Self::Result {
        info!("STATUS : {}", msg.msg);
    }
}

impl Handler<StartPoll> for ProducerActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: StartPoll, ctx: &mut Self::Context) -> Self::Result {
        info!("received ping");
        let topic = "words";
        let kafka_config = self.kafka_config.take().unwrap();
        let rpc_config = self.rpc_config.take().unwrap();
        let self_addr = ctx.address();
        let stats_actor = self.stats_actor.clone();

        Box::pin(
            async move {
                // let mut kafka = KafkaClient { config: kafka_config, producer: None };
                let mut rpc = WordsRpc::new_client(
                    rpc_config.host.as_str(), rpc_config.port)
                    .await.unwrap();
                self_addr.do_send(Status { msg: "Created a kafka and rpc clients".to_string() });

                // create the kafka topic
                let admin_client = KafkaAdmin::new(kafka_config.clone());
                let res = admin_client.create_topic(kafka_config.source_topic.as_str()).await;

                match res {
                    Ok(r) => {
                        info!("created a new topic {:#?}",r);
                        self_addr.do_send(Status { msg: "Created a new topic".to_string() })
                    }
                    Err(e) => error!("failed to create the topic {}", e)
                }

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
                        stats_actor.do_send(WordsCount{count : counts})
                    }
                }

                info!("Total words saved {}", counts);
            }
                .into_actor(self)
        )
    }
}
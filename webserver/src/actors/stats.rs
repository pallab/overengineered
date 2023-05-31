use std::collections::HashMap;
use std::time::{ Instant, Duration};
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use log::info;
use actix::prelude::*;
use serde_json::*;

pub struct StatsActor{
    pub(crate) stats : HashMap<i32, Value>,
    pub(crate) last_updated_at : Instant
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct PrintStats;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WordsCount{
    pub(crate) count : i32
}
const KEY_WORD_COUNT: i32 = 1;

impl StatsActor{
    fn add_stat(&mut self, key: i32, value: Value) {
        self.stats.insert(key, value);
        self.last_updated_at = Instant::now();
    }
}

impl Actor for StatsActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Stats actor started");
        ctx.run_interval(Duration::from_secs(10), |act, ctx|{
            ctx.address().do_send(PrintStats)
        });
    }
}

impl Handler<WordsCount> for StatsActor {
    type Result = ();

    fn handle(&mut self, msg: WordsCount, ctx: &mut Self::Context) -> Self::Result {
        self.add_stat(KEY_WORD_COUNT, Value::from(Number::from(msg.count)))
    }
}

impl Handler<PrintStats> for StatsActor {
    type Result = ();

    fn handle(&mut self, msg: PrintStats, ctx: &mut Self::Context) -> Self::Result {
        if Instant::now() - self.last_updated_at < Duration::from_secs(10) {
            info!("Stats : {:?}", self.stats)
        }
    }
}


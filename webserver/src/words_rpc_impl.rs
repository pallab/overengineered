
use log::info;
use tonic::Streaming;
use tonic::transport::{Channel, Error};
use crate::config::RpcConfig;
use crate::words_rpc::{GetWordsRequest, GetWordsResponse};
use crate::words_rpc::words_client::WordsClient;

pub struct WordsRpc;

impl WordsRpc {

    pub async fn new_client() -> Result<WordsClient<Channel>, Error> {
        WordsClient::connect( RpcConfig::address()).await
    }

    pub async fn get_words_stream(client : &mut WordsClient<Channel>) -> Streaming<GetWordsResponse> {
        let request = tonic::Request::new(GetWordsRequest{});

        let response = client.get_words(request)
            .await
            .expect("get_words_stream error ");
        info!("get_words response");

        response.into_inner()
    }
}